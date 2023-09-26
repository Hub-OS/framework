mod promise;
mod sync_result_async_error;
mod task;

pub(crate) use promise::*;
pub use sync_result_async_error::SyncResultAsyncError;
pub use task::*;

// no wasm support https://github.com/async-rs/async-std/issues/220
crate::cfg_native! {
  mod native;

  pub use native::*;
}

crate::cfg_web! {
  mod web;

  pub use web::*;
}

pub use async_io::block_on;
