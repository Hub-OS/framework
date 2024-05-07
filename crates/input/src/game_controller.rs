use math::*;
use std::time::Duration;
pub use strum::{EnumString, IntoStaticStr};

#[derive(EnumString, IntoStaticStr, Debug, PartialEq, Eq, Copy, Clone)]
pub enum AnalogAxis {
    DPadX,
    DPadY,
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}

impl AnalogAxis {
    pub fn is_x_axis(&self) -> bool {
        matches!(
            self,
            AnalogAxis::DPadX | AnalogAxis::LeftStickX | AnalogAxis::RightStickX
        )
    }

    pub fn is_y_axis(&self) -> bool {
        matches!(
            self,
            AnalogAxis::DPadY | AnalogAxis::LeftStickY | AnalogAxis::RightStickY
        )
    }
}

#[derive(EnumString, IntoStaticStr, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Button {
    Meta,
    Start,
    Select,
    A,
    B,
    X,
    Y,
    C,
    Z,
    LeftShoulder,
    LeftTrigger,
    RightShoulder,
    RightTrigger,
    LeftStick,
    RightStick,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    LeftStickUp,
    LeftStickDown,
    LeftStickLeft,
    LeftStickRight,
    RightStickUp,
    RightStickDown,
    RightStickLeft,
    RightStickRight,
    Paddle1,
    Paddle2,
    Paddle3,
    Paddle4,
}

pub trait RumblePack {
    fn rumble(&self, weak: f32, strong: f32, duration: Duration);
}

pub struct GameController {
    id: usize,
    rumble_pack: Box<dyn RumblePack>,
    latest_button: Option<Button>,
    previous_buttons: Vec<Button>,
    pressed_buttons: Vec<Button>,

    raw_left_stick_x: f32,
    raw_left_stick_y: f32,
    raw_right_stick_x: f32,
    raw_right_stick_y: f32,

    left_stick_x: f32,
    left_stick_y: f32,
    right_stick_x: f32,
    right_stick_y: f32,
    left_trigger: f32,
    right_trigger: f32,
    deadzone: f32,
}

