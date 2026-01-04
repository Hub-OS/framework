use crate::key_translation::{translate_android_button, translate_android_key};
use android::util::is_this_device_a_controller;
use android_activity::input::{
    Axis as AndroidAxis, InputEvent as AndroidInputEvent, KeyAction as AndroidKeyAction,
    KeyEvent as AndroidKeyEvent, KeyMapChar as AndroidKeyMapChar, Keycode as AndroidKeyCode,
    MotionAction as AndroidMotionAction, Source as AndroidInputSource,
};
use android_activity::AndroidApp;
use framework_core::common::GameWindow;
use framework_core::runtime::InputEvent;
use input::{AnalogAxis, Touch, TouchPhase};
use math::Vec2;

pub(crate) fn translate_input_event(
    app: &AndroidApp,
    window: &dyn GameWindow,
    combining_accent: &mut Option<char>,
    event: &AndroidInputEvent,
    mut push: impl FnMut(InputEvent),
) {
    match event {
        AndroidInputEvent::KeyEvent(key_event) => {
            let key_code = key_event.key_code();

            // check for controller input
            match key_event.action() {
                AndroidKeyAction::Down => {
                    translate_android_button(key_code, |button| {
                        push(InputEvent::ControllerButtonDown {
                            controller_id: 0,
                            button,
                        });
                    });
                }
                AndroidKeyAction::Up => {
                    translate_android_button(key_code, |button| {
                        push(InputEvent::ControllerButtonUp {
                            controller_id: 0,
                            button,
                        });
                    });
                }
                _ => {}
            }

            if !is_this_device_a_controller(app, key_event.device_id())
                || key_event.source() == AndroidInputSource::Keyboard
            {
                // key down + up events
                match key_event.action() {
                    AndroidKeyAction::Down => {
                        if let Some(key) = translate_android_key(key_code) {
                            push(InputEvent::KeyDown(key));
                        }
                    }
                    AndroidKeyAction::Up => {
                        if let Some(key) = translate_android_key(key_code) {
                            push(InputEvent::KeyUp(key));
                        }
                    }
                    _ => {}
                }

                // text events
                let combined_key_char =
                    character_map_and_combine_key(&app, key_event, combining_accent);

                if matches!(
                    key_event.action(),
                    AndroidKeyAction::Down | AndroidKeyAction::Multiple
                ) {
                    if let Some(AndroidKeyMapChar::Unicode(c)) = combined_key_char {
                        push(InputEvent::Text(c.to_string()));
                    } else if key_code == AndroidKeyCode::Del {
                        push(InputEvent::Text(String::from("\u{8}")));
                    } else if key_code == AndroidKeyCode::ForwardDel {
                        push(InputEvent::Text(String::from("\u{7f}")));
                    }
                }
            }
        }
        AndroidInputEvent::MotionEvent(motion_event) => match motion_event.source() {
            AndroidInputSource::Dpad
            | AndroidInputSource::Gamepad
            | AndroidInputSource::Joystick
                if motion_event.action() == AndroidMotionAction::Move =>
            {
                const AXIS_LIST: &[(AndroidAxis, AnalogAxis, f32)] = &[
                    (AndroidAxis::X, AnalogAxis::LeftStickX, 1.0),
                    (AndroidAxis::Y, AnalogAxis::LeftStickY, -1.0),
                    (AndroidAxis::Ltrigger, AnalogAxis::LeftTrigger, 1.0),
                    (AndroidAxis::Rtrigger, AnalogAxis::RightTrigger, 1.0),
                    (AndroidAxis::HatX, AnalogAxis::DPadX, 1.0),
                    (AndroidAxis::HatY, AnalogAxis::DPadY, 1.0),
                    (AndroidAxis::Z, AnalogAxis::RightStickX, 1.0),
                    (AndroidAxis::Rz, AnalogAxis::RightStickY, -1.0),
                ];

                for pointer in motion_event.pointers() {
                    for &(android_axis, axis, scale) in AXIS_LIST {
                        push(InputEvent::ControllerAxis {
                            controller_id: 0,
                            axis,
                            value: pointer.axis_value(android_axis) * scale,
                        });
                    }
                }
            }

            AndroidInputSource::Touchscreen => {
                let action_pointer_index = motion_event.pointer_index();

                for pointer in motion_event.pointers() {
                    let phase = if pointer.pointer_index() == action_pointer_index {
                        match motion_event.action() {
                            AndroidMotionAction::Down | AndroidMotionAction::PointerDown => {
                                TouchPhase::Start
                            }
                            AndroidMotionAction::Up | AndroidMotionAction::PointerUp => {
                                TouchPhase::End
                            }
                            AndroidMotionAction::Move => TouchPhase::Moving,
                            _ => TouchPhase::Cancelled,
                        }
                    } else {
                        TouchPhase::Moving
                    };

                    push(InputEvent::Touch(Touch {
                        id: pointer.pointer_id() as _,
                        phase,
                        position: window.normalize_vec2(Vec2::new(pointer.x(), pointer.y())),
                        pressure: Some(pointer.pressure()),
                    }));
                }
            }
            _ => {}
        },
        AndroidInputEvent::TextEvent(state) => {
            #[cfg(debug_assertions)]
            println!("Input Method State: {state:?}");
        }
        _ => {}
    }
}

/// Copied from: https://github.com/rust-mobile/android-activity/blob/main/examples/na-mainloop/src/lib.rs
fn character_map_and_combine_key(
    app: &AndroidApp,
    key_event: &AndroidKeyEvent,
    combining_accent: &mut Option<char>,
) -> Option<AndroidKeyMapChar> {
    let device_id = key_event.device_id();

    let key_map = match app.device_key_character_map(device_id) {
        Ok(key_map) => key_map,
        Err(err) => {
            log::error!("Failed to look up `KeyCharacterMap` for device {device_id}: {err:?}");
            return None;
        }
    };

    match key_map.get(key_event.key_code(), key_event.meta_state()) {
        Ok(AndroidKeyMapChar::Unicode(unicode)) => {
            // Only do dead key combining on key down
            if key_event.action() == AndroidKeyAction::Down {
                let combined_unicode = if let Some(accent) = combining_accent {
                    match key_map.get_dead_char(*accent, unicode) {
                        Ok(Some(key)) => Some(key),
                        Ok(None) => None,
                        Err(err) => {
                            log::error!("KeyEvent: Failed to combine 'dead key' accent '{accent}' with '{unicode}': {err:?}");
                            None
                        }
                    }
                } else {
                    Some(unicode)
                };
                *combining_accent = None;
                combined_unicode.map(|unicode| AndroidKeyMapChar::Unicode(unicode))
            } else {
                Some(AndroidKeyMapChar::Unicode(unicode))
            }
        }
        Ok(AndroidKeyMapChar::CombiningAccent(accent)) => {
            if key_event.action() == AndroidKeyAction::Down {
                *combining_accent = Some(accent);
            }
            Some(AndroidKeyMapChar::CombiningAccent(accent))
        }
        Ok(AndroidKeyMapChar::None) => None,
        Err(err) => {
            log::error!("KeyEvent: Failed to get key map character: {err:?}");
            *combining_accent = None;
            None
        }
    }
}
