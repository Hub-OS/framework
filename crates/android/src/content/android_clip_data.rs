use jni::objects::{JObject, JValue};
use jni::JNIEnv;

use crate::java::lang::JavaCharSequence;

/// https://developer.android.com/reference/android/content/ClipData
///
/// API level 11
pub struct AndroidClipData<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidClipData<'a> {
    /// https://developer.android.com/reference/android/content/ClipData#newPlainText(java.lang.CharSequence,%20java.lang.CharSequence)
    ///
    /// API level 11
    pub fn new_plain_text(
        jni_env: &mut JNIEnv<'a>,
        label: &JavaCharSequence,
        text: &JavaCharSequence,
    ) -> jni::errors::Result<Self> {
        let owned_obj = jni_env.call_static_method(
            "android/content/ClipData",
            "newPlainText",
            "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/content/ClipData;",
            &[label.into(), text.into()],
        )?;

        Ok(Self::from(JObject::try_from(owned_obj)?))
    }

    pub fn get_item_at(
        &self,
        jni_env: &mut JNIEnv<'a>,
        i: jni::sys::jint,
    ) -> jni::errors::Result<AndroidClipDataItem<'a>> {
        let owned_obj = jni_env.call_method(
            &self.j_object,
            "getItemAt",
            "(I)Landroid/content/ClipData$Item;",
            &[i.into()],
        )?;

        Ok(AndroidClipDataItem::from(JObject::try_from(owned_obj)?))
    }

    pub fn get_item_count(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<jni::sys::jint> {
        let owned_obj = jni_env.call_method(&self.j_object, "getItemCount", "()I", &[])?;

        owned_obj.i()
    }
}

impl<'object_ref, 'a> From<&'object_ref AndroidClipData<'a>> for JValue<'object_ref, 'a> {
    fn from(other: &'object_ref AndroidClipData<'a>) -> Self {
        JValue::Object(&other.j_object)
    }
}

impl<'a> From<JObject<'a>> for AndroidClipData<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}

/// https://developer.android.com/reference/android/content/ClipData.Item
///
/// API level 11
pub struct AndroidClipDataItem<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidClipDataItem<'a> {
    pub fn get_text(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<JavaCharSequence<'a>> {
        let owned_obj =
            jni_env.call_method(&self.j_object, "getText", "()Ljava/lang/CharSequence;", &[])?;

        Ok(JavaCharSequence::from(owned_obj.l()?))
    }
}

impl<'a> From<JObject<'a>> for AndroidClipDataItem<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
