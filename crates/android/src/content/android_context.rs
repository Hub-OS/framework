use crate::net::wifi::AndroidWifiManager;
use crate::text::AndroidClipboardManager;
use android_activity::AndroidApp;
use jni::objects::JString;

/// https://developer.android.com/reference/android/content/Context
///
/// API level 1
jni::bind_java_type! {
    pub AndroidContext => "android.content.Context",
    methods {
        /// https://developer.android.com/reference/android/content/Context#getSystemService(java.lang.String)
        ///
        /// API level 1
        pub fn get_system_service(name: JString) -> JObject,
    }
}

impl<'a> AndroidContext<'a> {
    /// https://developer.android.com/reference/android/content/Context#CLIPBOARD_SERVICE
    ///
    /// API level 1
    pub fn clipboard_service(
        &self,
        jni_env: &mut jni::Env<'a>,
    ) -> jni::errors::Result<AndroidClipboardManager<'a>> {
        let service_name = JString::from_jni_str(jni_env, jni::jni_str!("clipboard"))?;

        let o = self.get_system_service(jni_env, service_name)?;

        jni_env.cast_local::<AndroidClipboardManager>(o)
    }

    /// https://developer.android.com/reference/android/content/Context#WIFI_SERVICE
    ///
    /// API level 1
    pub fn wifi_service(
        &self,
        jni_env: &mut jni::Env<'a>,
    ) -> jni::errors::Result<AndroidWifiManager<'a>> {
        let service_name = JString::from_jni_str(jni_env, jni::jni_str!("wifi"))?;

        let o = self.get_system_service(jni_env, service_name)?;

        jni_env.cast_local::<AndroidWifiManager>(o)
    }
}

impl<'a> AndroidContext<'a> {
    pub fn from_app(jni_env: &jni::Env<'a>, app: &AndroidApp) -> Self {
        unsafe { Self::from_raw(jni_env, app.activity_as_ptr().cast()) }
    }
}
