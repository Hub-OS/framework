use super::translate_sdl_key;
use framework_core::runtime::{GameWindowEvent, GameWindowLifecycle, InputEvent};
use input::*;
use math::*;
use sdl3::event::{Event as SDLEvent, WindowEvent as SDLWindowEvent};
use sdl3::gamepad::Axis as SDLAxis;
use sdl3::gamepad::Button as SDLButton;
use sdl3::mouse::MouseButton as SDLMouseButton;

pub(crate) fn translate_sdl_event(
    window: &dyn GameWindowLifecycle,
    game_window_id: u32,
    event: SDLEvent,
) -> Option<GameWindowEvent> {
    match event {
        SDLEvent::Quit { .. } => Some(GameWindowEvent::CloseRequested),
        SDLEvent::Window {
            window_id,
            win_event,
            ..
        } if game_window_id == window_id => match win_event {
            SDLWindowEvent::Resized(width, height) => Some(GameWindowEvent::Resized(UVec2::new(
                width as u32,
                height as u32,
            ))),
            SDLWindowEvent::Moved(x, y) => Some(GameWindowEvent::Moved(IVec2::new(x, y))),
            _ => None,
        },
        SDLEvent::TextInput {
            window_id, text, ..
        } => {
            if game_window_id == window_id {
                Some(InputEvent::Text(text).into())
            } else {
                None
            }
        }
        SDLEvent::DropBegin { window_id, .. } => {
            if game_window_id == window_id {
                Some(InputEvent::DropStart.into())
            } else {
                None
            }
        }
        SDLEvent::DropComplete { window_id, .. } => {
            if game_window_id == window_id {
                Some(InputEvent::DropCancelled.into())
            } else {
                None
            }
        }
        SDLEvent::DropFile {
            filename,
            window_id,
            ..
        } => {
            if game_window_id == window_id {
                Some(InputEvent::DroppedFile(filename.into()).into())
            } else {
                None
            }
        }
        SDLEvent::DropText {
            filename,
            window_id,
            ..
        } => {
            if game_window_id == window_id {
                Some(InputEvent::DroppedText(filename).into())
            } else {
                None
            }
        }
        SDLEvent::FingerDown {
            finger_id,
            x,
            y,
            pressure,
            ..
        }
        | SDLEvent::FingerMotion {
            finger_id,
            x,
            y,
            pressure,
            ..
        }
        | SDLEvent::FingerUp {
            finger_id,
            x,
            y,
            pressure,
            ..
        } => {
            let phase = match event {
                SDLEvent::FingerDown { .. } => TouchPhase::Start,
                SDLEvent::FingerMotion { .. } => TouchPhase::Moving,
                SDLEvent::FingerUp { .. } => TouchPhase::End,
                _ => unreachable!(),
            };

            let touch = Touch {
                id: finger_id,
                phase,
                position: Vec2::new(x * 2.0 - 1.0, -(y * 2.0 - 1.0)),
                pressure: Some(pressure),
            };

            Some(InputEvent::Touch(touch).into())
        }
        SDLEvent::MouseMotion {
            window_id, x, y, ..
        } => {
            if game_window_id == window_id {
                let position = Vec2::new(x, y);
                let position = window.normalize_vec2(position);

                Some(InputEvent::MouseMoved(position).into())
            } else {
                None
            }
        }
        SDLEvent::MouseButtonDown {
            window_id,
            mouse_btn,
            ..
        } => {
            if game_window_id == window_id {
                Some(InputEvent::MouseButtonDown(translate_sdl_mouse_button(mouse_btn)?).into())
            } else {
                None
            }
        }
        SDLEvent::MouseButtonUp {
            window_id,
            mouse_btn,
            ..
        } => {
            if game_window_id == window_id {
                Some(InputEvent::MouseButtonUp(translate_sdl_mouse_button(mouse_btn)?).into())
            } else {
                None
            }
        }
        SDLEvent::KeyDown {
            window_id, keycode, ..
        } => {
            if game_window_id == window_id {
                keycode
                    .and_then(translate_sdl_key)
                    .map(|key| GameWindowEvent::InputEvent(InputEvent::KeyDown(key)))
            } else {
                None
            }
        }
        SDLEvent::KeyUp {
            window_id, keycode, ..
        } => {
            if game_window_id == window_id {
                keycode
                    .and_then(translate_sdl_key)
                    .map(|key| GameWindowEvent::InputEvent(InputEvent::KeyUp(key)))
            } else {
                None
            }
        }
        SDLEvent::GamepadRemoved { which, .. } => {
            Some(InputEvent::ControllerDisconnected(which.raw() as usize).into())
        }
        SDLEvent::GamepadButtonDown { which, button, .. } => Some(
            InputEvent::ControllerButtonDown {
                controller_id: which.raw() as usize,
                button: convert_button(button)?,
            }
            .into(),
        ),
        SDLEvent::GamepadButtonUp { which, button, .. } => Some(
            InputEvent::ControllerButtonUp {
                controller_id: which.raw() as usize,
                button: convert_button(button)?,
            }
            .into(),
        ),
        SDLEvent::GamepadAxisMotion {
            which, axis, value, ..
        } => Some({
            let axis = convert_axis(axis);
            let mut value = ((value as f32) / 32767.0).max(-1.0);

            if axis.is_y_axis() {
                value = -value;
            }

            InputEvent::ControllerAxis {
                controller_id: which.raw() as usize,
                axis,
                value,
            }
            .into()
        }),
        _ => None,
    }
}

fn convert_button(button: SDLButton) -> Option<Button> {
    match button {
        SDLButton::South => Some(Button::A),
        SDLButton::East => Some(Button::B),
        SDLButton::West => Some(Button::X),
        SDLButton::North => Some(Button::Y),
        SDLButton::Guide => Some(Button::Meta),
        SDLButton::Back => Some(Button::Select),
        SDLButton::Start => Some(Button::Start),
        SDLButton::LeftStick => Some(Button::LeftStick),
        SDLButton::RightStick => Some(Button::RightStick),
        SDLButton::LeftShoulder => Some(Button::LeftShoulder),
        SDLButton::RightShoulder => Some(Button::RightShoulder),
        SDLButton::DPadUp => Some(Button::DPadUp),
        SDLButton::DPadDown => Some(Button::DPadDown),
        SDLButton::DPadLeft => Some(Button::DPadLeft),
        SDLButton::DPadRight => Some(Button::DPadRight),
        SDLButton::LeftPaddle1 => Some(Button::Paddle1),
        SDLButton::RightPaddle1 => Some(Button::Paddle2),
        SDLButton::LeftPaddle2 => Some(Button::Paddle3),
        SDLButton::RightPaddle2 => Some(Button::Paddle4),
        _ => None,
    }
}

fn convert_axis(axis: SDLAxis) -> AnalogAxis {
    match axis {
        SDLAxis::LeftX => AnalogAxis::LeftStickX,
        SDLAxis::LeftY => AnalogAxis::LeftStickY,
        SDLAxis::RightX => AnalogAxis::RightStickX,
        SDLAxis::RightY => AnalogAxis::RightStickY,
        SDLAxis::TriggerLeft => AnalogAxis::LeftTrigger,
        SDLAxis::TriggerRight => AnalogAxis::RightTrigger,
    }
}

fn translate_sdl_mouse_button(button: SDLMouseButton) -> Option<MouseButton> {
    match button {
        SDLMouseButton::Left => Some(MouseButton::Left),
        SDLMouseButton::Middle => Some(MouseButton::Middle),
        SDLMouseButton::Right => Some(MouseButton::Right),
        _ => None,
    }
}
