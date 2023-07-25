mod event_translation;
mod key_translation;
mod window;
mod window_loop;

pub(crate) use event_translation::*;
use key_translation::*;
pub use window::*;
pub use window_loop::*;

use crate::{cfg_android, cfg_desktop_and_web};

cfg_desktop_and_web! {
    mod desktop_and_web;
    pub(crate) use desktop_and_web::*;
}

cfg_android! {
    mod android;
    pub(crate) use android::*;
}
