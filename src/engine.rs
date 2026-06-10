use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::DerefMut,
    rc::Rc,
};

use gilrs::Gilrs;
use keydraw::{
    Command, Program,
    builders::{FragmentBuilder, PipelineBuilder, VertexBuilder},
    data::CameraUniform,
    state::State,
};
use wgpu::util::DeviceExt;
use window_clipboard::Clipboard;
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    window::Fullscreen,
};
use winit_input_helper::WinitInputHelper;

use crate::{
    AnyAxis, Color, Cursor, Drawer, Game, Input, Pos, Rect, Size, Sprite, Text, TextAlign,
    input::{Controller, ControllerButton},
};

pub struct EngineState<'a> {
    state: &'a mut State,
}

impl<'a> EngineState<'a> {
    pub fn set_window_title(&mut self, title: &str) {
        self.state.window.set_title(title);
    }

    pub fn get_window_title(&mut self) -> String {
        self.state.window.title()
    }

    pub fn set_window_resizable(&mut self, resizable: bool) {
        self.state.window.set_resizable(resizable);
    }

    pub fn fullscreen_window(&mut self) {
        self.state
            .window
            .set_fullscreen(Some(Fullscreen::Borderless(None)));
    }

    pub fn focus_window(&mut self) {
        self.state.window.focus_window();
    }

    pub fn window_has_focus(&mut self) -> bool {
        self.state.window.has_focus()
    }

    pub fn set_cursor_position(&mut self, pos: Pos) {
        self.state
            .window
            .set_cursor_position(LogicalPosition::new(pos.x, pos.y))
            .unwrap();
    }

    pub fn window_size(&self) -> Size {
        let size = self.state.window.inner_size();
        Size::new(size.width as f32, size.height as f32)
    }

    pub fn set_cursor(&self, cursor: Cursor) {
        self.state.window.set_cursor(cursor);
    }

    pub fn set_window_min_size(&self, size: Size) {
        self.state
            .window
            .set_min_inner_size(Some(LogicalSize::new(size.w, size.h)));
    }
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
        state
            .window
            .set_max_inner_size(Some(LogicalSize::new(8000.0, 8000.0)));
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
        let mut took_gilrs = std::mem::take(&mut self.engine.gilrs).unwrap();
        let input = Input::new(
            &took_input,
            &mut took_gilrs,
            std::mem::take(&mut self.engine.prev_buttons),
            std::mem::take(&mut self.engine.prev_axes),
            self.engine.axis_press_threshold,
            self.engine.deadzone,
        );

        self.game
            .update(&mut self.engine, &mut EngineState { state }, &input);

        #[cfg(feature = "ui")]
        {
            let mut messages = Vec::new();
            for container in self.game.containers_mut() {
                messages.append(&mut container.update(
                    &mut self.engine,
                    &mut EngineState { state },
                    &input,
                ));
            }
            self.game
                .messages(&mut self.engine, &mut EngineState { state }, messages);
        }

