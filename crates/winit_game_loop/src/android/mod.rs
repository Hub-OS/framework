#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod android_activity;
mod android_activity_window;
mod android_insets;
mod android_insets_controller;
mod android_jvm;
mod android_view;

use android_activity::*;
use android_activity_window::*;
use android_insets::*;
use android_insets_controller::*;
use android_view::*;

pub(crate) mod android_rumble_pack;
pub(crate) mod controller_event_pump;

pub use android_jvm::*;
pub use jni;

use super::WinitPlatformApp as PlatformApp;

pub(crate) fn show_system_bars(app: &PlatformApp) {
    let vm = AndroidJVM::from(app);

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let insets_controller = activity_window.get_insets_controller(jni_env)?;

        insets_controller.set_system_bars_behavior(jni_env, InsetsBehavior::BehaviorDefault)?;
        insets_controller.show(jni_env, InsetsType::NavigationBars as i32)?;
        insets_controller.hide(jni_env, InsetsType::DisplayCutout as i32)?;

        Ok(())
    });
}

pub(crate) fn hide_system_bars(app: &PlatformApp) {
    let vm = AndroidJVM::from(app);

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let insets_controller = activity_window.get_insets_controller(jni_env)?;

        insets_controller
            .set_system_bars_behavior(jni_env, InsetsBehavior::BehaviorShowTransientBarsBySwipe)?;
        insets_controller.show(jni_env, InsetsType::DisplayCutout as i32)?;
        insets_controller.hide(jni_env, InsetsType::NavigationBars as i32)?;

        Ok(())
    });
}

pub(crate) fn show_ime(app: &PlatformApp) {
    // https://stackoverflow.com/questions/75477112/android-12-ignoring-showsoftinput-as-view-is-not-served

    let vm = AndroidJVM::from(app);

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let insets_controller = activity_window.get_insets_controller(jni_env)?;

        insets_controller.show(jni_env, InsetsType::Ime as i32)?;

        Ok(())
    });
}

pub(crate) fn hide_ime(app: &PlatformApp) {
    let vm = AndroidJVM::from(app);

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let insets_controller = activity_window.get_insets_controller(jni_env)?;

        insets_controller.hide(jni_env, InsetsType::Ime as i32)?;

        Ok(())
    });
}

pub(crate) fn get_ime_height(app: &PlatformApp) -> i32 {
    let vm = AndroidJVM::from(app);

    let mut height = 0;

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let view = activity_window.get_decor_view(jni_env)?;
        let window_insets = view.get_root_window_insets(jni_env)?;
        let ime_mask = AndroidWindowInsetsType::ime(jni_env)?;
        let insets = window_insets.get_insets(jni_env, ime_mask)?;

        height = insets.bottom(jni_env)?;

        Ok(())
    });

    height
}
