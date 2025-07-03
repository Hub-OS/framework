use super::*;
use crate::WinitPlatformApp as PlatformApp;
use jni::objects::{JClass, JObject};
use jni::JNIEnv;

pub struct AndroidWindowInsets<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidWindowInsets<'a> {
    // API 30
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

pub struct AndroidWindowInsetsType {}

impl AndroidWindowInsetsType {
    pub fn ime(jni_env: &mut JNIEnv) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj =
            jni_env.call_static_method("android/view/WindowInsets$Type", "ime", "()I", &[])?;
        owned_obj.i()
    }
}

// API 29
pub struct AndroidInsets<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidInsets<'a> {
    pub fn top(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.get_field(&self.j_object, "top", "I")?;
        owned_obj.i()
    }

    pub fn bottom(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.get_field(&self.j_object, "bottom", "I")?;
        owned_obj.i()
    }

    pub fn left(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.get_field(&self.j_object, "left", "I")?;
        owned_obj.i()
    }

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
