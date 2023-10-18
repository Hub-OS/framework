use crate::common::GameIO;
use crate::graphics::RenderPass;

pub trait SceneTransition {
    fn draw(
        &mut self,
        game_io: &mut GameIO,
        render_pass: &mut RenderPass,
        draw_previous_scene: &mut dyn FnMut(&mut GameIO, &mut RenderPass),
        draw_next_scene: &mut dyn FnMut(&mut GameIO, &mut RenderPass),
    );

    fn is_complete(&self) -> bool;
}
