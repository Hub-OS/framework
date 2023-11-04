use crate::graphics::*;
use math::*;
use std::sync::Arc;

pub struct RenderTarget {
    clear_color: Option<Color>,
    texture: Arc<Texture>,
    usage: wgpu::TextureUsages,
    format: wgpu::TextureFormat,
}

impl RenderTarget {
    // todo: Swap when const_trait_impl is stable
    /// wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC  | wgpu::TextureUsages::RENDER_ATTACHMENT
    pub const DEFAULT_USAGE: wgpu::TextureUsages =
        wgpu::TextureUsages::from_bits_retain((1 << 0) | (1 << 2) | (1 << 4));

    pub fn new(graphics: &impl HasGraphicsContext, size: UVec2) -> Self {
        Self::new_with_usage(graphics, size, Self::DEFAULT_USAGE)
    }

    pub fn new_with_usage(
        graphics: &impl HasGraphicsContext,
        size: UVec2,
        usage: wgpu::TextureUsages,
    ) -> Self {
        let graphics = graphics.graphics();
        let format = graphics.default_texture_format();

        Self {
            texture: RenderTarget::create_texture(graphics, size, usage, format),
            clear_color: Some(Color::TRANSPARENT),
            usage,
            format,
        }
    }

    pub fn new_with_format(
        graphics: &impl HasGraphicsContext,
        size: UVec2,
        format: wgpu::TextureFormat,
    ) -> Self {
        let usage = Self::DEFAULT_USAGE;

        Self {
            texture: RenderTarget::create_texture(graphics, size, usage, format),
            clear_color: Some(Color::TRANSPARENT),
            usage,
            format,
        }
    }

    pub fn from_view(view: wgpu::TextureView, size: UVec2) -> Self {
        Self {
            texture: Arc::new(Texture {
                texture: None,
                view,
                size,
            }),
            clear_color: Some(Color::TRANSPARENT),
            usage: Self::DEFAULT_USAGE,
            format: Texture::DEFAULT_FORMAT,
        }
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }

    pub fn size(&self) -> UVec2 {
        self.texture.size
    }

    pub fn resize(&mut self, graphics: &impl HasGraphicsContext, size: UVec2) {
        if self.texture.size == size {
            return;
        }

        self.texture = RenderTarget::create_texture(graphics, size, self.usage, self.format);
    }

    pub fn clear_color(&self) -> Option<Color> {
        self.clear_color
    }

    /// If the render target acts as a depth buffer, only the color.a value will be used
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
                store: wgpu::StoreOp::Store,
            },
        }
    }

    pub(crate) fn depth_attachment(&self) -> wgpu::RenderPassDepthStencilAttachment {
        wgpu::RenderPassDepthStencilAttachment {
            view: &self.texture.view,
            depth_ops: Some(wgpu::Operations {
                load: match self.clear_color {
                    Some(color) => wgpu::LoadOp::Clear(color.a),
                    None => wgpu::LoadOp::Load,
                },
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }
    }

    fn create_texture(
        graphics: &impl HasGraphicsContext,
        size: UVec2,
        usage: wgpu::TextureUsages,
        format: wgpu::TextureFormat,
    ) -> Arc<Texture> {
        let graphics = graphics.graphics();
        let device = graphics.device();

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
            texture: Some(texture),
            view: texture_view,
            size,
        })
    }
}
