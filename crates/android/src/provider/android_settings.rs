/// https://developer.android.com/reference/android/provider/Settings
///
/// API level 1
jni::bind_java_type! {
    pub AndroidSettings => "android.provider.Settings",
    fields {
        /// https://developer.android.com/reference/android/provider/Settings#ACTION_MANAGE_ALL_FILES_ACCESS_PERMISSION
        ///
        /// API level 30
        pub static action_manage_all_files_access_permission {
            sig = JString,
            name = "ACTION_MANAGE_ALL_FILES_ACCESS_PERMISSION"
        }
    }
}
