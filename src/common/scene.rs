use crate::prelude::*;

pub trait Scene {
    /// Engine accessor to the variable storing NextScene
    fn next_scene(&mut self) -> &mut NextScene;

    /// Called when this scene is processed as the NextScene
    fn enter(&mut self, _game_io: &mut GameIO) {}

    /// Called every tick, put game logic here
    fn update(&mut self, game_io: &mut GameIO);

    /// Called to perform rendering. Not guaranteed to be called after every update
    fn draw(&mut self, game_io: &mut GameIO, render_pass: &mut RenderPass);
}
