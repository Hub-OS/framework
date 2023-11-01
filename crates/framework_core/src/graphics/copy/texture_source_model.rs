use crate::common::GameIO;
use crate::graphics::*;
use math::*;
use std::sync::Arc;

pub struct DefaultTextureSourceMesh {
    mesh: Arc<Mesh<Vec2>>,
}

impl DefaultTextureSourceMesh {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        Self {
            mesh: Mesh::new(
                game_io,
                &[
                    Vec2::new(-1.0, -1.0),
                    Vec2::new(-1.0, 1.0),
                    Vec2::new(1.0, 1.0),
                    Vec2::new(1.0, -1.0),
                ],
                &[0, 1, 2, 2, 0, 3],
            ),
        }
    }

    pub fn as_mesh(&self) -> &Arc<Mesh<Vec2>> {
        &self.mesh
    }
}

pub struct TextureSourceModel {
    mesh: Arc<Mesh<Vec2>>,
    texture: Arc<Texture>,
    sampler: Arc<TextureSampler>,
}

impl TextureSourceModel {
    pub fn new(game_io: &GameIO, texture: Arc<Texture>) -> Self {
        let sampler = game_io
            .resource::<DefaultSpriteSampler>()
            .unwrap()
            .as_texture_sampler()
            .clone();

        Self::new_with_sampler(game_io, texture, sampler)
    }

    pub fn new_with_sampler(
        game_io: &GameIO,
        texture: Arc<Texture>,
        sampler: Arc<TextureSampler>,
    ) -> Self {
        Self {
            mesh: game_io
                .resource::<DefaultTextureSourceMesh>()
                .unwrap()
                .mesh
                .clone(),
            texture,
            sampler,
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
