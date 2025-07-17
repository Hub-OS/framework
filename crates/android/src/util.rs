use crate::activity::AndroidApp;
use crate::view::*;
use crate::AndroidJVM;

pub fn show_system_bars(app: &AndroidApp) {
    let vm = AndroidJVM::from(app);

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let insets_controller = activity_window.get_insets_controller(jni_env)?;

        insets_controller
            .set_system_bars_behavior(jni_env, AndroidWindowInsetsBehavior::BehaviorDefault)?;

        let navigation_bars = AndroidWindowInsetsType::navigation_bars(jni_env)?;
        let display_cutout = AndroidWindowInsetsType::display_cutout(jni_env)?;
        insets_controller.show(jni_env, navigation_bars)?;
        insets_controller.hide(jni_env, display_cutout)?;

        Ok(())
    });
}

pub fn hide_system_bars(app: &AndroidApp) {
    let vm = AndroidJVM::from(app);

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let insets_controller = activity_window.get_insets_controller(jni_env)?;

        insets_controller.set_system_bars_behavior(
            jni_env,
            AndroidWindowInsetsBehavior::BehaviorShowTransientBarsBySwipe,
        )?;

        let navigation_bars = AndroidWindowInsetsType::navigation_bars(jni_env)?;
        let display_cutout = AndroidWindowInsetsType::display_cutout(jni_env)?;
        insets_controller.show(jni_env, display_cutout)?;
        insets_controller.hide(jni_env, navigation_bars)?;

        Ok(())
    });
}

pub fn show_ime(app: &AndroidApp) {
    // https://stackoverflow.com/questions/75477112/android-12-ignoring-showsoftinput-as-view-is-not-served

    let vm = AndroidJVM::from(app);

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let insets_controller = activity_window.get_insets_controller(jni_env)?;

        let ime = AndroidWindowInsetsType::ime(jni_env)?;
        insets_controller.show(jni_env, ime)?;

        Ok(())
    });
}

pub fn hide_ime(app: &AndroidApp) {
    let vm = AndroidJVM::from(app);

    vm.wrap(|jni_env| {
        let activity = AndroidActivity::from(app);
        let activity_window = activity.get_window(jni_env)?;
        let insets_controller = activity_window.get_insets_controller(jni_env)?;

        let ime = AndroidWindowInsetsType::ime(jni_env)?;
        insets_controller.hide(jni_env, ime)?;

        Ok(())
    });
}

pub fn get_ime_height(app: &AndroidApp) -> i32 {
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
