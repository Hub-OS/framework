use super::*;
use crate::WinitPlatformApp as PlatformApp;
use jni::objects::JObject;
use jni::JNIEnv;

pub struct AndroidActivity<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidActivity<'a> {
    pub fn get_window(
        &self,
        jni_env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<AndroidActivityWindow<'a>> {
        let owned_obj =
            jni_env.call_method(&self.j_object, "getWindow", "()Landroid/view/Window;", &[])?;

        Ok(AndroidActivityWindow::from(JObject::try_from(owned_obj)?))
    }
}

impl<'a> From<&PlatformApp> for AndroidActivity<'a> {
    fn from(app: &PlatformApp) -> Self {
        let j_object = unsafe { JObject::from_raw(std::mem::transmute(app.activity_as_ptr())) };

        Self { j_object }
    }
}
