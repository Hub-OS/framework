use super::*;
use jni::objects::JObject;
use jni::JNIEnv;
use std::convert::From;

pub struct AndroidActivityWindow<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidActivityWindow<'a> {
    // API 30
    pub fn get_insets_controller(
        &self,
        jni_env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<AndroidInsetsController<'a>> {
        let owned_obj = jni_env.call_method(
            &self.j_object,
            "getInsetsController",
            "()Landroid/view/WindowInsetsController;",
            &[],
        )?;

        Ok(AndroidInsetsController::from(JObject::try_from(owned_obj)?))
    }

    // API 1
    pub fn get_decor_view(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<AndroidView<'a>> {
        let owned_obj =
            jni_env.call_method(&self.j_object, "getDecorView", "()Landroid/view/View;", &[])?;

        Ok(AndroidView::from(JObject::try_from(owned_obj)?))
    }
}

impl<'a> From<JObject<'a>> for AndroidActivityWindow<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
