use super::*;
use crate::activity::AndroidApp;
use jni::objects::JObject;
use jni::JNIEnv;

/// https://developer.android.com/reference/android/app/Activity
///
/// API level 1
pub struct AndroidActivity<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidActivity<'a> {
    /// https://developer.android.com/reference/android/app/Activity#getWindow()
    ///
    /// API level 1
    pub fn get_window(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<AndroidWindow<'a>> {
        let owned_obj =
            jni_env.call_method(&self.j_object, "getWindow", "()Landroid/view/Window;", &[])?;

        Ok(AndroidWindow::from(JObject::try_from(owned_obj)?))
    }

    /// https://developer.android.com/reference/android/app/Activity#getWindow()
    ///
    /// API level 21
    pub fn finish(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<()> {
        jni_env.call_method(&self.j_object, "finish", "()V", &[])?;
        Ok(())
    }
}

impl<'a> From<&AndroidApp> for AndroidActivity<'a> {
    fn from(app: &AndroidApp) -> Self {
        let j_object = unsafe { JObject::from_raw(std::mem::transmute(app.activity_as_ptr())) };

        Self { j_object }
    }
}
