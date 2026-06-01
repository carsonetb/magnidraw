use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use keydraw::{
    Command, Program,
    builders::{FragmentBuilder, PipelineBuilder, VertexBuilder},
    data::CameraUniform,
    state::State,
};
use wgpu::util::DeviceExt;
use winit_input_helper::WinitInputHelper;

use crate::{Color, Drawer, Game, Input, Sprite, Text};

pub struct EngineState<'a> {
    state: &'a mut State,
}

pub(crate) struct EngineHolder {
    game: Box<dyn Game>,
    engine: Engine,
}

impl EngineHolder {
    pub(crate) fn new(game: Box<dyn Game>) -> Self {
        Self {
            game,
            engine: Engine::new(),
        }
    }
}

impl Program for EngineHolder {
    fn init(&mut self, state: &mut State) {
        self.engine.init(state);
        self.game
            .setup(&mut self.engine, &mut EngineState { state });
    }

    fn event(&mut self, event: &winit::event::WindowEvent, state: &mut State) {
        self.engine.event(event, state);
    }

    fn device_event(&mut self, device_event: &winit::event::DeviceEvent, state: &mut State) {
        self.engine.device_event(device_event, state);
    }

    fn new_events(&mut self, _state: &mut State) {
        self.engine.input.step();
    }

    fn about_to_wait(&mut self, state: &mut State) {
        self.engine.input.end_step();

        let took_input = std::mem::take(&mut self.engine.input);
        let input = Input::new(&took_input);
        self.game
            .update(&mut self.engine, &mut EngineState { state }, &input);
        self.engine.input = took_input;
    }

    fn render(&'_ mut self, state: &mut State) -> Vec<Command<'_>> {
        let mut drawer = Drawer::new(
            self.engine.camera_material_index,
            self.engine.rect_pipeline_index,
            self.engine.rectext_pipeline_index,
            self.engine.sprite_pipeline_index,
        );

        self.game
            .render(&mut self.engine, &mut EngineState { state }, &mut drawer);

        drawer.collect(
            self.engine.font_system.as_ref().unwrap().clone(),
            self.engine.swash_cache.as_ref().unwrap().clone(),
            self.engine.text_atlas.as_ref().unwrap().clone(),
            self.engine.text_renderer.as_ref().unwrap().clone(),
            self.engine.viewport.as_ref().unwrap(),
        )
    }
}

pub struct Engine {
    // Pipelines and materials
    rect_pipeline_index: u32,
    rectext_pipeline_index: u32,
    sprite_pipeline_index: u32,
    sprite_bind_group_layout: Option<wgpu::BindGroupLayout>,
    camera_material_index: u32,
    camera_buffer: Option<wgpu::Buffer>,

    // Text stuff
    font_system: Option<Rc<RefCell<glyphon::FontSystem>>>,
    swash_cache: Option<Rc<RefCell<glyphon::SwashCache>>>,
    text_atlas: Option<Rc<RefCell<glyphon::TextAtlas>>>,
    text_renderer: Option<Rc<RefCell<glyphon::TextRenderer>>>,
    viewport: Option<glyphon::Viewport>,

    // Input
    input: WinitInputHelper,
}

impl Engine {
    pub fn set_clear_color(&mut self, state: &mut EngineState, color: Color) {
        state.state.clear_color = wgpu::Color {
            r: color.r as f64,
            g: color.g as f64,
            b: color.b as f64,
            a: color.a as f64,
        };
    }

    pub fn load_sprite(&mut self, state: &mut EngineState, bytes: &'static [u8]) -> Sprite {
        Sprite::new(
            "Unnamed Sprite",
            &mut state.state,
            &self.sprite_bind_group_layout.as_ref().unwrap(),
            bytes,
        )
    }

    pub fn load_font(&self, bytes: &'static [u8]) {
        self.font_system
            .as_ref()
            .unwrap()
            .borrow_mut()
            .deref_mut()
            .db_mut()
            .load_font_data(bytes.to_vec());
    }

    pub fn load_text(&self, text: &str, font_size: f32, family: Option<&'static str>) -> Text {
        Text::new(
            self.font_system.as_ref().unwrap().borrow_mut().deref_mut(),
            text,
            font_size,
            font_size + 10.0,
            None,
            family,
        )
    }

    pub fn reload_text(&self, text: &mut Text) {
        text.reload(self.font_system.as_ref().unwrap().borrow_mut().deref_mut());
    }

    fn new() -> Self {
        Self {
            camera_buffer: None,
            rect_pipeline_index: u32::MAX,
            rectext_pipeline_index: u32::MAX,
            sprite_pipeline_index: u32::MAX,
            sprite_bind_group_layout: None,
            camera_material_index: u32::MAX,
            font_system: None,
            swash_cache: None,
            text_atlas: None,
            text_renderer: None,
            viewport: None,
            input: WinitInputHelper::new(),
        }
    }

