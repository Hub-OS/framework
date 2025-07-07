use crate::graphics::*;

pub struct StructResource<T> {
    data: T,
    buffer_resource: BufferResource,
}

impl<T: bytemuck::Pod> StructResource<T> {
    pub fn new(graphics: &impl HasGraphicsContext, data: T) -> Self {
        Self {
            data,
            buffer_resource: BufferResource::new(graphics, bytemuck::bytes_of(&data)),
        }
    }

    pub fn new_with_layout(
        graphics: &impl HasGraphicsContext,
        data: T,
        layout: &[VertexFormat],
    ) -> Self {
        let mut slice = bytemuck::bytes_of(&data);
        let mut bytes = Vec::with_capacity(slice.len());

        for vertex_format in layout {
            let count = vertex_format.size() as usize;
            bytes.extend_from_slice(&slice[..count]);

            bytes.extend(std::iter::repeat_n(0, count.next_power_of_two() - count));

            slice = &slice[count..];
        }

        Self {
            data,
            buffer_resource: BufferResource::new(graphics, &bytes),
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
