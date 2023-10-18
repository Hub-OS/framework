use input::RumblePack;
use std::cell::RefCell;
use std::time::Duration;

pub(crate) struct Sdl2RumblePack {
    controller: RefCell<sdl2::controller::GameController>,
}

impl Sdl2RumblePack {
    pub fn new(controller: sdl2::controller::GameController) -> Self {
        Self {
            controller: RefCell::new(controller),
        }
    }
}

impl RumblePack for Sdl2RumblePack {
    fn rumble(&self, weak: f32, strong: f32, duration: Duration) {
        let _ = self.controller.borrow_mut().set_rumble(
            (weak.clamp(0.0, 1.0) * u16::MAX as f32) as u16,
            (strong.clamp(0.0, 1.0) * u16::MAX as f32) as u16,
            duration.as_millis() as u32,
        );
    }
}
