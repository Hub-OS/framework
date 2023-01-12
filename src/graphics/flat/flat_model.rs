use crate::prelude::*;
use std::sync::Arc;

pub struct FlatModel {
    mesh: Arc<Mesh<FlatVertex>>,
    color: Color,
    origin: Vec2,
    position: Vec2,
    scale: Vec2,
    rotation: f32,
}

impl FlatModel {
    pub fn new(mesh: Arc<Mesh<FlatVertex>>) -> Self {
        Self {
            mesh,
            color: Color::WHITE,
            origin: Vec2::new(0.0, 0.0),
            position: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
        }
    }

    pub fn new_square_mesh() -> Arc<Mesh<FlatVertex>> {
        Mesh::new(
            &[
                FlatVertex {
                    vertex: [-0.5, -0.5],
                },
                FlatVertex {
                    vertex: [-0.5, 0.5],
                },
                FlatVertex { vertex: [0.5, 0.5] },
                FlatVertex {
                    vertex: [0.5, -0.5],
                },
            ],
            &[0, 1, 2, 2, 0, 3],
        )
    }

    pub fn new_square_model() -> Self {
        Self::new(Self::new_square_mesh())
    }

    pub fn new_circle_mesh(vertex_count: usize) -> Arc<Mesh<FlatVertex>> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        vertices.reserve(vertex_count);
        indices.reserve(vertex_count * 3);

        let angle_increment = std::f32::consts::TAU / vertex_count as f32;

        vertices.push(FlatVertex { vertex: [0.0, 0.0] });

        for i in 0..vertex_count {
            let angle = angle_increment * i as f32;

            vertices.push(FlatVertex {
                vertex: [angle.cos() * 0.5, angle.sin() * 0.5],
            });
        }

        for i in 1..vertex_count {
            indices.push(0);
            indices.push(i as u32 + 1);
            indices.push(i as u32);
        }

        if vertex_count > 0 {
            indices.push(0);
            indices.push(1);
            indices.push(vertex_count as u32);
        }

        Mesh::new(&vertices, &indices)
    }

    pub fn new_circle_model(vertex_count: usize) -> Self {
        Self::new(Self::new_circle_mesh(vertex_count))
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    pub fn set_origin(&mut self, origin: Vec2) {
        self.origin = origin
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position
    }

    pub fn scale(&self) -> Vec2 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
}

impl Model<FlatVertex, FlatInstanceData> for FlatModel {
    fn mesh(&self) -> &Arc<Mesh<FlatVertex>> {
        &self.mesh
    }
}

impl Instance<FlatInstanceData> for FlatModel {
    fn instance_data(&self) -> FlatInstanceData {
        let mut transform =
            Mat3::from_scale_angle_translation(self.scale, self.rotation, self.position);
        transform *= Mat3::from_translation(-self.origin);

        FlatInstanceData {
            transform,
            color: self.color,
        }
    }

    fn instance_resources(&self) -> Vec<Arc<dyn AsBinding>> {
        vec![]
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FlatVertex {
    pub vertex: [f32; 2],
}

impl Vertex for FlatVertex {
    fn vertex_layout() -> VertexLayout {
        VertexLayout::new(&[VertexFormat::Float32x2])
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FlatInstanceData {
    transform: Mat3,
    color: Color,
}

impl InstanceData for FlatInstanceData {
    fn instance_layout() -> InstanceLayout {
        InstanceLayout::new(&[
            VertexFormat::Float32x3,
            VertexFormat::Float32x3,
            VertexFormat::Float32x3,
            VertexFormat::Float32x4,
        ])
    }
}
