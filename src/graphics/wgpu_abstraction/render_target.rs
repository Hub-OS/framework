use crate::prelude::*;
use std::sync::Arc;

pub struct RenderTarget {
    clear_color: Option<Color>,
    texture: Arc<Texture>,
    usage: TextureUsages,
}

impl RenderTarget {
    // todo: Swap when const_trait_impl is stable
    /// wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC  | wgpu::TextureUsages::RENDER_ATTACHMENT
    pub const DEFAULT_USAGE: TextureUsages =
        TextureUsages::from_bits_retain((1 << 0) | (1 << 2) | (1 << 4));

    pub fn new(game_io: &GameIO, size: UVec2) -> Self {
        Self {
            texture: RenderTarget::create_texture(game_io, size, Self::DEFAULT_USAGE),
            clear_color: Some(Color::TRANSPARENT),
            usage: Self::DEFAULT_USAGE,
        }
    }

    pub fn new_with_usage(game_io: &GameIO, size: UVec2, usage: TextureUsages) -> Self {
        Self {
            texture: RenderTarget::create_texture(game_io, size, usage),
            clear_color: Some(Color::TRANSPARENT),
            usage,
        }
    }

    pub(crate) fn from_view(view: wgpu::TextureView, size: UVec2) -> Self {
        Self {
            texture: Arc::new(Texture { view, size }),
            clear_color: Some(Color::TRANSPARENT),
            usage: Self::DEFAULT_USAGE,
        }
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }

    pub fn size(&self) -> UVec2 {
        self.texture.size
    }

    pub fn resize(&mut self, game_io: &GameIO, size: UVec2) {
        if self.texture.size == size {
            return;
        }

        self.texture = RenderTarget::create_texture(game_io, size, self.usage);
    }

    pub fn clear_color(&self) -> Option<Color> {
        self.clear_color
    }

    pub fn set_clear_color(&mut self, color: Option<Color>) {
        self.clear_color = color;
    }

    pub(crate) fn color_attachment(&self) -> wgpu::RenderPassColorAttachment {
        wgpu::RenderPassColorAttachment {
            view: &self.texture.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: match self.clear_color {
                    Some(color) => wgpu::LoadOp::Clear(color.into()),
                    None => wgpu::LoadOp::Load,
                },
                store: true,
            },
        }
    }

    pub(crate) fn depth_attachment(&self) -> Option<wgpu::RenderPassDepthStencilAttachment> {
        None
    }

    fn create_texture(game_io: &GameIO, size: UVec2, usage: TextureUsages) -> Arc<Texture> {
        let graphics = game_io.graphics();
        let device = graphics.device();
        let format = graphics.surface_config().format;

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            label: None,
            view_formats: &[],
        };

        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        Arc::new(Texture {
            view: texture_view,
            size,
        })
    }
}
