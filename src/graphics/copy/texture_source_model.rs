use crate::prelude::*;
use std::sync::Arc;

pub struct TextureSourceModel {
    mesh: Arc<Mesh<Vec2>>,
    texture: Arc<Texture>,
    sampler: Arc<TextureSampler>,
}

impl TextureSourceModel {
    pub fn new(game_io: &GameIO, texture: Arc<Texture>) -> Self {
        Self {
            mesh: Mesh::new(
                &[
                    Vec2::new(-1.0, -1.0),
                    Vec2::new(-1.0, 1.0),
                    Vec2::new(1.0, 1.0),
                    Vec2::new(1.0, -1.0),
                ],
                &[0, 1, 2, 2, 0, 3],
            ),
            texture,
            sampler: game_io
                .resource::<DefaultSpriteSampler>()
                .unwrap()
                .as_texture_sampler()
                .clone(),
        }
    }

    pub fn texture(&self) -> &Arc<Texture> {
        &self.texture
    }

    pub fn set_texture(&mut self, texture: Arc<Texture>) {
        self.texture = texture;
    }
}

impl Instance<()> for TextureSourceModel {
    fn instance_data(&self) {}

    fn instance_resources(&self) -> Vec<Arc<dyn AsBinding>> {
        vec![self.texture.clone(), self.sampler.clone()]
    }
}

impl Model<Vec2, ()> for TextureSourceModel {
    fn mesh(&self) -> &Arc<Mesh<Vec2>> {
        &self.mesh
    }
}
