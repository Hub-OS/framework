use crate::java::io::JFile;

/// https://developer.android.com/reference/android/net/Uri
///
/// API level 1
jni::bind_java_type! {
    pub AndroidUri => "android.net.Uri",
    type_map {
        JFile => "java.io.File",
    },
    methods {
        /// https://developer.android.com/reference/android/net/Uri#fromFile(java.io.File)
        ///
        /// API level 1
        pub static fn from_file(file: JFile) -> AndroidUri,

        /// https://developer.android.com/reference/android/net/Uri#fromParts(java.lang.String,%20java.lang.String,%20java.lang.String)
        ///
        /// API level 1
        pub static fn from_parts(scheme: JString, ssp: JString, fragment: JString) -> AndroidUri,
    }
}
