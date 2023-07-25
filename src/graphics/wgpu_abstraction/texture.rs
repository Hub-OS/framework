use crate::prelude::*;
use std::sync::Arc;

pub struct Texture {
    pub(crate) size: UVec2,
    pub(crate) view: wgpu::TextureView,
}

impl Texture {
    pub fn load_from_memory(game_io: &GameIO, bytes: &[u8]) -> anyhow::Result<Arc<Self>> {
        let image = image::load_from_memory(bytes)?;
        let rgba_image = image.to_rgba8();
        let size = rgba_image.dimensions();
        let (width, height) = size;

        let graphics = game_io.graphics();
        let device = graphics.device();
        let queue = graphics.queue();

        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let image_copy_texture = wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };

        let image_data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        };

        queue.write_texture(image_copy_texture, &rgba_image, image_data_layout, extent);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Arc::new(Self {
            size: size.into(),
            view,
        }))
    }

    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }
}

impl AsBinding for Texture {
    fn as_binding(&self) -> BindingResource {
        wgpu::BindingResource::TextureView(&self.view)
    }
}
