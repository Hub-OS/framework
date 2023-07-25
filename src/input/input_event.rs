use crate::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)] // not every event is supported on every platform
pub(crate) enum InputEvent {
    Text(String),
    DropStart,
    DropCancelled,
    DroppedFile(PathBuf),
    #[allow(dead_code)] // winit needs support: https://github.com/rust-windowing/winit/issues/720
    DroppedText(String),
    Touch(Touch),
    MouseMoved(Vec2),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    KeyDown(Key),
    KeyUp(Key),
    ControllerConnected {
        controller_id: usize,
        rumble_pack: RumblePack,
    },
    ControllerDisconnected(usize),
    ControllerButtonDown {
        controller_id: usize,
        button: Button,
    },
    ControllerButtonUp {
        controller_id: usize,
        button: Button,
    },
    ControllerAxis {
        controller_id: usize,
        axis: AnalogAxis,
        value: f32,
    },
}
