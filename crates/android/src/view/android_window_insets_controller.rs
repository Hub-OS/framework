use jni::JValue;

pub enum AndroidWindowInsetsBehavior {
    /// https://developer.android.com/reference/android/view/WindowInsetsController#BEHAVIOR_DEFAULT
    ///
    /// API level 30 as BEHAVIOR_SHOW_BARS_BY_SWIPE, 31 as BEHAVIOR_DEFAULT
    BehaviorDefault = 1,
    /// https://developer.android.com/reference/android/view/WindowInsetsController#BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    ///
    /// API level 30
    BehaviorShowTransientBarsBySwipe = 2,
}

/// https://developer.android.com/reference/android/view/WindowInsetsController
///
///  API level 30
jni::bind_java_type! {
    pub AndroidWindowInsetsController => "android.view.WindowInsetsController",
    methods {
        /// https://developer.android.com/reference/android/view/WindowInsetsController#show(int)
        ///
        /// API level 30
        pub fn show(flags: i32),

        /// https://developer.android.com/reference/android/view/WindowInsetsController#hide(int)
        ///
        /// API level 30
        pub fn hide(flags: i32),
    }
}

impl<'a> AndroidWindowInsetsController<'a> {
    /// https://developer.android.com/reference/android/view/WindowInsetsController#setSystemBarsBehavior(int)
    ///
    /// API level 30
    pub fn set_system_bars_behavior(
        &self,
        jni_env: &mut jni::Env<'a>,
        inset_behavior: AndroidWindowInsetsBehavior,
    ) -> jni::errors::Result<()> {
        jni_env.call_method(
            &self,
            jni::jni_str!("setSystemBarsBehavior"),
            jni::jni_sig!("(I)V"),
            &[JValue::Int(inset_behavior as _)],
        )?;
        Ok(())
    }
}
