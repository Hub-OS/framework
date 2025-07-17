use jni::objects::{JObject, JValueGen};
use jni::JNIEnv;

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

impl<O> From<AndroidWindowInsetsBehavior> for JValueGen<O> {
    fn from(other: AndroidWindowInsetsBehavior) -> Self {
        (other as i32).into()
    }
}

/// https://developer.android.com/reference/android/view/WindowInsetsController
///
///  API level 30
pub struct AndroidWindowInsetsController<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidWindowInsetsController<'a> {
    /// https://developer.android.com/reference/android/view/WindowInsetsController#setSystemBarsBehavior(int)
    ///
    /// API level 30
    pub fn set_system_bars_behavior(
        &self,
        jni_env: &mut JNIEnv<'a>,
        inset_behavior: AndroidWindowInsetsBehavior,
    ) -> jni::errors::Result<()> {
        jni_env.call_method(
            &self.j_object,
            "setSystemBarsBehavior",
            "(I)V",
            &[inset_behavior.into()],
        )?;
        Ok(())
    }

    /// https://developer.android.com/reference/android/view/WindowInsetsController#show(int)
    ///
    /// API level 30
    pub fn show(&self, jni_env: &mut JNIEnv<'a>, flags: i32) -> jni::errors::Result<()> {
        jni_env.call_method(&self.j_object, "show", "(I)V", &[flags.into()])?;
        Ok(())
    }

    /// https://developer.android.com/reference/android/view/WindowInsetsController#hide(int)
    ///
    /// API level 30
    pub fn hide(&self, jni_env: &mut JNIEnv<'a>, flags: i32) -> jni::errors::Result<()> {
        jni_env.call_method(&self.j_object, "hide", "(I)V", &[flags.into()])?;
        Ok(())
    }
}

impl<'a> From<JObject<'a>> for AndroidWindowInsetsController<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