        self.engine.input = took_input;
        self.engine.gilrs = Some(took_gilrs);
    }

    fn render(&'_ mut self, state: &mut State) -> Vec<Command<'_>> {
        let size = state.window.inner_size();
        self.engine.window_width = size.width as f32;
        self.engine.window_height = size.height as f32;

        let mut drawer = Drawer::new(
            self.engine.camera_material_index,
            self.engine.rect_pipeline_index,
            self.engine.rectext_pipeline_index,
            self.engine.sprite_pipeline_index,
        );

        self.game
            .render(&mut self.engine, &mut EngineState { state }, &mut drawer);

        #[cfg(feature = "ui")]
        for container in self.game.containers() {
            container.render(&mut self.engine, &mut EngineState { state }, &mut drawer);
        }

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
    gilrs: Option<Gilrs>,
    prev_buttons: HashSet<ControllerButton>,
    prev_axes: HashMap<(Controller, AnyAxis), f32>,
    axis_press_threshold: f32,
    deadzone: f32,

    // Misc
    clipboard: Option<Clipboard>,
    window_width: f32,
    window_height: f32,
}

impl Engine {
    pub fn get_font_system(&self) -> Rc<RefCell<glyphon::FontSystem>> {
        self.font_system.as_ref().unwrap().clone()
    }

    /// Set the basic color the window clears to every frame.
    /// No, turning down the alpha will not make the window transparent 😭
    pub fn set_clear_color(&mut self, state: &mut EngineState, color: Color) {
        state.state.clear_color = wgpu::Color {
            r: color.r as f64,
            g: color.g as f64,
            b: color.b as f64,
            a: color.a as f64,
        };
    }

    /// Load an image. You can get the bytes by using the include_bytes! macro.
    pub fn load_sprite(&mut self, state: &mut EngineState, bytes: &'static [u8]) -> Sprite {
        Sprite::new(
            "Unnamed Sprite",
            &mut state.state,
            &self.sprite_bind_group_layout.as_ref().unwrap(),
            bytes,
        )
    }

    /// Load a font, this can be used later via the string. You can get the
    /// bytes by using the include_bytes! macro.
    pub fn load_font(&self, bytes: &'static [u8]) {
        self.font_system
            .as_ref()
            .unwrap()
            .borrow_mut()
            .deref_mut()
            .db_mut()
            .load_font_data(bytes.to_vec());
    }

    /// Create a Text object, do not do this every frame.
    pub fn load_text(
        &self,
        text: &str,
        font_size: f32,
        family: Option<&'static str>,
        align: TextAlign,
        region: Option<Rect>,
    ) -> Text {
        let region = region.map(|region| glyphon::TextBounds {
            left: region.pos.x as i32,
            top: region.pos.y as i32,
            right: (region.pos.x + region.size.w) as i32,
            bottom: (region.pos.y + region.size.h) as i32,
        });
        Text::new(
            self.font_system.as_ref().unwrap().borrow_mut().deref_mut(),
            text,
            font_size,
            font_size + 10.0,
            None,
            family,
            align,
            region,
        )
    }

    /// After changing properties of the Text, use this to apply those changes.
    pub fn reload_text(&self, text: &mut Text) {
        text.reload(self.font_system.as_ref().unwrap().borrow_mut().deref_mut());
    }

    pub fn copy(&mut self, text: String) {
        self.clipboard.as_mut().unwrap().write(text).unwrap();
    }

    pub fn paste(&mut self) -> String {
        self.clipboard.as_ref().unwrap().read().unwrap()
    }

    #[cfg(feature = "ui")]
    pub fn apply_theme(&mut self, state: &mut EngineState, theme: &crate::ui::Theme) {
        self.set_clear_color(state, theme.clear_color);
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
            gilrs: Some(Gilrs::new().unwrap()),
            prev_buttons: HashSet::new(),
            prev_axes: HashMap::new(),
            axis_press_threshold: 0.5,
            deadzone: 0.2,
            clipboard: None,
            window_width: 0.0,
            window_height: 0.0,
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
                    bind_group_layouts: &[Some(&camera_bind_group_layout)],
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
                    bind_group_layouts: &[Some(&camera_bind_group_layout)],
                    immediate_size: 0,
                });
        let rectext_pipeline = PipelineBuilder::new(
            "RectExt Pipeline",
            &rectext_pipeline_layout,
            &VertexBuilder::new(&rectext_shader)
                .with_simple_vertex_buffer()
                .with_buffer(wgpu::VertexBufferLayout {
                    array_stride: size_of::<f32>() as u64 * 17,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32, 6 => Float32x4],
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
                    bind_group_layouts: &[
                        Some(&camera_bind_group_layout),
                        Some(&sprite_bind_group_layout),
                    ],
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

        self.clipboard = Some(unsafe { Clipboard::connect(state.window.as_ref()).unwrap() });
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
