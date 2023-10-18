use crate::common::{GameIO, NextScene};
use crate::graphics::RenderPass;

pub trait Scene {
    /// Engine accessor to the variable storing NextScene
    fn next_scene(&mut self) -> &mut NextScene;

    /// Called when this scene is processed as the NextScene
    fn enter(&mut self, _game_io: &mut GameIO) {}

    /// Called when the scene is no longer active, signaling updates will stop
    fn exit(&mut self, _game_io: &mut GameIO) {}

    /// Called before the scene is dropped
    fn destroy(&mut self, _game_io: &mut GameIO) {}

    /// Called every tick even when the scene is inactive
    fn continuous_update(&mut self, _game_io: &mut GameIO) {}

    /// Called every tick while the scene is active, put game logic here
    fn update(&mut self, game_io: &mut GameIO);

    /// Called to perform rendering. Not guaranteed to be called after every update.
    /// Can be called multiple times in a single tick from transitions.
    fn draw(&mut self, game_io: &mut GameIO, render_pass: &mut RenderPass);
}
