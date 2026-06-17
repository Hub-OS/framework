use crate::activity::AndroidApp;
use jni::JavaVM;

pub struct AndroidJVM {
    vm: JavaVM,
}

impl From<&AndroidApp> for AndroidJVM {
    fn from(app: &AndroidApp) -> Self {
        Self {
            vm: unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) },
        }
    }
}

impl AndroidJVM {
    pub fn wrap(&self, func: impl FnOnce(&mut jni::Env) -> jni::errors::Result<()>) {
        let res = self
            .vm
            .attach_current_thread(|jni_env| match func(jni_env) {
                Ok(_) => Ok(()),
                Err(jni::errors::Error::JavaException) => {
                    jni_env.exception_describe();
                    jni_env.exception_clear();
                    Ok(())
                }
                Err(err) => Err(err),
            });

        if let Err(err) = res {
            log::error!("{err:?}")
        }
    }
}
