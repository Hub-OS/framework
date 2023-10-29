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

    /// Normalizes from a position on the window, with (0.0, 0.0) as the top left and the window size as the bottom right,
    /// to a position on the render, with (-1.0, -1.0) as the top left of the render and (1.0, 1.0) as the bottom right.
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

    fn render_offset(&self) -> Vec2 {
        let window_size = self.size().as_vec2();
        let window_resolution = self.resolution().as_vec2();
        let scale = window_size / window_resolution;
        let mut scaled_resolution = window_resolution;

        if scale.x > scale.y {
            scaled_resolution *= scale.y;
            Vec2::new((window_size.x - scaled_resolution.x) * 0.5, 0.0)
        } else {
            scaled_resolution *= scale.x;
            Vec2::new(0.0, (window_size.y - scaled_resolution.y) * 0.5)
        }
    }

    fn render_scale(&self) -> f32 {
        let window_size = self.size().as_vec2();
        let window_resolution = self.resolution().as_vec2();
        let scale = window_size / window_resolution;

        if scale.x > scale.y {
            scale.y
        } else {
            scale.x
        }
    }
}
