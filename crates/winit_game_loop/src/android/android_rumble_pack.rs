use std::time::Duration;

#[derive(Debug)]
pub(crate) struct AndroidRumblePack {}

impl AndroidRumblePack {
    pub(super) fn new() -> Self {
        Self {}
    }

    pub fn rumble(&self, _weak: f32, _strong: f32, _duration: Duration) {
        // todo: rumble support on android
    }
}
