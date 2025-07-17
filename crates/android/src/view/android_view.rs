use super::*;
use jni::objects::JObject;
use jni::JNIEnv;

/// https://developer.android.com/reference/android/view/View
///
/// API level 1
pub struct AndroidView<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidView<'a> {
    /// https://developer.android.com/reference/android/view/View#getRootWindowInsets()
    ///
    /// API level 23
    pub fn get_root_window_insets(
        &self,
        jni_env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<AndroidWindowInsets<'a>> {
        let owned_obj = jni_env.call_method(
            &self.j_object,
            "getRootWindowInsets",
            "()Landroid/view/WindowInsets;",
            &[],
        )?;

        Ok(AndroidWindowInsets::from(JObject::try_from(owned_obj)?))
    }
}

impl<'a> From<JObject<'a>> for AndroidView<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
