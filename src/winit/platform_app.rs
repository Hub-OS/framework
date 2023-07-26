use crate::cfg_android;

#[cfg(not(target_os = "android"))]
#[derive(Default, Clone)]
pub struct PlatformApp {}

cfg_android! {
    use winit::platform::android::activity::AndroidApp;

    pub type PlatformApp = AndroidApp;
}

#[cfg(target_os = "android")]
pub(crate) mod android_platform {
    use super::PlatformApp;
    use jni::objects::{JObject, JValueGen};
    use jni::{JNIEnv, JavaVM};

    enum InsetsType {
        NavigationBars = 1 << 1,
        Ime = 1 << 3,
        DisplayCutout = 1 << 7,
    }

    //

    enum InsetsBehavior {
        BehaviorDefault = 1,
        BehaviorShowTransientBarsBySwipe = 2,
    }

    impl<O> From<InsetsBehavior> for JValueGen<O> {
        fn from(other: InsetsBehavior) -> Self {
            (other as i32).into()
        }
    }

    //

    struct AndroidActivity<'a> {
        j_object: JObject<'a>,
    }

    impl<'a> AndroidActivity<'a> {
        fn get_window(
            &self,
            jni_env: &mut JNIEnv<'a>,
        ) -> jni::errors::Result<AndroidActivityWindow<'a>> {
            let owned_obj =
                jni_env.call_method(&self.j_object, "getWindow", "()Landroid/view/Window;", &[])?;

            Ok(AndroidActivityWindow {
                j_object: JObject::try_from(owned_obj)?,
            })
        }
    }

    impl<'a> From<&PlatformApp> for AndroidActivity<'a> {
        fn from(app: &PlatformApp) -> AndroidActivity<'a> {
            let j_object = unsafe { JObject::from_raw(std::mem::transmute(app.activity_as_ptr())) };

            Self { j_object }
        }
    }

    //

    struct AndroidActivityWindow<'a> {
        j_object: JObject<'a>,
    }

    impl<'a> AndroidActivityWindow<'a> {
        // API 30
        fn get_insets_controller(
            &self,
            jni_env: &mut JNIEnv<'a>,
        ) -> jni::errors::Result<WindowInsetsController<'a>> {
            let owned_obj = jni_env.call_method(
                &self.j_object,
                "getInsetsController",
                "()Landroid/view/WindowInsetsController;",
                &[],
            )?;

            Ok(WindowInsetsController {
                j_object: JObject::try_from(owned_obj)?,
            })
        }
    }

    //

    struct WindowInsetsController<'a> {
        j_object: JObject<'a>,
    }

    impl<'a> WindowInsetsController<'a> {
        fn set_system_bars_behavior(
            &self,
            jni_env: &mut JNIEnv<'a>,
            inset_behavior: InsetsBehavior,
        ) -> jni::errors::Result<()> {
            jni_env.call_method(
                &self.j_object,
                "setSystemBarsBehavior",
                "(I)V",
                &[inset_behavior.into()],
            )?;
            Ok(())
        }

        fn show(&self, jni_env: &mut JNIEnv<'a>, flags: i32) -> jni::errors::Result<()> {
            jni_env.call_method(&self.j_object, "show", "(I)V", &[flags.into()])?;
            Ok(())
        }

        fn hide(&self, jni_env: &mut JNIEnv<'a>, flags: i32) -> jni::errors::Result<()> {
            jni_env.call_method(&self.j_object, "hide", "(I)V", &[flags.into()])?;
            Ok(())
        }
    }

    //

    struct AndroidJVM {
        vm: JavaVM,
    }

    impl From<&PlatformApp> for AndroidJVM {
        fn from(app: &PlatformApp) -> Self {
            Self {
                vm: unsafe { JavaVM::from_raw(std::mem::transmute(app.vm_as_ptr())).unwrap() },
            }
        }
    }

    impl AndroidJVM {
        fn wrap(&self, mut func: impl FnMut(&mut JNIEnv) -> jni::errors::Result<()>) {
            let mut jni_env = self.vm.get_env().unwrap();

            match func(&mut jni_env) {
                Ok(()) => {}
                Err(jni::errors::Error::JavaException) => {
                    jni_env.exception_describe().unwrap();
                    jni_env.exception_clear().unwrap();
                }
                Err(_) => {}
            }
        }
    }

    //

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

            insets_controller.set_system_bars_behavior(
                jni_env,
                InsetsBehavior::BehaviorShowTransientBarsBySwipe,
            )?;
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
}
