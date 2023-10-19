use crate::graphics::Color;
use math::*;

pub trait GameWindow {
    fn position(&self) -> IVec2;

    fn set_position(&mut self, position: IVec2);

    fn fullscreen(&self) -> bool;

    fn set_fullscreen(&mut self, fullscreen: bool);

    fn size(&self) -> UVec2;

    fn has_locked_resolution(&self) -> bool;

    fn lock_resolution(&mut self, resolution: UVec2);

    fn unlock_resolution(&mut self);

    fn resolution(&self) -> UVec2;

    fn set_title(&mut self, title: &str);

    fn clear_color(&self) -> Option<Color>;

    fn set_clear_color(&mut self, color: Option<Color>);

    fn vsync_enabled(&self) -> bool;

    fn set_vsync_enabled(&mut self, enabled: bool);

    fn normalize_vec2(&self, mut position: Vec2) -> Vec2 {
        let window_size = self.size().as_vec2();
        let scale = window_size / self.resolution().as_vec2();

        position.x = position.x / window_size.x * 2.0 - 1.0;
        position.y = -(position.y / window_size.y * 2.0 - 1.0);

        if scale.x > scale.y {
            position.x *= scale.x / scale.y;
        } else {
            position.y *= scale.y / scale.x;
        }

        position
    }
}
