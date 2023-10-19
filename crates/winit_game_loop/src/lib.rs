mod event_translation;
mod key_translation;
mod loop_states;
mod winit_game_loop;
mod winit_game_window;
mod winit_platform_app;

use cfg_macros::*;
use key_translation::*;
use winit_game_window::*;

pub use winit_game_loop::*;
pub use winit_platform_app::*;

use crate::{cfg_android, cfg_desktop_and_web};

cfg_desktop_and_web! {
    mod desktop_and_web;
    pub(crate) use desktop_and_web::*;
}

cfg_android! {
    mod android;
    pub use android::*;
}
