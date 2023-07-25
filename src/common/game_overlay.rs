use crate::prelude::*;

pub enum GameOverlayTarget {
    Render,
    Window,
}

pub trait GameOverlay {
    /// Called every tick, put game logic here
    fn update(&mut self, game_io: &mut GameIO);

    /// Called to perform rendering. Not guaranteed to be called after every update
    fn draw(&mut self, game_io: &mut GameIO, render_pass: &mut RenderPass);
}
