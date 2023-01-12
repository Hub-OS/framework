use crate::prelude::*;
use std::sync::Arc;

/// RenderQueues only render when consumed by a RenderPass
pub struct SpriteQueue<'a, InstanceData: self::InstanceData> {
    sprite_pipeline: &'a SpritePipeline<InstanceData>,
    render_queue: RenderQueue<'a, SpriteVertex, InstanceData>,
    mesh: &'a Arc<Mesh<SpriteVertex>>,
}

impl<'a, InstanceData: self::InstanceData> SpriteQueue<'a, InstanceData> {
    pub fn new<'b, Globals, I>(
        game_io: &'a GameIO<Globals>,
        sprite_pipeline: &'a SpritePipeline<InstanceData>,
        uniform_resources: I,
    ) -> Self
    where
        I: IntoIterator<Item = BindingResource<'b>>,
    {
        let mesh = sprite_pipeline.mesh();

        Self {
            sprite_pipeline,
            render_queue: RenderQueue::new(game_io, sprite_pipeline, uniform_resources),
            mesh,
        }
    }

    pub fn with_inverted_y(mut self, invert: bool) -> Self {
        self.mesh = if invert {
            self.sprite_pipeline.inverted_mesh()
        } else {
            self.sprite_pipeline.mesh()
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

impl<'a, InstanceData: self::InstanceData> RenderQueueTrait for SpriteQueue<'a, InstanceData> {
    fn into_operation_vec(self) -> Vec<RenderOperation> {
        self.render_queue.into_operation_vec()
    }
}
