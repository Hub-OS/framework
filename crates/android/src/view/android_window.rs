use super::*;
use jni::objects::JObject;
use jni::JNIEnv;
use std::convert::From;

/// https://developer.android.com/reference/android/view/Window
///
/// API level 1
pub struct AndroidWindow<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidWindow<'a> {
    /// https://developer.android.com/reference/android/view/Window?hl=en#getInsetsController()
    ///
    /// API level 30
    pub fn get_insets_controller(
        &self,
        jni_env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<AndroidWindowInsetsController<'a>> {
        let owned_obj = jni_env.call_method(
            &self.j_object,
            "getInsetsController",
            "()Landroid/view/WindowInsetsController;",
            &[],
        )?;

        Ok(AndroidWindowInsetsController::from(JObject::try_from(
            owned_obj,
        )?))
    }

    /// https://developer.android.com/reference/android/view/Window?hl=en#getDecorView()
    ///
    /// API level 1
    pub fn get_decor_view(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<AndroidView<'a>> {
        let owned_obj =
            jni_env.call_method(&self.j_object, "getDecorView", "()Landroid/view/View;", &[])?;

        Ok(AndroidView::from(JObject::try_from(owned_obj)?))
    }
}

impl<'a> From<JObject<'a>> for AndroidWindow<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
