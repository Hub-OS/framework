#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod android_activity;
mod android_activity_window;
mod android_insets_controller;
mod android_jvm;
mod android_rumble_pack;
mod controller_event_pump;

use android_activity::*;
use android_activity_window::*;
use android_insets_controller::*;
use android_jvm::*;

pub(crate) use android_rumble_pack::*;
pub(crate) use controller_event_pump::*;

use super::WinitPlatformApp as PlatformApp;

pub fn show_system_bars(app: &PlatformApp) {
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

pub fn hide_system_bars(app: &PlatformApp) {
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

pub fn show_ime(app: &PlatformApp) {
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
