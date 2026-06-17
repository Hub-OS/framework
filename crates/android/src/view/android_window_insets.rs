use crate::graphics::*;

/// https://developer.android.com/reference/android/view/WindowInsets
///
/// API level 20
jni::bind_java_type! {
    pub AndroidWindowInsets => "android.view.WindowInsets",
    type_map = {
        AndroidInsets => "android.graphics.Insets"
    },
    methods {
        /// https://developer.android.com/reference/android/view/WindowInsets#getInsets(int)
        ///
        /// API level 30
        pub fn get_insets(type_mask: jint) -> AndroidInsets,
    }
}

/// https://developer.android.com/reference/android/view/WindowInsets.Type
///
/// API level 30
jni::bind_java_type! {
    pub AndroidWindowInsetsType => "android.view.WindowInsets$Type",
    methods {
        /// https://developer.android.com/reference/android/view/WindowInsets.Type#ime()
        ///
        /// API level 30
        pub static fn ime() -> jint,

        /// https://developer.android.com/reference/android/view/WindowInsets.Type#navigationBars()
        ///
        /// API level 30
        pub static fn navigation_bars() -> jint,

        /// https://developer.android.com/reference/android/view/WindowInsets.Type#displayCutout()
        ///
        /// API level 30
        pub static fn display_cutout() -> jint,
    }
}
