use crate::common::GameIO;
use crate::graphics::*;
use math::Rect;
use std::sync::Arc;

/// RenderQueues only render when consumed by a RenderPass
pub struct SpriteQueue<'a, InstanceData: self::InstanceData = SpriteInstanceData> {
    game_io: &'a GameIO,
    render_queue: RenderQueue<'a, SpriteVertex, InstanceData>,
    mesh: &'a Arc<Mesh<SpriteVertex>>,
}

impl<'a> SpriteQueue<'a, SpriteInstanceData> {
    pub fn new_with_default_pipeline<'b, I>(game_io: &'a GameIO, uniform_resources: I) -> Self
    where
        I: IntoIterator<Item = BindingResource<'b>>,
    {
        let default_sprite_pipeline = game_io.resource::<DefaultSpritePipeline>().unwrap();
        let render_pipeline = default_sprite_pipeline.as_sprite_pipeline();

        Self::new(game_io, render_pipeline, uniform_resources)
    }
}

impl<'a, InstanceData: self::InstanceData> SpriteQueue<'a, InstanceData> {
    pub fn new<'b, I>(
        game_io: &'a GameIO,
        sprite_pipeline: &'a SpritePipeline<InstanceData>,
        uniform_resources: I,
    ) -> Self
    where
        I: IntoIterator<Item = BindingResource<'b>>,
    {
        Self {
            game_io,
            render_queue: RenderQueue::new(game_io, sprite_pipeline, uniform_resources),
            mesh: game_io.resource::<DefaultSpriteMesh>().unwrap().as_mesh(),
        }
    }

    pub fn with_inverted_y(mut self, invert: bool) -> Self {
        self.mesh = if invert {
            self.game_io
                .resource::<DefaultSpriteMeshInverted>()
                .unwrap()
                .as_mesh()
        } else {
            self.game_io
                .resource::<DefaultSpriteMesh>()
                .unwrap()
                .as_mesh()
        };

        self
    }

    pub fn set_uniforms<'b, I>(&mut self, uniform_resources: I)
    where
        I: IntoIterator<Item = BindingResource<'b>>,
    {
        self.render_queue.set_uniforms(uniform_resources);
    }

    pub fn set_scissor(&mut self, rect: Rect) {
        self.render_queue.set_scissor(rect);
    }

    pub fn draw_sprite<Instance: self::Instance<InstanceData>>(&mut self, sprite: &Instance) {
        self.render_queue.draw_instance(self.mesh, sprite);
    }
}

impl<InstanceData: self::InstanceData> RenderQueueTrait for SpriteQueue<'_, InstanceData> {
    fn into_operation_vec(self) -> Vec<RenderOperation> {
        self.render_queue.into_operation_vec()
    }
}
