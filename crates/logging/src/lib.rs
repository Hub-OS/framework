use cfg_macros::*;

pub use log;
pub mod crate_name;
pub mod default_logger;

#[derive(Clone)]
pub struct LogRecord {
    pub level: log::Level,
    pub target: String,
    pub message: String,
}

cfg_web! {
  mod web_logger;
}

cfg_native! {
  mod native_logger;
}
