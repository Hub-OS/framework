use crate::cfg_android;

cfg_android! {
  use winit::platform::android::activity::AndroidApp;

  pub type PlatformApp = AndroidApp;
}

#[cfg(not(target_os = "android"))]
#[derive(Default)]
pub struct PlatformApp {}
