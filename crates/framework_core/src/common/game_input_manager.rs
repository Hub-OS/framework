use crate::runtime::*;
use copypasta::{ClipboardContext, ClipboardProvider};
use input::*;
use math::*;
use std::path::PathBuf;

pub struct GameInputManager {
    clipboard: Option<ClipboardContext>,
    latest_mouse_button: Option<MouseButton>,
    latest_key: Option<Key>,
    touches: Vec<Touch>,
    mouse_position: Vec2,
    previous_mouse_buttons: Vec<MouseButton>,
    pressed_mouse_buttons: Vec<MouseButton>,
    previous_keys: Vec<Key>,
    pressed_keys: Vec<Key>,
    repeated_keys: Vec<Key>,
    controllers: Vec<GameController>,
    dropping_data: bool,
    dropped_file: Option<PathBuf>,
    dropped_text: Option<String>,
    text: String,
    accept_text: bool,
    requires_ime_update: bool,
    pending_ime_cursor_area: Option<Rect>,
}

impl Default for GameInputManager {
    fn default() -> Self {
        Self {
            clipboard: ClipboardContext::new().ok(),
            latest_mouse_button: None,
            latest_key: None,
            touches: Vec::new(),
            mouse_position: Vec2::new(0.0, 0.0),
            previous_mouse_buttons: Vec::new(),
            pressed_mouse_buttons: Vec::new(),
            previous_keys: Vec::new(),
            pressed_keys: Vec::new(),
            repeated_keys: Vec::new(),
            controllers: Vec::new(),
            dropping_data: false,
            dropped_file: None,
            dropped_text: None,
            text: String::new(),
            accept_text: false,
            requires_ime_update: false,
            pending_ime_cursor_area: Default::default(),
        }
    }
}

impl GameInputManager {
    pub fn requires_ime_update(&self) -> bool {
        self.requires_ime_update
    }

    pub fn pending_ime_cursor_area_update(&self) -> Option<Rect> {
        self.pending_ime_cursor_area
    }

    pub fn accepting_text(&self) -> bool {
        self.accept_text
    }

    pub fn start_text_input(&mut self) {
        self.accept_text = true;
        self.requires_ime_update = true;
    }

    /// Relative to the render. Top left is (-1.0, 1.0), bottom right is (1.0, -1.0)
    pub fn set_ime_cursor_area(&mut self, area: Rect) {
        self.pending_ime_cursor_area = Some(area);
    }

    pub fn end_text_input(&mut self) {
        self.accept_text = false;
        self.requires_ime_update = true;
    }

    pub fn request_clipboard_text(&mut self) -> String {
        if let Some(clipboard) = &mut self.clipboard {
            clipboard.get_contents().unwrap_or_default()
        } else {
            String::new()
        }
    }

