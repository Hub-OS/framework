#[macro_export]
macro_rules! cfg_desktop {
  ($($tt:tt)+) => {
    $crate::create_cfg_rules! { cfg(not(any(target_os = "android", target_arch = "wasm32"))), $($tt)+ }
  };
  () => {
    cfg!(not(any(target_os = "android", target_arch = "wasm32")))
  };
}

#[macro_export]
macro_rules! cfg_web {
  ($($tt:tt)+) => {
    $crate::create_cfg_rules! { cfg(target_arch = "wasm32"), $($tt)+ }
  };
  () => {
    cfg!(target_arch = "wasm32")
  };
}

#[macro_export]
macro_rules! cfg_desktop_and_web {
  ($($tt:tt)+) => {
    $crate::create_cfg_rules! { cfg(not(target_os = "android")), $($tt)+ }
  };
  () => {
    cfg!(not(target_os = "android"))
  };
}

#[macro_export]
macro_rules! cfg_android {
  ($($tt:tt)+) => {
    $crate::create_cfg_rules! { cfg(target_os = "android"), $($tt)+ }
  };
  () => {
    cfg!(target_os = "android")
  };
}

#[macro_export]
macro_rules! cfg_native {
  ($($tt:tt)+) => {
    $crate::create_cfg_rules! { cfg(not(target_arch = "wasm32")), $($tt)+ }
  };
  () => {
    cfg!(not(target_arch = "wasm32"))
  };
}

#[macro_export]
macro_rules! create_cfg_rules {
  ($meta:meta, $expr:expr) => {
    #[$meta]
    $expr
  };
  ($meta:meta, $($item:item)*) => {
    $(
      #[$meta]
      $item
    )+
  };
}
