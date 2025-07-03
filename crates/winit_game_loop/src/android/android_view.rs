use super::*;
use crate::WinitPlatformApp as PlatformApp;
use jni::objects::JObject;
use jni::JNIEnv;

pub struct AndroidView<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidView<'a> {
    // API 23
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
