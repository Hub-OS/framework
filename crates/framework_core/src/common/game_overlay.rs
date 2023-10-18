use crate::common::GameIO;
use crate::graphics::RenderPass;

pub enum GameOverlayTarget {
    Render,
    Window,
}

pub trait GameOverlay {
    /// Called every tick after service pre_updates and before scene updates
    fn pre_update(&mut self, _game_io: &mut GameIO) {}

    /// Called every tick after scene updates and before service post_updates
    fn post_update(&mut self, _game_io: &mut GameIO) {}

    /// Called to perform rendering. Not guaranteed to be called after every update
    fn draw(&mut self, game_io: &mut GameIO, render_pass: &mut RenderPass);
}
