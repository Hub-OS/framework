use crate::AndroidPlatformApp;
use android::content::{AndroidClipData, AndroidContext};
use android::java::lang::JavaString;
use android::text::AndroidClipboardManager;
use android::AndroidJVM;
use framework_core::runtime::GameClipboard;

pub struct AndroidClipboard {
    app: AndroidPlatformApp,
}

impl AndroidClipboard {
    pub fn new(app: AndroidPlatformApp) -> Self {
        Self { app }
    }
}

impl GameClipboard for AndroidClipboard {
    fn request_text(&mut self) -> String {
        let vm = AndroidJVM::from(&self.app);
        let mut result = String::new();

        vm.wrap(|jni_env| {
            let android_context = AndroidContext::from(&self.app);
            let clipboard_service = android_context.clipboard_service(jni_env)?;
            let clipboard_manager = AndroidClipboardManager::from(clipboard_service);

            let Some(clip_data) = clipboard_manager.get_primary_clip(jni_env)? else {
                return Ok(());
            };

            if clip_data.get_item_count(jni_env)? > 0 {
                let clip_item = clip_data.get_item_at(jni_env, 0)?;
                let text = clip_item.get_text(jni_env)?;
                result = text
                    .to_java_string(jni_env)?
                    .get_jni_string(jni_env)?
                    .into();
            }

            Ok(())
        });

        result
    }

    fn set_text(&mut self, text: String) -> bool {
        let vm = AndroidJVM::from(&self.app);
        let mut success = false;

        vm.wrap(|jni_env| {
            let android_context = AndroidContext::from(&self.app);
            let clipboard_service = android_context.clipboard_service(jni_env)?;
            let clipboard_manager = AndroidClipboardManager::from(clipboard_service);

            let label = JavaString::from_str(jni_env, "text")?.into();
            let content = JavaString::from_str(jni_env, &text)?.into();
            let clip_data = AndroidClipData::new_plain_text(jni_env, &label, &content)?;

            clipboard_manager.set_primary_clip(jni_env, &clip_data)?;

            success = true;

            Ok(())
        });

        success
    }
}
