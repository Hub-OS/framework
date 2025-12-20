use input::RumblePack;
use std::time::Duration;

pub(crate) struct AndroidRumblePack;

impl RumblePack for AndroidRumblePack {
    fn rumble(&self, _weak: f32, _strong: f32, _duration: Duration) {
        // todo
    }
}
