jni::bind_java_type! {
    pub JFile => "java.io.File",
    methods {
        pub fn get_name() -> JString,

        pub fn get_path() -> JString,

        pub fn get_parent() -> JString,
        pub fn get_parent_file() -> JFile,

        pub fn get_canonical_path() -> JString,
        pub fn get_canonical_file() -> JFile,

        pub fn get_absolute_path() -> JString,
        pub fn get_absolute_file() -> JFile,
    }
}
