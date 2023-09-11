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
            texture: Arc::new(Texture {
                texture: None,
                view,
                size,
            }),
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
            texture: Some(texture),
            view: texture_view,
            size,
        })
    }

    pub fn read_rgba_bytes(&self, game_io: &GameIO) -> impl std::future::Future<Output = Vec<u8>> {
        let graphics = game_io.graphics();
        let device = graphics.device();
        let queue = graphics.queue();

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // create buffer
        let width = self.texture.width();
        let height = self.texture.height();

        let final_bytes_per_row = 4 * width;
        let alignment_remainder = final_bytes_per_row % wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let bytes_per_row = if alignment_remainder > 0 {
            final_bytes_per_row + (wgpu::COPY_BYTES_PER_ROW_ALIGNMENT - alignment_remainder)
        } else {
            final_bytes_per_row
        };
        let output_buffer_size = (bytes_per_row * height) as wgpu::BufferAddress;

        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let output_buffer = device.create_buffer(&output_buffer_desc);

        // copy render target data to buffer
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                // expecting texture to be None only from internal API usage
                // this function should never be called by internal API
                texture: self.texture.texture.as_ref().unwrap(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width: self.texture.width(),
                height: self.texture.height(),
                depth_or_array_layers: 1,
            },
        );

        // submit to gpu
        queue.submit(Some(encoder.finish()));

        // maping buffer + polling

        let buffer_slice = output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise the application will freeze.
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);

        // convert rx to future
        let rx_future = std::future::poll_fn(move |_| {
            use core::task::Poll;

            match rx.try_recv() {
                Ok(value) => Poll::Ready(value),
                Err(_) => Poll::Pending,
            }
        });

        async move {
            rx_future.await.unwrap();

            let data = {
                let buffer_slice = output_buffer.slice(..);
                let buffer_view = buffer_slice.get_mapped_range();

                // move bytes to vec and remove padding
                let bytes_per_row = bytes_per_row as usize;
                let final_bytes_per_row = final_bytes_per_row as usize;
                let height = height as usize;
                let mut data = Vec::with_capacity(final_bytes_per_row * height);

                data.extend(
                    buffer_view
                        .chunks(bytes_per_row)
                        .flat_map(|chunk| chunk.iter().take(final_bytes_per_row)),
                );

                data
            };

            output_buffer.unmap();
            data
        }
    }
}
