use super::*;

/// https://developer.android.com/reference/android/view/Window
///
/// API level 1
jni::bind_java_type! {
    pub AndroidWindow => "android.view.Window",
    type_map = {
        AndroidWindowInsetsController => "android.view.WindowInsetsController",
        AndroidView => "android.view.View"
    },
    methods {
        /// https://developer.android.com/reference/android/view/Window?hl=en#getInsetsController()
        ///
        /// API level 30
        pub fn get_insets_controller() -> AndroidWindowInsetsController,

        /// https://developer.android.com/reference/android/view/Window?hl=en#getDecorView()
        ///
        /// API level 1
        pub fn get_decor_view() -> AndroidView,
    }
}
