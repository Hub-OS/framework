use math::*;
use std::time::Duration;
pub use strum::{EnumString, IntoStaticStr};

const STICK_DEADZONE: f32 = 0.1;
const STICK_BUTTON_DEADZONE: f32 = 0.35;

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
}

impl GameController {
    pub fn new(id: usize, rumble_pack: Box<dyn RumblePack>) -> Self {
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
                self.left_trigger = if value < STICK_DEADZONE { 0.0 } else { value };

                if self.left_trigger > 0.0 {
                    self.simulate_button_press(Button::LeftTrigger);
                } else {
                    self.simulate_button_release(Button::LeftTrigger);
                }
            }
            AnalogAxis::RightTrigger => {
                self.right_trigger = if value < STICK_DEADZONE { 0.0 } else { value };

                if self.right_trigger > 0.0 {
                    self.simulate_button_press(Button::RightTrigger);
                } else {
                    self.simulate_button_release(Button::RightTrigger);
                }
            }
            AnalogAxis::LeftStickX => {
                self.raw_left_stick_x = value;
            }
            AnalogAxis::LeftStickY => {
                self.raw_left_stick_y = value;
            }
            AnalogAxis::RightStickX => {
                self.raw_right_stick_x = value;
            }
            AnalogAxis::RightStickY => {
                self.raw_right_stick_y = value;
            }
        }
    }

    pub fn update_sticks(&mut self) {
        // left stick
        (self.left_stick_x, self.left_stick_y) =
            self.apply_deadzone(STICK_DEADZONE, self.raw_left_stick_x, self.raw_left_stick_y);

        let (button_x, button_y) = self.apply_deadzone(
            STICK_BUTTON_DEADZONE,
            self.raw_left_stick_x,
            self.raw_left_stick_y,
        );

        self.axis_simulate_button(
            button_x,
            button_y,
            Button::LeftStickLeft,
            Button::LeftStickRight,
        );

        self.axis_simulate_button(
            button_y,
            button_x,
            Button::LeftStickDown,
            Button::LeftStickUp,
        );

        // right stick
        (self.right_stick_x, self.right_stick_y) = self.apply_deadzone(
            STICK_DEADZONE,
            self.raw_right_stick_x,
            self.raw_right_stick_y,
        );

        let (button_x, button_y) = self.apply_deadzone(
            STICK_BUTTON_DEADZONE,
            self.raw_right_stick_x,
            self.raw_right_stick_y,
        );

        self.axis_simulate_button(
            button_x,
            button_y,
            Button::RightStickLeft,
            Button::RightStickRight,
        );

        self.axis_simulate_button(
            button_y,
            button_x,
            Button::RightStickDown,
            Button::RightStickUp,
        );
    }

    fn axis_simulate_button(&mut self, value: f32, other: f32, low: Button, high: Button) {
        // a = 22.5 / 180 * Math.PI; Math.cos(a) / Math.sin(a)
        const RATIO_THRESHOLD: f32 = 2.414;

        if value == 0.0 || (other / value).abs() > RATIO_THRESHOLD {
            self.simulate_button_release(low);
            self.simulate_button_release(high);
            return;
        }

        if value < 0.0 {
            self.simulate_button_press(low);
            self.simulate_button_release(high);
        } else if value > 0.0 {
            self.simulate_button_press(high);
            self.simulate_button_release(low);
        }
    }

    fn apply_deadzone(&self, deadzone: f32, x: f32, y: f32) -> (f32, f32) {
        if deadzone == 0.0 {
            return (x, y);
        }

        let v = Vec2::new(x, y);

        let length = v.length().min(1.0);

        if length < deadzone {
            return (0.0, 0.0);
        }

        let norm = inverse_lerp!(deadzone, 1.0, length) / length;

        (v * norm).into()
    }

    pub fn flush(&mut self) {
        self.previous_buttons.clone_from(&self.pressed_buttons);
        self.latest_button = None;
    }
}
