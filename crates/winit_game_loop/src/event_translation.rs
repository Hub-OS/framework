use cfg_macros::*;
use framework_core::runtime::{GameWindowEvent, GameWindowLifecycle, InputEvent};
use input::*;
use math::*;
use winit::event::MouseButton as WinitMouseButton;
use winit::event::WindowEvent as WinitWindowEvent;
use winit::keyboard::PhysicalKey;

// todo: winit needs support for DroppedText: https://github.com/rust-windowing/winit/issues/720

pub(crate) fn translate_winit_event(
    window: &dyn GameWindowLifecycle,
    event: winit::event::WindowEvent,
) -> Vec<GameWindowEvent> {
    match event {
        WinitWindowEvent::CloseRequested => vec![GameWindowEvent::CloseRequested],
        WinitWindowEvent::Resized(winit::dpi::PhysicalSize { width, height }) => {
            vec![GameWindowEvent::Resized(UVec2::new(width, height))]
        }
        WinitWindowEvent::Moved(position) => {
            vec![GameWindowEvent::Moved(IVec2::new(position.x, position.y))]
        }
        WinitWindowEvent::HoveredFile(_) => vec![InputEvent::DropStart.into()],
        WinitWindowEvent::HoveredFileCancelled => vec![InputEvent::DropCancelled.into()],
        WinitWindowEvent::DroppedFile(path_buf) => {
            vec![InputEvent::DroppedFile(path_buf).into()]
        }
        WinitWindowEvent::Touch(touch) => {
            let phase = match touch.phase {
                winit::event::TouchPhase::Started => TouchPhase::Start,
                winit::event::TouchPhase::Moved => TouchPhase::Moving,
                winit::event::TouchPhase::Ended => TouchPhase::End,
                winit::event::TouchPhase::Cancelled => TouchPhase::Cancelled,
            };

            let position = Vec2::new(touch.location.x as f32, touch.location.y as f32);

            let touch = Touch {
                id: touch.id,
                phase,
                position: window.normalize_vec2(position),
                pressure: touch.force.map(|f| f.normalized() as f32),
            };

            vec![InputEvent::Touch(touch).into()]
        }
        WinitWindowEvent::CursorMoved { position, .. } => {
            let position = Vec2::new(position.x as f32, position.y as f32);
            let normalized = window.normalize_vec2(position);

            vec![InputEvent::MouseMoved(normalized).into()]
        }
        WinitWindowEvent::MouseInput { state, button, .. } => {
            if let Some(button) = translate_winit_mouse_button(button) {
                if state == winit::event::ElementState::Pressed {
                    vec![InputEvent::MouseButtonDown(button).into()]
                } else {
                    vec![InputEvent::MouseButtonUp(button).into()]
                }
            } else {
                Vec::new()
            }
        }
        WinitWindowEvent::Ime(ime_event) => match ime_event {
            winit::event::Ime::Enabled => {
                vec![]
            }
            winit::event::Ime::Preedit(text, selection) => {
                vec![InputEvent::TextPreEdit(text, selection).into()]
            }
            winit::event::Ime::Commit(text) => vec![InputEvent::Text(text).into()],
            winit::event::Ime::Disabled => vec![InputEvent::TextPreEditEnd.into()],
        },
        #[allow(unused_variables)]
        WinitWindowEvent::KeyboardInput {
            event:
                winit::event::KeyEvent {
                    state,
                    physical_key,
                    logical_key,
                    text,
                    ..
                },
            ..
        } => {
            // multiple events
            let mut events = Vec::new();

            // key event
            let key_code = if let PhysicalKey::Code(key_code) = physical_key {
                Some(key_code)
            } else {
                None
            };

            if let Some(key) = key_code.and_then(super::translate_winit_key) {
                if state == winit::event::ElementState::Pressed {
                    events.push(InputEvent::KeyDown(key).into());
                } else {
                    events.push(InputEvent::KeyUp(key).into());
                }
            }

            if state == winit::event::ElementState::Pressed {
                // text event
                let text = {
                    cfg_desktop_and_web! { text.as_ref().map(|smol_string| smol_string.as_str()) }
                    cfg_android! { logical_key.to_text() }
                };

                if let Some(text) = text {
                    let text = if text == "\r" {
                        String::from("\n")
                    } else {
                        text.to_string()
                    };

                    events.push(InputEvent::Text(text).into());
                };
            }

            events
        }
        _ => Vec::new(),
    }
}

fn translate_winit_mouse_button(button: WinitMouseButton) -> Option<MouseButton> {
    match button {
        WinitMouseButton::Left => Some(MouseButton::Left),
        WinitMouseButton::Middle => Some(MouseButton::Middle),
        WinitMouseButton::Right => Some(MouseButton::Right),
        _ => None,
    }
}
