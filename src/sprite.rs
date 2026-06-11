use image::GenericImageView;
use keydraw::state::State;

use crate::{Scale, Size};

/// A sprite which can be drawn.
/// This struct has a very low amount of actual data in it, just a reference to
/// the actual material of the sprite by ID.
#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub(crate) material: u32,
    pub size: Size,
}

impl Sprite {
    fn common_new(
        state: &mut State,
        name: &str,
        layout: &wgpu::BindGroupLayout,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = state.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        state.queue.write_texture(
            wgpu::TexelCopyTextureInfoBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = state.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Sprite material"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let material = state.get_material();
        state.material_db.insert(material, bind_group);

        Self {
            material,
            size: Size::new(width as f32, height as f32),
        }
    }

    pub(crate) fn new_svg(
        name: &str,
        state: &mut State,
        layout: &wgpu::BindGroupLayout,
        data: &'static [u8],
        scale: Scale,
    ) -> Self {
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_data(data, &opt).unwrap();

        let width = (tree.size().width().ceil() * scale.x) as u32;
        let height = (tree.size().height().ceil() * scale.y) as u32;

        let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
        resvg::render(
            &tree,
            tiny_skia::Transform::from_scale(scale.x, scale.y),
            &mut pixmap.as_mut(),
        );

        Self::common_new(state, name, layout, width, height, pixmap.data())
    }

    pub(crate) fn new(
        name: &str,
        state: &mut State,
        layout: &wgpu::BindGroupLayout,
        data: &'static [u8],
    ) -> Self {
        let diffuse_image = image::load_from_memory(data).expect("Invalid data read for an image!");
        let rgba = diffuse_image.to_rgba8();
        let dimensions = diffuse_image.dimensions();

        Self::common_new(state, name, layout, dimensions.0, dimensions.1, &rgba)
    }
}
