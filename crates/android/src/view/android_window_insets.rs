use crate::graphics::*;
use jni::objects::JObject;
use jni::JNIEnv;

/// https://developer.android.com/reference/android/view/WindowInsets
///
/// API level 20
pub struct AndroidWindowInsets<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidWindowInsets<'a> {
    /// https://developer.android.com/reference/android/view/WindowInsets#getInsets(int)
    ///
    /// API level 30
    pub fn get_insets(
        &self,
        jni_env: &mut JNIEnv<'a>,
        type_mask: jni::sys::jint,
    ) -> jni::errors::Result<AndroidInsets<'a>> {
        let owned_obj = jni_env.call_method(
            &self.j_object,
            "getInsets",
            "(I)Landroid/graphics/Insets;",
            &[jni::objects::JValueGen::Int(type_mask)],
        )?;

        Ok(AndroidInsets::from(JObject::try_from(owned_obj)?))
    }
}

impl<'a> From<JObject<'a>> for AndroidWindowInsets<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}

/// https://developer.android.com/reference/android/view/WindowInsets.Type
///
/// API level 30
pub struct AndroidWindowInsetsType {}

impl AndroidWindowInsetsType {
    fn named_type(jni_env: &mut JNIEnv, name: &str) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj =
            jni_env.call_static_method("android/view/WindowInsets$Type", name, "()I", &[])?;
        owned_obj.i()
    }

    /// https://developer.android.com/reference/android/view/WindowInsets.Type#ime()
    ///
    /// API level 30
    pub fn ime(jni_env: &mut JNIEnv) -> jni::errors::Result<jni::sys::jint> {
        Self::named_type(jni_env, "ime")
    }

    /// https://developer.android.com/reference/android/view/WindowInsets.Type#navigationBars()
    ///
    /// API level 30
    pub fn navigation_bars(jni_env: &mut JNIEnv) -> jni::errors::Result<jni::sys::jint> {
        Self::named_type(jni_env, "navigationBars")
    }

    /// https://developer.android.com/reference/android/view/WindowInsets.Type#displayCutout()
    ///
    /// API level 30
    pub fn display_cutout(jni_env: &mut JNIEnv) -> jni::errors::Result<jni::sys::jint> {
        Self::named_type(jni_env, "displayCutout")
    }
}
