use crate::WinitPlatformApp as PlatformApp;
use jni::{JNIEnv, JavaVM};

pub struct AndroidJVM {
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
    pub fn wrap(&self, mut func: impl FnMut(&mut JNIEnv) -> jni::errors::Result<()>) {
        let mut jni_env = self.vm.get_env().unwrap();

        match func(&mut jni_env) {
            Ok(()) => {}
            Err(jni::errors::Error::JavaException) => {
                jni_env.exception_describe().unwrap();
                jni_env.exception_clear().unwrap();
            }
            Err(err) => {
                logging::error!("{err:?}");
            }
        }
    }
}
