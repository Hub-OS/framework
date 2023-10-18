use math::*;
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};

pub struct RawWindowAndDisplayHandle {
    pub window_handle: RawWindowHandle,
    pub display_handle: RawDisplayHandle,
}

unsafe impl HasRawDisplayHandle for RawWindowAndDisplayHandle {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.display_handle
    }
}

unsafe impl HasRawWindowHandle for RawWindowAndDisplayHandle {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }
}

pub trait GameWindow: HasRawDisplayHandle + HasRawWindowHandle {
    fn raw_window_and_display_handle(&self) -> RawWindowAndDisplayHandle {
        RawWindowAndDisplayHandle {
            window_handle: self.raw_window_handle(),
            display_handle: self.raw_display_handle(),
        }
    }

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

    /// Called by GameIO to update tracked position
    fn set_moved(&mut self, position: IVec2);

    /// Called by GameIO to update tracked size
    fn set_resized(&mut self, size: UVec2);

    /// Called by GameIO to notify the window to display a virtual keyboard
    fn set_accepting_text_input(&mut self, accept: bool);
}
