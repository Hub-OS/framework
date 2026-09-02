use crate::net::AndroidUri;

/// https://developer.android.com/reference/android/content/Intent
///
/// API level 1
jni::bind_java_type! {
    pub AndroidIntent => "android.content.Intent",
    type_map = {
        AndroidUri => "android.net.Uri",
    },
    constructors {
        fn new(),
        fn new_with_action(action: JString),
    },
    methods {
        /// https://developer.android.com/reference/android/content/Intent#setData(android.net.Uri)
        ///
        /// API level 1
        pub fn set_data(uri: AndroidUri) -> AndroidIntent,
    }
}
