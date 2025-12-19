use input::*;
use math::*;
use std::path::PathBuf;

pub enum InputEvent {
    Text(String),
    TextPreEdit(String, Option<(usize, usize)>),
    TextPreEditEnd,
    DropStart,
    DropCancelled,
    DroppedFile(PathBuf),
    DroppedText(String),
    Touch(Touch),
    MouseMoved(Vec2),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    KeyDown(Key),
    KeyUp(Key),
    ControllerConnected {
        controller_id: usize,
        rumble_pack: Box<dyn RumblePack>,
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
