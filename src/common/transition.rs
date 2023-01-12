use crate::prelude::*;

pub trait Transition {
    fn draw(
        &mut self,
        game_io: &mut GameIO,
        render_pass: &mut RenderPass,
        previous_scene: &mut Box<dyn Scene>,
        next_scene: &mut Box<dyn Scene>,
    );

    fn is_complete(&self) -> bool;
}
