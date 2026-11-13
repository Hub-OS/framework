use input::RumblePack;
use std::cell::RefCell;
use std::time::Duration;

pub(crate) struct Sdl3RumblePack {
    controller: RefCell<sdl3::gamepad::Gamepad>,
}

impl Sdl3RumblePack {
    pub fn new(controller: sdl3::gamepad::Gamepad) -> Self {
        Self {
            controller: RefCell::new(controller),
        }
    }
}

impl RumblePack for Sdl3RumblePack {
    fn rumble(&self, weak: f32, strong: f32, duration: Duration) {
        let _ = self.controller.borrow_mut().set_rumble(
            (weak.clamp(0.0, 1.0) * u16::MAX as f32) as u16,
            (strong.clamp(0.0, 1.0) * u16::MAX as f32) as u16,
            duration.as_millis() as u32,
        );
    }
}
