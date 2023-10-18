use cfg_macros::cfg_android;

#[cfg(not(target_os = "android"))]
#[derive(Default, Clone)]
pub struct WinitPlatformApp {}

cfg_android! {
    use winit::platform::android::activity::AndroidApp;

    pub type WinitPlatformApp = AndroidApp;
}
