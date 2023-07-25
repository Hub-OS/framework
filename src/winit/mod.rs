mod event_translation;
mod key_translation;
mod platform_app;
mod window;
mod window_loop;

pub(crate) use event_translation::*;
use key_translation::*;
pub use platform_app::*;
pub use window::*;
use window_loop::*;

use crate::{cfg_android, cfg_desktop_and_web};

cfg_desktop_and_web! {
    mod desktop_and_web;
    pub(crate) use desktop_and_web::*;
}

cfg_android! {
    mod android;
    pub(crate) use android::*;
}
