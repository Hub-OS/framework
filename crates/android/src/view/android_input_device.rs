use jni::objects::JObject;
use jni::JNIEnv;

/// https://developer.android.com/reference/android/view/InputDevice
///
/// API level 9
pub struct AndroidInputDevice<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidInputDevice<'a> {
    /// https://developer.android.com/reference/android/view/InputDevice#getDevice(int)
    ///
    /// API level 9
    pub fn get_device(jni_env: &mut JNIEnv<'a>, id: jni::sys::jint) -> jni::errors::Result<Self> {
        let owned_obj = jni_env.call_static_method(
            "android/view/InputDevice",
            "getDevice",
            "(I)Landroid/view/InputDevice;",
            &[jni::objects::JValueGen::Int(id)],
        )?;

        Ok(Self {
            j_object: owned_obj.l()?,
        })
    }

    /// https://developer.android.com/reference/android/view/InputDevice#getControllerNumber()
    ///
    /// API level 19
    pub fn get_controller_number(
        &self,
        jni_env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.call_method(&self.j_object, "getControllerNumber", "()I", &[])?;

        owned_obj.i()
    }
}

impl<'a> From<JObject<'a>> for AndroidInputDevice<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
