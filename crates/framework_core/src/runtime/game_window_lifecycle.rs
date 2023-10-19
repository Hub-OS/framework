use crate::common::GameWindow;
use crate::graphics::{HasGraphicsContext, RenderTarget};
use math::*;

pub trait GameWindowLifecycle: GameWindow + HasGraphicsContext {
    fn rebuild_surface(&mut self);

    fn acquire_render_target(&mut self) -> Option<RenderTarget>;

    fn present_frame(&mut self, render_target: RenderTarget);

    /// Called by GameIO to update tracked position
    fn moved(&mut self, position: IVec2);

    /// Called by GameIO to update tracked size
    fn resized(&mut self, size: UVec2);

    /// Called by GameIO to notify the window to display a virtual keyboard
    fn set_accepting_text_input(&mut self, accept: bool);
}
