use jni::objects::JObject;
use jni::JNIEnv;

/// https://developer.android.com/reference/android/graphics/Insets
///
/// API level 29
pub struct AndroidInsets<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidInsets<'a> {
    /// https://developer.android.com/reference/android/graphics/Insets#top
    ///
    /// API level 29
    pub fn top(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.get_field(&self.j_object, "top", "I")?;
        owned_obj.i()
    }

    /// https://developer.android.com/reference/android/graphics/Insets#bottom
    ///
    /// API level 29
    pub fn bottom(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.get_field(&self.j_object, "bottom", "I")?;
        owned_obj.i()
    }

    /// https://developer.android.com/reference/android/graphics/Insets#left
    ///
    /// API level 29
    pub fn left(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.get_field(&self.j_object, "left", "I")?;
        owned_obj.i()
    }

    /// https://developer.android.com/reference/android/graphics/Insets#right
    ///
    /// API level 29
    pub fn right(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.get_field(&self.j_object, "right", "I")?;
        owned_obj.i()
    }
}

impl<'a> From<JObject<'a>> for AndroidInsets<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