    pub fn set_clipboard_text(&mut self, text: String) -> bool {
        if let Some(clipboard) = &mut self.clipboard {
            clipboard.set_contents(text).is_ok()
        } else {
            false
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn touches(&self) -> &[Touch] {
        &self.touches
    }

    /// Relative to the render. Top left is (-1.0, 1.0), bottom right is (1.0, -1.0)
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn latest_mouse_button(&self) -> Option<MouseButton> {
        self.latest_mouse_button
    }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    pub fn was_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        !self.previous_mouse_buttons.contains(&button)
            && self.pressed_mouse_buttons.contains(&button)
    }

    pub fn was_mouse_button_released(&self, button: MouseButton) -> bool {
        self.previous_mouse_buttons.contains(&button)
            && !self.pressed_mouse_buttons.contains(&button)
    }

    pub fn controllers(&self) -> &[GameController] {
        self.controllers.as_slice()
    }

    pub fn controller(&self, id: usize) -> Option<&GameController> {
        self.controllers.iter().find(|c| c.id() == id)
    }

    fn controller_mut(&mut self, id: usize) -> Option<&mut GameController> {
        self.controllers.iter_mut().find(|c| c.id() == id)
    }

    pub fn latest_button(&self) -> Option<Button> {
        self.controllers
            .iter()
            .find(|controller| controller.latest_button().is_some())
            .and_then(|controller| controller.latest_button())
    }

    pub fn controller_axis(&self, id: usize, axis: AnalogAxis) -> f32 {
        if let Some(controller) = self.controller(id) {
            controller.axis(axis)
        } else {
            0.0
        }
    }

    pub fn is_button_down(&self, controller_id: usize, button: Button) -> bool {
        if let Some(controller) = self.controller(controller_id) {
            controller.is_button_down(button)
        } else {
            false
        }
    }

    pub fn was_button_just_pressed(&self, controller_id: usize, button: Button) -> bool {
        if let Some(controller) = self.controller(controller_id) {
            controller.was_button_just_pressed(button)
        } else {
            false
        }
    }

    pub fn was_button_released(&self, controller_id: usize, button: Button) -> bool {
        if let Some(controller) = self.controller(controller_id) {
            controller.was_button_released(button)
        } else {
            false
        }
    }

    pub fn keys_as_axis(&self, negative: Key, positive: Key) -> f32 {
        let mut value = 0.0;

        if self.pressed_keys.contains(&negative) {
            value -= 1.0;
        }

        if self.pressed_keys.contains(&positive) {
            value += 1.0;
        }

        value
    }

    pub fn latest_key(&self) -> Option<Key> {
        self.latest_key
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn is_key_repeated(&self, key: Key) -> bool {
        self.repeated_keys.contains(&key)
    }

    pub fn was_key_just_pressed(&self, key: Key) -> bool {
        !self.previous_keys.contains(&key) && self.pressed_keys.contains(&key)
    }

    pub fn was_key_released(&self, key: Key) -> bool {
        self.previous_keys.contains(&key) && !self.pressed_keys.contains(&key)
    }

    pub fn dropping_data(&self) -> bool {
        self.dropping_data
    }

    pub fn dropped_file(&self) -> Option<PathBuf> {
        self.dropped_file.clone()
    }

    pub fn dropped_text(&self) -> Option<String> {
        self.dropped_text.clone()
    }

    pub fn simulate_key_press(&mut self, key: Key) {
        if !self.pressed_keys.contains(&key) {
            self.latest_key = Some(key);
            self.pressed_keys.push(key);
        } else if !self.repeated_keys.contains(&key) {
            self.repeated_keys.push(key);
        }
    }

    pub fn simulate_key_release(&mut self, key: Key) {
        if let Some(index) = self.pressed_keys.iter().position(|v| *v == key) {
            self.pressed_keys.swap_remove(index);
        }
    }

    pub fn simulate_mouse_press(&mut self, button: MouseButton) {
        if !self.pressed_mouse_buttons.contains(&button) {
            self.latest_mouse_button = Some(button);
            self.pressed_mouse_buttons.push(button);
        }
    }

    pub fn simulate_mouse_release(&mut self, button: MouseButton) {
        if let Some(index) = self.pressed_mouse_buttons.iter().position(|v| *v == button) {
            self.pressed_mouse_buttons.swap_remove(index);
        }
    }

    pub fn simulate_mouse_move(&mut self, position: Vec2) {
        self.mouse_position = position;
    }

    pub(crate) fn flush(&mut self) {
        self.previous_mouse_buttons
            .clone_from(&self.pressed_mouse_buttons);
        self.repeated_keys.clear();
        self.previous_keys.clone_from(&self.pressed_keys);
        self.latest_mouse_button = None;
        self.latest_key = None;
        self.dropped_file = None;
        self.dropped_text = None;
        self.requires_ime_update = false;
        self.pending_ime_cursor_area = None;
        self.text.clear();
        self.touches.retain_mut(|touch| {
            if touch.phase == TouchPhase::Start {
                // change phase as it's not our first frame with this touch
                touch.phase = TouchPhase::Moving;
            }

            touch.phase != TouchPhase::End
        });

        for controller in &mut self.controllers {
            controller.flush();
        }
    }

    pub(crate) fn finalize_events(&mut self) {
        for controller in &mut self.controllers {
            controller.update_sticks();
        }
    }

    pub fn handle_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::Touch(mut touch) => {
                let mut touch_iter = self.touches.iter_mut();

                match touch_iter.find(|existing| existing.id == touch.id) {
                    Some(existing) => {
                        if existing.phase == TouchPhase::Start && touch.phase != TouchPhase::End {
                            // retain TouchPhase::Start
                            touch.phase = existing.phase;
                        }

                        *existing = touch
                    }
                    None => self.touches.push(touch),
                }
            }
            InputEvent::MouseMoved(position) => self.mouse_position = position,
            InputEvent::MouseButtonDown(button) => self.simulate_mouse_press(button),
            InputEvent::MouseButtonUp(button) => self.simulate_mouse_release(button),
            InputEvent::KeyDown(key) => self.simulate_key_press(key),
            InputEvent::KeyUp(key) => self.simulate_key_release(key),
            InputEvent::ControllerConnected {
                controller_id,
                rumble_pack,
            } => {
                self.controllers
                    .push(GameController::new(controller_id, rumble_pack));
            }
            InputEvent::ControllerDisconnected(id) => {
                if let Some(index) = self.controllers.iter().position(|c| c.id() == id) {
                    self.controllers.swap_remove(index);
                }
            }
            InputEvent::ControllerButtonDown {
                controller_id,
                button,
            } => {
                if let Some(controller) = self.controller_mut(controller_id) {
                    controller.simulate_button_press(button);
                }
            }
            InputEvent::ControllerButtonUp {
                controller_id,
                button,
            } => {
                if let Some(controller) = self.controller_mut(controller_id) {
                    controller.simulate_button_release(button);
                }
            }
            InputEvent::ControllerAxis {
                controller_id,
                axis,
                value,
            } => {
                if let Some(controller) = self.controller_mut(controller_id) {
                    controller.simulate_axis_movement(axis, value);
                }
            }
            InputEvent::Text(text) => {
                if self.accept_text {
                    self.text = text;
                }
            }
            InputEvent::DropStart => {
                self.dropping_data = true;
            }
            InputEvent::DropCancelled => {
                self.dropping_data = false;
            }
            InputEvent::DroppedText(text) => {
                self.dropped_text = Some(text);
                self.dropping_data = false;
            }
            InputEvent::DroppedFile(path_buf) => {
                self.dropped_file = Some(path_buf);
                self.dropping_data = false;
            }
        }
    }
}
