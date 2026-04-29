use crate::java::lang::JavaString;
use android_activity::AndroidApp;
use jni::objects::JObject;
use jni::JNIEnv;

pub struct AndroidContext<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidContext<'a> {
    /// https://developer.android.com/reference/android/content/Context#CLIPBOARD_SERVICE
    ///
    /// API level 1
    pub fn clipboard_service_name(jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<JavaString<'a>> {
        JavaString::from_str(jni_env, "clipboard")
    }

    pub fn clipboard_service(
        &self,
        jni_env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<jni::objects::JObject<'a>> {
        let clipboard_service_name = AndroidContext::clipboard_service_name(jni_env)?;
        self.get_system_service(jni_env, &clipboard_service_name)
    }

    /// https://developer.android.com/reference/android/content/Context#getSystemService(java.lang.String)
    ///
    /// API level 1
    pub fn get_system_service(
        &self,
        jni_env: &mut JNIEnv<'a>,
        name: &JavaString<'a>,
    ) -> jni::errors::Result<jni::objects::JObject<'a>> {
        let owned_obj = jni_env.call_method(
            &self.j_object,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[name.into()],
        )?;

        owned_obj.l()
    }
}

impl<'a> From<&AndroidApp> for AndroidContext<'a> {
    fn from(app: &AndroidApp) -> Self {
        let j_object = unsafe { JObject::from_raw(std::mem::transmute(app.activity_as_ptr())) };

        Self { j_object }
    }
}