    fn init(&mut self, state: &mut State) {
        // --- Setup camera and uniform material ---

        let camera_uniform = CameraUniform::new(800.0, 600.0);
        let camera_buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let camera_bind_group_layout =
            state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("camera_bind_group_layout"),
                });
        let material = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
        self.camera_material_index = state.get_material();
        state
            .material_db
            .insert(self.camera_material_index, material);

        self.camera_buffer = Some(camera_buffer);

        // --- Setup rect shader pipelines ---

        let rect_shader = state
            .device
            .create_shader_module(wgpu::include_wgsl!("rect.wgsl"));
        let rect_pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Rect Render Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    immediate_size: 0,
                });
        let rect_pipeline = PipelineBuilder::new(
            "Rect Pipeline",
            &rect_pipeline_layout,
            &VertexBuilder::new(&rect_shader)
                .with_simple_vertex_buffer()
                .with_buffer(wgpu::VertexBufferLayout {
                    array_stride: size_of::<f32>() as u64 * 8,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4],
                }),
            &FragmentBuilder::new(&rect_shader).with_alpha_target(state),
        )
        .resolve(state);
        self.rect_pipeline_index = state.get_pipeline();
        state
            .pipeline_db
            .insert(self.rect_pipeline_index, rect_pipeline);

        let rectext_shader = state
            .device
            .create_shader_module(wgpu::include_wgsl!("rect_complex.wgsl"));
        let rectext_pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("RectExt Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    immediate_size: 0,
                });
        let rectext_pipeline = PipelineBuilder::new(
            "RectExt Pipeline",
            &rectext_pipeline_layout,
            &VertexBuilder::new(&rectext_shader)
                .with_simple_vertex_buffer()
                .with_buffer(wgpu::VertexBufferLayout {
                    array_stride: size_of::<f32>() as u64 * 12,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4],
                }),
            &FragmentBuilder::new(&rectext_shader).with_alpha_target(state),
        )
        .resolve(state);
        self.rectext_pipeline_index = state.get_pipeline();
        state
            .pipeline_db
            .insert(self.rectext_pipeline_index, rectext_pipeline);

        // --- Setup sprite shader pipeline ---

        let sprite_bind_group_layout =
            state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("sprite_bind_group_layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                            count: None,
                        },
                    ],
                });
        let sprite_shader = state
            .device
            .create_shader_module(wgpu::include_wgsl!("sprite.wgsl"));
        let sprite_pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Sprite Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout, &sprite_bind_group_layout],
                    immediate_size: 0,
                });
        let sprite_pipeline = PipelineBuilder::new(
            "Sprite Pipeline",
            &sprite_pipeline_layout,
            &VertexBuilder::new(&sprite_shader)
                .with_simple_vertex_buffer()
                .with_buffer(wgpu::VertexBufferLayout {
                    array_stride: size_of::<f32>() as u64 * 12,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4]
                }),
            &FragmentBuilder::new(&sprite_shader).with_alpha_target(state),
        ).resolve(state);
        self.sprite_pipeline_index = state.get_pipeline();
        state
            .pipeline_db
            .insert(self.sprite_pipeline_index, sprite_pipeline);
        self.sprite_bind_group_layout = Some(sprite_bind_group_layout);

        // --- Setup text rendering system ---

        let font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();
        let cache = glyphon::Cache::new(&state.device);
        let mut viewport = glyphon::Viewport::new(&state.device, &cache);
        viewport.update(
            &state.queue,
            glyphon::Resolution {
                width: state.config.width,
                height: state.config.height,
            },
        );
        let mut atlas = glyphon::TextAtlas::new(
            &state.device,
            &state.queue,
            &cache,
            state.config.format.clone(),
        );
        let text_renderer = glyphon::TextRenderer::new(
            &mut atlas,
            &state.device,
            wgpu::MultisampleState::default(),
            None,
        );

        self.font_system = Some(Rc::new(RefCell::new(font_system)));
        self.swash_cache = Some(Rc::new(RefCell::new(swash_cache)));
        self.text_atlas = Some(Rc::new(RefCell::new(atlas)));
        self.text_renderer = Some(Rc::new(RefCell::new(text_renderer)));
        self.viewport = Some(viewport);

        state.enable_vsync();
    }

    fn event(&mut self, event: &winit::event::WindowEvent, state: &mut State) {
        self.input.process_window_event(event);

        match event {
            winit::event::WindowEvent::Resized(winit::dpi::PhysicalSize { width, height }) => {
                let camera = CameraUniform::new(*width as f32, *height as f32);
                state.queue.write_buffer(
                    &self.camera_buffer.as_ref().unwrap(),
                    0,
                    &bytemuck::cast_slice(&[camera]),
                );

                self.viewport.as_mut().unwrap().update(
                    &state.queue,
                    glyphon::Resolution {
                        width: *width,
                        height: *height,
                    },
                );
            }
            _ => (),
        }
    }

    fn device_event(&mut self, device_event: &winit::event::DeviceEvent, _state: &mut State) {
        self.input.process_device_event(device_event);
    }
}
