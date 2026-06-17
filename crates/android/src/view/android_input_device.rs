/// https://developer.android.com/reference/android/view/InputDevice
///
/// API level 9
jni::bind_java_type! {
    pub AndroidInputDevice => "android.view.InputDevice",
    methods {
        /// https://developer.android.com/reference/android/view/InputDevice#getDevice(int)
        ///
        /// API level 9
        pub static fn get_device(id: jint) -> AndroidInputDevice,

        /// https://developer.android.com/reference/android/view/InputDevice#getControllerNumber()
        ///
        /// API level 19
        pub fn get_controller_number() -> jint,
    }
}
