use crate::cfg_android;

#[cfg(not(target_os = "android"))]
#[derive(Default, Clone)]
pub struct PlatformApp {}

cfg_android! {
    use winit::platform::android::activity::AndroidApp;

    pub type PlatformApp = AndroidApp;
}
