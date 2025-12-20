use framework_core::raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use ndk::native_window::NativeWindow as AndroidNativeWindow;

pub(crate) struct AndroidWindowHandle {
    window: AndroidNativeWindow,
}

impl From<AndroidNativeWindow> for AndroidWindowHandle {
    fn from(window: AndroidNativeWindow) -> Self {
        Self { window }
    }
}

impl HasDisplayHandle for AndroidWindowHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::android())
    }
}

impl HasWindowHandle for AndroidWindowHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.window.window_handle()
    }
}
