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
    MouseMoved {
        x: f32,
        y: f32,
    },
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

impl InputEvent {
    pub(crate) fn scale_mouse_event(&mut self, window: &Window) {
        let Self::MouseMoved { x, y } = self else {
            return;
        };

        let window_size = window.size().as_vec2();
        let scale = window_size / window.resolution().as_vec2();

        *x = *x / window_size.x * 2.0 - 1.0;
        *y = -(*y / window_size.y * 2.0 - 1.0);

        if scale.x > scale.y {
            *x *= scale.x / scale.y;
        } else {
            *y *= scale.y / scale.x;
        }
    }
}
