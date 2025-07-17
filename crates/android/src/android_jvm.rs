use crate::activity::AndroidApp;
use jni::{JNIEnv, JavaVM};

pub struct AndroidJVM {
    vm: JavaVM,
}

impl From<&AndroidApp> for AndroidJVM {
    fn from(app: &AndroidApp) -> Self {
        Self {
            vm: unsafe { JavaVM::from_raw(std::mem::transmute(app.vm_as_ptr())).unwrap() },
        }
    }
}

impl AndroidJVM {
    pub fn wrap(&self, func: impl FnOnce(&mut JNIEnv) -> jni::errors::Result<()>) {
        let mut jni_env = self.vm.get_env().unwrap();

        match func(&mut jni_env) {
            Ok(()) => {}
            Err(jni::errors::Error::JavaException) => {
                jni_env.exception_describe().unwrap();
                jni_env.exception_clear().unwrap();
            }
            Err(err) => {
                log::error!("{err:?}");
            }
        }
    }
}
