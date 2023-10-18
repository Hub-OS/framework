use cfg_macros::*;

pub use log::Level as LogLevel;
pub use log::LevelFilter as LogLevelFilter;
pub use log::{debug, error, info, log, trace, warn};
pub mod crate_name;
pub mod default_logger;

#[derive(Clone)]
pub struct LogRecord {
    pub level: LogLevel,
    pub target: String,
    pub message: String,
}

cfg_web! {
  mod web_logger;
}

cfg_native! {
  mod native_logger;
}
