use crate::AndroidPlatformApp;
use android::content::{AndroidClipData, AndroidContext};
use android::util::j_object_to_j_string;
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
            let android_context = AndroidContext::from_app(jni_env, &self.app);
            let clipboard_manager = android_context.clipboard_service(jni_env)?;

            let clip_data = clipboard_manager.get_primary_clip(jni_env)?;

            if clip_data.is_null() {
                return Ok(());
            };

            if clip_data.get_item_count(jni_env)? > 0 {
                let clip_item = clip_data.get_item_at(jni_env, 0)?;
                let text_sequence = clip_item.get_text(jni_env)?;
                result = j_object_to_j_string(jni_env, text_sequence)?.to_string();
            }

            Ok(())
        });

        result
    }

    fn set_text(&mut self, text: String) -> bool {
        let vm = AndroidJVM::from(&self.app);
        let mut success = false;

        vm.wrap(|jni_env| {
            let android_context = AndroidContext::from_app(jni_env, &self.app);
            let clipboard_manager = android_context.clipboard_service(jni_env)?;

            let label = jni_env.new_string("text")?;
            let label = label.as_char_sequence();
            let content = jni_env.new_string(text)?;
            let clip_data = AndroidClipData::new_plain_text(jni_env, &label, &content)?;

            clipboard_manager.set_primary_clip(jni_env, &clip_data)?;

            success = true;

            Ok(())
        });

        success
    }
}
