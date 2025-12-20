use cfg_macros::cfg_web;

pub use cfg_macros;
pub use framework_core::async_task;
pub use framework_core::common;
pub use framework_core::graphics;
pub use framework_core::graphics::wgpu;
pub use framework_core::runtime;
pub use input;
pub use logging;
pub use math;
pub mod prelude;

cfg_web! {
  pub use wasm_forward::wasm_bindgen;
  pub use wasm_forward::wasm_bindgen::prelude::wasm_bindgen;
}

#[cfg(feature = "sdl2")]
pub use sdl2_game_loop;

#[cfg(feature = "winit")]
pub use winit_game_loop;

#[cfg(target_os = "android")]
#[cfg(feature = "android")]
pub use android_game_loop;
