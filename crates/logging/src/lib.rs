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

/// A cross platform panic hook.
///
/// Panics won't log on Web and Android unless a panic hook is set.
pub fn panic_hook() -> Box<dyn Fn(&std::panic::PanicHookInfo<'_>) + 'static + Sync + Send> {
    cfg_web!({ Box::new(console_error_panic_hook::hook) });

    cfg_android!({
        Box::new(|panic_info| {
            use backtrace::Backtrace;
            let backtrace = Backtrace::new();

            use ndk_sys::android_LogPriority as AndroidLogPriority;
            use std::ffi::{c_int, CString};

            let tag = CString::new("panic").unwrap_or_default();
            let msg = CString::new(format!("{panic_info}\nBacktrace: \n{backtrace:?}"))
                .unwrap_or_default();

            unsafe {
                ndk_sys::__android_log_write(
                    AndroidLogPriority::ANDROID_LOG_ERROR.0 as c_int,
                    tag.as_c_str().as_ptr(),
                    msg.as_c_str().as_ptr(),
                );
            }
        })
    });

    cfg_desktop!({ std::panic::take_hook() })
}
