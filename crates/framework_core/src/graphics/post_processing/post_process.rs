use crate::common::GameIO;
use crate::graphics::*;

pub trait PostProcess {
    fn render_pipeline(&self) -> &PostPipeline;
    fn uniform_resources(&self) -> Vec<BindingResource<'_>>;

    fn update(&mut self, _game_io: &GameIO) {}

    fn draw(
        &mut self,
        game_io: &GameIO,
        mut render_pass: RenderPass,
        texture_source: &TextureSourceModel,
    ) {
        let mut queue = RenderQueue::new(game_io, self.render_pipeline(), self.uniform_resources());

        queue.draw_model(texture_source);
        render_pass.consume_queue(queue);
        render_pass.flush();
    }
}
