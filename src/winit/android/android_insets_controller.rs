use jni::objects::{JObject, JValueGen};
use jni::JNIEnv;

pub enum InsetsType {
    NavigationBars = 1 << 1,
    Ime = 1 << 3,
    DisplayCutout = 1 << 7,
}

pub enum InsetsBehavior {
    BehaviorDefault = 1,
    BehaviorShowTransientBarsBySwipe = 2,
}

impl<O> From<InsetsBehavior> for JValueGen<O> {
    fn from(other: InsetsBehavior) -> Self {
        (other as i32).into()
    }
}

pub struct AndroidInsetsController<'a> {
    j_object: JObject<'a>,
}

impl<'a> AndroidInsetsController<'a> {
    pub fn set_system_bars_behavior(
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

    pub fn show(&self, jni_env: &mut JNIEnv<'a>, flags: i32) -> jni::errors::Result<()> {
        jni_env.call_method(&self.j_object, "show", "(I)V", &[flags.into()])?;
        Ok(())
    }

    pub fn hide(&self, jni_env: &mut JNIEnv<'a>, flags: i32) -> jni::errors::Result<()> {
        jni_env.call_method(&self.j_object, "hide", "(I)V", &[flags.into()])?;
        Ok(())
    }
}

impl<'a> From<JObject<'a>> for AndroidInsetsController<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}
