/// https://developer.android.com/reference/android/content/ClipData.Item
///
/// API level 11
jni::bind_java_type! {
    pub AndroidClipDataItem => "android.content.ClipData$Item",
    methods {
        pub fn get_text() -> JCharSequence,
    }
}

/// https://developer.android.com/reference/android/content/ClipData
///
/// API level 11
jni::bind_java_type! {
    pub AndroidClipData => "android.content.ClipData",
    type_map = {
        AndroidClipDataItem => "android.content.ClipData$Item",
    },
    methods {
        /// https://developer.android.com/reference/android/content/ClipData#newPlainText(java.lang.CharSequence,%20java.lang.CharSequence)
        ///
        /// API level 11
        pub static fn new_plain_text(label: JCharSequence, text: JCharSequence) -> AndroidClipData,

        /// https://developer.android.com/reference/android/content/ClipData#getItemAt(int)
        ///
        /// API level 11
        pub fn get_item_at(index: jint) -> AndroidClipDataItem,

        /// https://developer.android.com/reference/android/content/ClipData#getItemCount()
        ///
        /// API level 11
        pub fn get_item_count() -> jint,
    }
}
