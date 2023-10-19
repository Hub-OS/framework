use crate::graphics::*;
use wgpu::util::DeviceExt;

pub struct BufferResource {
    graphics: GraphicsContext,
    buffer: wgpu::Buffer,
}

impl BufferResource {
    pub fn new(graphics: &impl HasGraphicsContext, data: &[u8]) -> Self {
        let graphics = graphics.graphics();
        let device = graphics.device();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            graphics: graphics.clone(),
            buffer,
        }
    }

    pub fn write(&self, offset: u64, data: &[u8]) {
        let queue = self.graphics.queue();
        queue.write_buffer(&self.buffer, offset, data);
    }
}

impl AsBinding for BufferResource {
    fn as_binding(&self) -> BindingResource<'_> {
        self.buffer.as_entire_binding()
    }
}
