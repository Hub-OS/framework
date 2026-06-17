/// https://developer.android.com/reference/android/graphics/Insets
///
/// API level 29
jni::bind_java_type! {
    pub AndroidInsets => "android.graphics.Insets",
    fields {
        top: jint,
        bottom: jint,
        left: jint,
        right: jint,
    }
}
