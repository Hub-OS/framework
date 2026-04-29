use jni::objects::JObject;
use jni::JNIEnv;

use crate::content::AndroidClipData;

/// https://developer.android.com/reference/android/content/ClipboardManager
///
/// API level 11
pub struct AndroidClipboardManager<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidClipboardManager<'a> {
    /// https://developer.android.com/reference/android/content/ClipboardManager#getPrimaryClip()
    ///
    /// API level 11
    pub fn get_primary_clip(
        &self,
        jni_env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<Option<AndroidClipData<'a>>> {
        let owned_obj = jni_env.call_method(
            &self.j_object,
            "getPrimaryClip",
            "()Landroid/content/ClipData;",
            &[],
        )?;

        let Ok(o) = owned_obj.l() else {
            return Ok(None);
        };

        Ok(Some(AndroidClipData::from(o)))
    }

    /// https://developer.android.com/reference/android/content/ClipboardManager#setPrimaryClip(android.content.ClipData)
    ///
    /// API level 11
    pub fn set_primary_clip(
        &self,
        jni_env: &mut JNIEnv<'a>,
        clip_data: &AndroidClipData,
    ) -> jni::errors::Result<()> {
        jni_env.call_method(
            &self.j_object,
            "setPrimaryClip",
            "(Landroid/content/ClipData;)V",
            &[clip_data.into()],
        )?;

        Ok(())
    }
}

impl<'a> From<JObject<'a>> for AndroidClipboardManager<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
