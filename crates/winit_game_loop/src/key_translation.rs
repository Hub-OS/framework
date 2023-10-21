use input::*;
use winit::keyboard::KeyCode as WinitKeyCode;

pub(super) fn translate_winit_key(winit_key: WinitKeyCode) -> Option<Key> {
    match winit_key {
        WinitKeyCode::Backquote => Some(Key::Backquote),
        WinitKeyCode::Digit1 => Some(Key::Key1),
        WinitKeyCode::Digit2 => Some(Key::Key2),
        WinitKeyCode::Digit3 => Some(Key::Key3),
        WinitKeyCode::Digit4 => Some(Key::Key4),
        WinitKeyCode::Digit5 => Some(Key::Key5),
        WinitKeyCode::Digit6 => Some(Key::Key6),
        WinitKeyCode::Digit7 => Some(Key::Key7),
        WinitKeyCode::Digit8 => Some(Key::Key8),
        WinitKeyCode::Digit9 => Some(Key::Key9),
        WinitKeyCode::Digit0 => Some(Key::Key0),
        WinitKeyCode::Minus => Some(Key::Minus),
        WinitKeyCode::Equal => Some(Key::Equal),
        WinitKeyCode::Backspace => Some(Key::Backspace),
        WinitKeyCode::Numpad1 => Some(Key::Numpad1),
        WinitKeyCode::Numpad2 => Some(Key::Numpad2),
        WinitKeyCode::Numpad3 => Some(Key::Numpad3),
        WinitKeyCode::Numpad4 => Some(Key::Numpad4),
        WinitKeyCode::Numpad5 => Some(Key::Numpad5),
        WinitKeyCode::Numpad6 => Some(Key::Numpad6),
        WinitKeyCode::Numpad7 => Some(Key::Numpad7),
        WinitKeyCode::Numpad8 => Some(Key::Numpad8),
        WinitKeyCode::Numpad9 => Some(Key::Numpad9),
        WinitKeyCode::Numpad0 => Some(Key::Numpad0),
        WinitKeyCode::NumpadAdd => Some(Key::NumpadPlus),
        WinitKeyCode::NumpadDivide => Some(Key::NumpadDivide),
        WinitKeyCode::NumpadDecimal => Some(Key::NumpadDecimal),
        WinitKeyCode::NumpadComma => Some(Key::NumpadComma),
        WinitKeyCode::NumpadEnter => Some(Key::NumpadEnter),
        WinitKeyCode::NumpadEqual => Some(Key::NumpadEqual),
        WinitKeyCode::NumpadMultiply => Some(Key::NumpadMultiply),
        WinitKeyCode::NumpadSubtract => Some(Key::NumpadMinus),
        WinitKeyCode::KeyA => Some(Key::A),
        WinitKeyCode::KeyB => Some(Key::B),
        WinitKeyCode::KeyC => Some(Key::C),
        WinitKeyCode::KeyD => Some(Key::D),
        WinitKeyCode::KeyE => Some(Key::E),
        WinitKeyCode::KeyF => Some(Key::F),
        WinitKeyCode::KeyG => Some(Key::G),
        WinitKeyCode::KeyH => Some(Key::H),
        WinitKeyCode::KeyI => Some(Key::I),
        WinitKeyCode::KeyJ => Some(Key::J),
        WinitKeyCode::KeyK => Some(Key::K),
        WinitKeyCode::KeyL => Some(Key::L),
        WinitKeyCode::KeyM => Some(Key::M),
        WinitKeyCode::KeyN => Some(Key::N),
        WinitKeyCode::KeyO => Some(Key::O),
        WinitKeyCode::KeyP => Some(Key::P),
        WinitKeyCode::KeyQ => Some(Key::Q),
        WinitKeyCode::KeyR => Some(Key::R),
        WinitKeyCode::KeyS => Some(Key::S),
        WinitKeyCode::KeyT => Some(Key::T),
        WinitKeyCode::KeyU => Some(Key::U),
        WinitKeyCode::KeyV => Some(Key::V),
        WinitKeyCode::KeyW => Some(Key::W),
        WinitKeyCode::KeyX => Some(Key::X),
        WinitKeyCode::KeyY => Some(Key::Y),
        WinitKeyCode::KeyZ => Some(Key::Z),
        WinitKeyCode::Escape => Some(Key::Escape),
        WinitKeyCode::F1 => Some(Key::F1),
        WinitKeyCode::F2 => Some(Key::F2),
        WinitKeyCode::F3 => Some(Key::F3),
        WinitKeyCode::F4 => Some(Key::F4),
        WinitKeyCode::F5 => Some(Key::F5),
        WinitKeyCode::F6 => Some(Key::F6),
        WinitKeyCode::F7 => Some(Key::F7),
        WinitKeyCode::F8 => Some(Key::F8),
        WinitKeyCode::F9 => Some(Key::F9),
        WinitKeyCode::F10 => Some(Key::F10),
        WinitKeyCode::F11 => Some(Key::F11),
        WinitKeyCode::F12 => Some(Key::F12),
        WinitKeyCode::F13 => Some(Key::F13),
        WinitKeyCode::F14 => Some(Key::F14),
        WinitKeyCode::F15 => Some(Key::F15),
        WinitKeyCode::F16 => Some(Key::F16),
        WinitKeyCode::F17 => Some(Key::F17),
        WinitKeyCode::F18 => Some(Key::F18),
        WinitKeyCode::F19 => Some(Key::F19),
        WinitKeyCode::F20 => Some(Key::F20),
        WinitKeyCode::F21 => Some(Key::F21),
        WinitKeyCode::F22 => Some(Key::F22),
        WinitKeyCode::F23 => Some(Key::F23),
        WinitKeyCode::F24 => Some(Key::F24),
        WinitKeyCode::Cut => Some(Key::Cut),
        WinitKeyCode::Copy => Some(Key::Copy),
        WinitKeyCode::Paste => Some(Key::Paste),
        WinitKeyCode::Tab => Some(Key::Tab),
        WinitKeyCode::NumLock => Some(Key::NumLock),
        WinitKeyCode::CapsLock => Some(Key::CapsLock),
        WinitKeyCode::ScrollLock => Some(Key::ScrollLock),
        WinitKeyCode::Pause => Some(Key::Pause),
        WinitKeyCode::Insert => Some(Key::Insert),
        WinitKeyCode::Delete => Some(Key::Delete),
        WinitKeyCode::Home => Some(Key::Home),
        WinitKeyCode::End => Some(Key::End),
        WinitKeyCode::PageUp => Some(Key::PageUp),
        WinitKeyCode::PageDown => Some(Key::PageDown),
        WinitKeyCode::ShiftLeft => Some(Key::LShift),
        WinitKeyCode::ControlLeft => Some(Key::LControl),
        WinitKeyCode::SuperLeft => Some(Key::LMeta),
        WinitKeyCode::SuperRight => Some(Key::RMeta),
        WinitKeyCode::AltLeft => Some(Key::LAlt),
        WinitKeyCode::AltRight => Some(Key::RAlt),
        WinitKeyCode::ContextMenu => Some(Key::ContextMenu),
        WinitKeyCode::ControlRight => Some(Key::RControl),
        WinitKeyCode::ShiftRight => Some(Key::RShift),
        WinitKeyCode::ArrowLeft => Some(Key::Left),
        WinitKeyCode::ArrowRight => Some(Key::Right),
        WinitKeyCode::ArrowUp => Some(Key::Up),
        WinitKeyCode::ArrowDown => Some(Key::Down),
        WinitKeyCode::BracketLeft => Some(Key::LBracket),
        WinitKeyCode::BracketRight => Some(Key::RBracket),
        WinitKeyCode::Semicolon => Some(Key::Semicolon),
        WinitKeyCode::Quote => Some(Key::Apostrophe),
        WinitKeyCode::Enter => Some(Key::Return),
        WinitKeyCode::Space => Some(Key::Space),
        WinitKeyCode::NumpadStar => Some(Key::Asterisk),
        WinitKeyCode::LaunchApp2 => Some(Key::Calculator),
        WinitKeyCode::Comma => Some(Key::Comma),
        WinitKeyCode::Period => Some(Key::Period),
        WinitKeyCode::Slash => Some(Key::Slash),
        WinitKeyCode::Backslash => Some(Key::Backslash),
        WinitKeyCode::LaunchMail => Some(Key::Mail),
        WinitKeyCode::MediaPlayPause => Some(Key::MediaPlayPause),
        WinitKeyCode::MediaSelect => Some(Key::MediaSelect),
        WinitKeyCode::MediaStop => Some(Key::MediaStop),
        WinitKeyCode::MediaTrackPrevious => Some(Key::MediaPrev),
        WinitKeyCode::MediaTrackNext => Some(Key::MediaNext),
        WinitKeyCode::AudioVolumeMute => Some(Key::Mute),
        WinitKeyCode::LaunchApp1 => Some(Key::MyComputer),
        WinitKeyCode::BrowserFavorites => Some(Key::WebBookmarks),
        WinitKeyCode::BrowserBack => Some(Key::WebBack),
        WinitKeyCode::BrowserForward => Some(Key::WebForward),
        WinitKeyCode::BrowserHome => Some(Key::WebHome),
        WinitKeyCode::BrowserRefresh => Some(Key::WebRefresh),
        WinitKeyCode::BrowserSearch => Some(Key::WebSearch),
        WinitKeyCode::BrowserStop => Some(Key::WebStop),
        _ => None,
    }
}