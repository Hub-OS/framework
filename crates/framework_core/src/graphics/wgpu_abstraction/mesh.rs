use super::HasGraphicsContext;
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct Mesh<Vertex: super::Vertex> {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    buffers: Arc<(wgpu::Buffer, wgpu::Buffer)>,
}

impl<Vertex: super::Vertex> Mesh<Vertex> {
    pub fn new(
        graphics: &impl HasGraphicsContext,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Arc<Self> {
        let device = graphics.graphics().device();

        let buffers = Arc::new((
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vertex_buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("index_buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        ));

        Arc::new(Self {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            buffers,
        })
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub(super) fn buffers(&self) -> &Arc<(wgpu::Buffer, wgpu::Buffer)> {
        &self.buffers
    }
}
