use crate::common::GameIO;
use crate::graphics::*;

pub struct StructResource<T> {
    data: T,
    buffer_resource: BufferResource,
}

impl<T: bytemuck::Pod> StructResource<T> {
    pub fn new(game_io: &GameIO, data: T) -> Self {
        Self {
            data,
            buffer_resource: BufferResource::new(game_io, bytemuck::bytes_of(&data)),
        }
    }

    pub fn new_with_layout(game_io: &GameIO, data: T, layout: &[VertexFormat]) -> Self {
        let mut slice = bytemuck::bytes_of(&data);
        let mut bytes = Vec::with_capacity(slice.len());

        for vertex_format in layout {
            let count = vertex_format.size() as usize;
            bytes.extend_from_slice(&slice[..count]);

            bytes.extend(std::iter::repeat(0).take(count.next_power_of_two() - count));

            slice = &slice[count..];
        }

        Self {
            data,
            buffer_resource: BufferResource::new(game_io, &bytes),
        }
    }

    pub fn value(&self) -> &T {
        &self.data
    }

    pub fn binding_type() -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    }
}

impl<T> AsBinding for StructResource<T> {
    fn as_binding(&self) -> BindingResource<'_> {
        self.buffer_resource.as_binding()
    }
}
