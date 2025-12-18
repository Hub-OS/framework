use crate::{async_task::promise_future, graphics::*};
use math::*;
use std::sync::Arc;

pub struct Texture {
    pub(crate) view: wgpu::TextureView,
}

impl Texture {
    pub const DEFAULT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

    pub fn load_from_memory(
        graphics: &impl HasGraphicsContext,
        bytes: &[u8],
    ) -> anyhow::Result<Arc<Self>> {
        Self::load_from_memory_with_format(graphics, bytes, Self::DEFAULT_FORMAT)
    }

    pub fn load_from_memory_with_format(
        graphics: &impl HasGraphicsContext,
        bytes: &[u8],
        format: wgpu::TextureFormat,
    ) -> anyhow::Result<Arc<Self>> {
        let image = image::load_from_memory(bytes)?;
        let rgba_image = image.to_rgba8();
        let size = rgba_image.dimensions();
        let (width, height) = size;

        let graphics = graphics.graphics();
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
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let image_copy_texture = wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };

        let image_data_layout = wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * width),
            rows_per_image: Some(height),
        };

        queue.write_texture(image_copy_texture, &rgba_image, image_data_layout, extent);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Arc::new(Self { view }))
    }

    pub fn width(&self) -> u32 {
        self.size().x
    }

    pub fn height(&self) -> u32 {
        self.size().y
    }

    pub fn size(&self) -> UVec2 {
        let size_3d = self.view.texture().size();
        UVec2::new(size_3d.width, size_3d.height)
    }

    pub fn read_rgba_bytes(
        &self,
        graphics: &impl HasGraphicsContext,
    ) -> impl std::future::Future<Output = Vec<u8>> {
        let graphics = graphics.graphics();
        let device = graphics.device();
        let queue = graphics.queue();

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // create buffer
        let width = self.width();
        let height = self.height();

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
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                // expecting texture to be None only from internal API usage
                // this function should never be called by internal API
                texture: self.view.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        // submit to gpu
        queue.submit([encoder.finish()]);

        // maping buffer + polling
        let (resolve_future, promised_future) = promise_future();
        let buffer_slice = output_buffer.slice(..);

        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            resolve_future(result);
        });

        async move {
            promised_future.await.unwrap();

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

impl AsBinding for Texture {
    fn as_binding(&self) -> BindingResource<'_> {
        wgpu::BindingResource::TextureView(&self.view)
    }
}
