pub trait GameClipboard {
    fn request_text(&mut self) -> String;

    fn set_text(&mut self, text: String) -> bool;
}

use copypasta::ClipboardProvider;

/// Does not touch OS clipboard
#[derive(Default)]
pub struct PrivateClipboard {
    text: String,
}

impl PrivateClipboard {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GameClipboard for PrivateClipboard {
    fn request_text(&mut self) -> String {
        self.text.clone()
    }

    fn set_text(&mut self, text: String) -> bool {
        self.text = text;
        true
    }
}

/// MacOS, Linux, and Windows compatible clipboard
pub struct DesktopClipboard {
    clipboard: Option<Box<dyn ClipboardProvider>>,
}

impl DesktopClipboard {
    pub fn new(#[allow(unused)] display_handle: Option<raw_window_handle::DisplayHandle>) -> Self {
        #[cfg(target_os = "linux")]
        {
            use raw_window_handle::RawDisplayHandle;

            if let Some(display_handle) = display_handle
                && let RawDisplayHandle::Wayland(wayland_handle) = display_handle.as_raw()
            {
                let (clipboard, _) = unsafe {
                    copypasta::wayland_clipboard::create_clipboards_from_external(
                        wayland_handle.display.as_ptr(),
                    )
                };

                return Self {
                    clipboard: Some(Box::new(clipboard)),
                };
            }
        }

        Self {
            clipboard: copypasta::ClipboardContext::new()
                .ok()
                .map(|clipboard| -> Box<dyn ClipboardProvider> { Box::new(clipboard) }),
        }
    }
}

impl GameClipboard for DesktopClipboard {
    fn request_text(&mut self) -> String {
        if let Some(clipboard) = &mut self.clipboard {
            clipboard.get_contents().unwrap_or_default()
        } else {
            String::new()
        }
    }

    fn set_text(&mut self, text: String) -> bool {
        if let Some(clipboard) = &mut self.clipboard {
            clipboard.set_contents(text).is_ok()
        } else {
            false
        }
    }
}
