use crate::prelude::*;
use std::sync::Arc;

pub struct FlatModel {
    mesh: Arc<Mesh<Vec2>>,
    color: Color,
    origin: Vec2,
    position: Vec2,
    scale: Vec2,
    rotation: f32,
}

impl FlatModel {
    pub fn new(mesh: Arc<Mesh<Vec2>>) -> Self {
        Self {
            mesh,
            color: Color::WHITE,
            origin: Vec2::new(0.0, 0.0),
            position: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
        }
    }

    pub fn new_square_mesh() -> Arc<Mesh<Vec2>> {
        Mesh::new(
            &[
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, 0.5),
                Vec2::new(0.5, 0.5),
                Vec2::new(0.5, -0.5),
            ],
            &[0, 1, 2, 2, 0, 3],
        )
    }

    pub fn new_square_model() -> Self {
        Self::new(Self::new_square_mesh())
    }

    pub fn new_circle_mesh(vertex_count: usize) -> Arc<Mesh<Vec2>> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        vertices.reserve(vertex_count);
        indices.reserve(vertex_count * 3);

        let angle_increment = std::f32::consts::TAU / vertex_count as f32;

        vertices.push(Vec2::ZERO);

        for i in 0..vertex_count {
            let angle = angle_increment * i as f32;

            vertices.push(Vec2::from_angle(angle) * 0.5);
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

impl Model<Vec2, FlatInstanceData> for FlatModel {
    fn mesh(&self) -> &Arc<Mesh<Vec2>> {
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

    fn bind_group_layout() -> &'static [BindGroupLayoutEntry] {
        &[]
    }

    fn instance_resources(&self) -> Vec<Arc<dyn AsBinding>> {
        vec![]
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
