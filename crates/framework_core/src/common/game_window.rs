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

    fn integer_scaling(&self) -> bool;

    fn set_integer_scaling(&mut self, value: bool);

    fn set_title(&mut self, title: &str);

    fn clear_color(&self) -> Option<Color>;

    fn set_clear_color(&mut self, color: Option<Color>);

    fn vsync_enabled(&self) -> bool;

    fn set_vsync_enabled(&mut self, enabled: bool);

    /// Normalizes from a position on the window, with (0.0, 0.0) as the top left and the window size as the bottom right,
    /// to a position on the render, with (-1.0, -1.0) as the top left of the render and (1.0, 1.0) as the bottom right.
    fn normalize_vec2(&self, mut position: Vec2) -> Vec2 {
        let window_size = self.size().as_vec2();

        position.x = position.x / window_size.x * 2.0 - 1.0;
        position.y = -(position.y / window_size.y * 2.0 - 1.0);

        position *= window_size / (self.resolution().as_vec2() * self.render_scale());

        position
    }

    fn render_offset(&self) -> Vec2 {
        if !self.has_locked_resolution() {
            return Vec2::ZERO;
        }

        let window_size = self.size().as_vec2();
        let window_resolution = self.resolution().as_vec2();
        let scaled_resolution = window_resolution * self.render_scale();

        let mut offset = Vec2::new(
            (window_size.x - scaled_resolution.x) * 0.5,
            (window_size.y - scaled_resolution.y) * 0.5,
        );

        if self.integer_scaling() {
            offset = offset.floor();
        }

        offset
    }

    fn render_scale(&self) -> f32 {
        if !self.has_locked_resolution() {
            return 1.0;
        }

        if self.integer_scaling() {
            let quotient = self.size() / self.resolution();
            quotient.min_element().max(1) as f32
        } else {
            let window_size = self.size().as_vec2();
            let window_resolution = self.resolution().as_vec2();
            let scale = window_size / window_resolution;

            scale.min_element()
        }
    }
}
