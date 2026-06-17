use super::*;

/// https://developer.android.com/reference/android/view/View
///
/// API level 1
jni::bind_java_type! {
    pub AndroidView => "android.view.View",
    type_map = {
        AndroidWindowInsets => "android.view.WindowInsets",
    },
    methods {
        /// https://developer.android.com/reference/android/view/View#getRootWindowInsets()
        ///
        /// API level 23
        pub fn get_root_window_insets() -> AndroidWindowInsets,
    }
}
