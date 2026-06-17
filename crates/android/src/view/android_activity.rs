use super::*;
use crate::activity::AndroidApp;

/// https://developer.android.com/reference/android/app/Activity
///
/// API level 1
jni::bind_java_type! {
    pub AndroidActivity => "android.app.Activity",
    type_map = {
        AndroidWindow => "android.view.Window",
    },
    methods {
        /// https://developer.android.com/reference/android/app/Activity#getWindow()
        ///
        /// API level 1
        pub fn get_window() -> AndroidWindow,

        /// https://developer.android.com/reference/android/app/Activity#getWindow()
        ///
        /// API level 21
        pub fn finish(),
    }
}

impl<'a> AndroidActivity<'a> {
    pub fn from_app(jni_env: &jni::Env<'a>, app: &AndroidApp) -> Self {
        unsafe { Self::from_raw(&jni_env, app.activity_as_ptr().cast()) }
    }
}
