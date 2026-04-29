pub trait GameClipboard {
    fn request_text(&mut self) -> String;

    fn set_text(&mut self, text: String) -> bool;
}

use copypasta::{ClipboardContext, ClipboardProvider};

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
    clipboard: Option<ClipboardContext>,
}

impl DesktopClipboard {
    pub fn new() -> Self {
        Self {
            clipboard: ClipboardContext::new().ok(),
        }
    }
}

impl Default for DesktopClipboard {
    fn default() -> Self {
        Self::new()
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
