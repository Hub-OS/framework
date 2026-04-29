use jni::objects::{JObject, JValue};
use jni::JNIEnv;

pub struct JavaString<'a> {
    j_object: JObject<'a>,
}

impl<'a> JavaString<'a> {
    pub fn from_str(jni_env: &mut JNIEnv<'a>, s: &str) -> jni::errors::Result<Self> {
        Ok(Self {
            j_object: jni_env.new_string(s)?.into(),
        })
    }

    pub fn get_jni_string(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<String> {
        let local_ref = jni_env.new_local_ref(&self.j_object)?;
        let jstring = jni::objects::JString::from(local_ref);
        let s = jni_env.get_string(&jstring)?.into();

        Ok(s)
    }
}

impl<'a> From<JObject<'a>> for JavaString<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}

impl<'object_ref, 'a> From<&'object_ref JavaString<'a>> for JValue<'object_ref, 'a> {
    fn from(other: &'object_ref JavaString<'a>) -> Self {
        JValue::Object(&other.j_object)
    }
}

impl<'a> From<JavaString<'a>> for JavaCharSequence<'a> {
    fn from(s: JavaString<'a>) -> Self {
        s.j_object.into()
    }
}

pub struct JavaCharSequence<'a> {
    j_object: JObject<'a>,
}

impl<'a> JavaCharSequence<'a> {
    pub fn to_java_string(&self, jni_env: &mut JNIEnv<'a>) -> jni::errors::Result<JavaString<'a>> {
        let java_string =
            jni_env.call_method(&self.j_object, "toString", "()Ljava/lang/String;", &[])?;

        Ok(JavaString::from(java_string.l()?))
    }
}

impl<'a> From<JObject<'a>> for JavaCharSequence<'a> {
    fn from(j_object: JObject<'a>) -> Self {
        Self { j_object }
    }
}

impl<'object_ref, 'a> From<&'object_ref JavaCharSequence<'a>> for JValue<'object_ref, 'a> {
    fn from(other: &'object_ref JavaCharSequence<'a>) -> Self {
        JValue::Object(&other.j_object)
    }
}
