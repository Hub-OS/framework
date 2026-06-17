use crate::content::AndroidClipData;

/// https://developer.android.com/reference/android/content/ClipboardManager
///
/// API level 11
jni::bind_java_type! {
    pub AndroidClipboardManager => "android.content.ClipboardManager",
    type_map = {
        AndroidClipData => "android.content.ClipData",
    },
    methods {
        /// https://developer.android.com/reference/android/content/ClipboardManager#getPrimaryClip()
        ///
        /// API level 11
        ///
        /// May return null
        pub fn get_primary_clip() -> AndroidClipData,

        /// https://developer.android.com/reference/android/content/ClipboardManager#setPrimaryClip(android.content.ClipData)
        ///
        /// API level 11
        pub fn set_primary_clip(clip_data: AndroidClipData),
    }
}