impl GameController {
    pub fn new(id: usize, rumble_pack: Box<dyn RumblePack>, deadzone: f32) -> Self {
        Self {
            id,
            rumble_pack,
            latest_button: None,
            previous_buttons: Vec::new(),
            pressed_buttons: Vec::new(),

            raw_left_stick_x: 0.0,
            raw_left_stick_y: 0.0,
            raw_right_stick_x: 0.0,
            raw_right_stick_y: 0.0,

            left_stick_x: 0.0,
            left_stick_y: 0.0,
            right_stick_x: 0.0,
            right_stick_y: 0.0,
            left_trigger: 0.0,
            right_trigger: 0.0,
            deadzone,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn rumble(&self, weak: f32, strong: f32, duration: Duration) {
        self.rumble_pack.rumble(weak, strong, duration);
    }

    pub fn latest_button(&self) -> Option<Button> {
        self.latest_button
    }

    pub fn is_button_down(&self, button: Button) -> bool {
        self.pressed_buttons.contains(&button)
    }

    pub fn was_button_just_pressed(&self, button: Button) -> bool {
        !self.previous_buttons.contains(&button) && self.pressed_buttons.contains(&button)
    }

    pub fn was_button_released(&self, button: Button) -> bool {
        self.previous_buttons.contains(&button) && !self.pressed_buttons.contains(&button)
    }

    pub fn buttons_as_axis(&self, negative: Button, positive: Button) -> f32 {
        let mut value = 0.0;

        if self.pressed_buttons.contains(&negative) {
            value -= 1.0;
        }

        if self.pressed_buttons.contains(&positive) {
            value += 1.0;
        }

        value
    }

    pub fn axis(&self, axis: AnalogAxis) -> f32 {
        match axis {
            AnalogAxis::LeftTrigger => self.left_trigger,
            AnalogAxis::RightTrigger => self.right_trigger,
            AnalogAxis::DPadX => self.buttons_as_axis(Button::DPadLeft, Button::DPadRight),
            AnalogAxis::DPadY => self.buttons_as_axis(Button::DPadDown, Button::DPadUp),
            AnalogAxis::LeftStickX => self.left_stick_x,
            AnalogAxis::LeftStickY => self.left_stick_y,
            AnalogAxis::RightStickX => self.right_stick_x,
            AnalogAxis::RightStickY => self.right_stick_y,
        }
    }

    pub fn simulate_button_press(&mut self, button: Button) {
        if !self.pressed_buttons.contains(&button) {
            self.latest_button = Some(button);
            self.pressed_buttons.push(button);
        }
    }

    pub fn simulate_button_release(&mut self, button: Button) {
        if let Some(index) = self.pressed_buttons.iter().position(|v| *v == button) {
            self.pressed_buttons.swap_remove(index);
        }
    }

    pub fn simulate_axis_movement(&mut self, axis: AnalogAxis, value: f32) {
        match axis {
            AnalogAxis::DPadX | AnalogAxis::DPadY => {}
            AnalogAxis::LeftTrigger => {
                self.left_trigger = if value < self.deadzone { 0.0 } else { value };

                if self.left_trigger > 0.0 {
                    self.simulate_button_press(Button::LeftTrigger);
                } else {
                    self.simulate_button_release(Button::LeftTrigger);
                }
            }
            AnalogAxis::RightTrigger => {
                self.right_trigger = if value < self.deadzone { 0.0 } else { value };

                if self.right_trigger > 0.0 {
                    self.simulate_button_press(Button::RightTrigger);
                } else {
                    self.simulate_button_release(Button::RightTrigger);
                }
            }
            AnalogAxis::LeftStickX => {
                self.raw_left_stick_x = value;

                (self.left_stick_x, self.left_stick_y) =
                    self.apply_deadzone(self.raw_left_stick_x, self.raw_left_stick_y);

                self.axis_simulate_button(
                    self.left_stick_x,
                    Button::LeftStickLeft,
                    Button::LeftStickRight,
                );
            }
            AnalogAxis::LeftStickY => {
                self.raw_left_stick_y = value;

                (self.left_stick_x, self.left_stick_y) =
                    self.apply_deadzone(self.raw_left_stick_x, self.raw_left_stick_y);

                self.axis_simulate_button(
                    self.left_stick_y,
                    Button::LeftStickDown,
                    Button::LeftStickUp,
                );
            }
            AnalogAxis::RightStickX => {
                self.raw_right_stick_x = value;

                (self.right_stick_x, self.right_stick_y) =
                    self.apply_deadzone(self.raw_right_stick_x, self.raw_right_stick_y);

                self.axis_simulate_button(
                    self.right_stick_x,
                    Button::RightStickLeft,
                    Button::RightStickRight,
                );
            }
            AnalogAxis::RightStickY => {
                self.raw_right_stick_y = value;

                (self.right_stick_x, self.right_stick_y) =
                    self.apply_deadzone(self.raw_right_stick_x, self.raw_right_stick_y);

                self.axis_simulate_button(
                    self.right_stick_y,
                    Button::RightStickDown,
                    Button::RightStickUp,
                );
            }
        }
    }

    fn axis_simulate_button(&mut self, value: f32, low: Button, high: Button) {
        if value < 0.0 {
            self.simulate_button_press(low);
            self.simulate_button_release(high);
        } else if value > 0.0 {
            self.simulate_button_press(high);
            self.simulate_button_release(low);
        } else {
            self.simulate_button_release(low);
            self.simulate_button_release(high);
        }
    }

    fn apply_deadzone(&self, x: f32, y: f32) -> (f32, f32) {
        if self.deadzone == 0.0 {
            return (x, y);
        }

        let v = Vec2::new(x, y);

        let length = v.length().min(1.0);

        if length < self.deadzone {
            return (0.0, 0.0);
        }

        let norm = inverse_lerp!(self.deadzone, 1.0, length) / length;

        (v * norm).into()
    }

    pub fn flush(&mut self) {
        self.previous_buttons = self.pressed_buttons.clone();
        self.latest_button = None;
    }
}
