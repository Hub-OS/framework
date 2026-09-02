use crate::java::io::JFile;

/// https://developer.android.com/reference/android/os/Environment
///
/// API level 1
jni::bind_java_type! {
    pub AndroidEnvironment => "android.os.Environment",
    type_map = {
        JFile => "java.io.File",
    },
    methods {
        /// https://developer.android.com/reference/android/os/Environment#getExternalStorageDirectory()
        ///
        /// API level 1
        pub static fn get_external_storage_directory() -> JFile,

        /// https://developer.android.com/reference/android/os/Environment#isExternalStorageManager()
        ///
        /// API level 30
        pub static fn is_external_storage_manager() -> bool,
    }
}
