use std::collections::{HashMap, HashSet};

use gilrs::Gilrs;
use winit_input_helper::WinitInputHelper;

use crate::{KeyboardButton, MouseButton};

#[macro_export]
macro_rules! __keycode {
    ($name:ident) => {
        Button::Keyboard(KeyboardButton::$name)
    };
}

pub use __keycode as keycode;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ControllerButton {
    button: gilrs::Button,
    id: gilrs::GamepadId,
}

pub enum Button {
    Controller(ControllerButton),
    Keyboard(KeyboardButton),
    Mouse(MouseButton),
}

pub struct Input<'a> {
    input: &'a WinitInputHelper,
    previous: HashSet<ControllerButton>,
    current: HashSet<ControllerButton>,
}

impl<'a> Input<'a> {
    const BUTTONS: [gilrs::Button; 19] = [
        gilrs::Button::South,
        gilrs::Button::East,
        gilrs::Button::North,
        gilrs::Button::West,
        gilrs::Button::C,
        gilrs::Button::Z,
        gilrs::Button::LeftTrigger,
        gilrs::Button::LeftTrigger2,
        gilrs::Button::RightTrigger,
        gilrs::Button::RightTrigger2,
        gilrs::Button::Select,
        gilrs::Button::Start,
        gilrs::Button::Mode,
        gilrs::Button::LeftThumb,
        gilrs::Button::RightThumb,
        gilrs::Button::DPadUp,
        gilrs::Button::DPadDown,
        gilrs::Button::DPadLeft,
        gilrs::Button::DPadRight,
    ];

    pub fn new(
        input: &'a WinitInputHelper,
        gilrs: &mut Gilrs,
        previous: HashSet<ControllerButton>,
    ) -> Self {
        let mut current = HashSet::new();

        while let Some(gilrs::Event { .. }) = gilrs.next_event() {}

        for (id, gamepad) in gilrs.gamepads() {
            for button in Self::BUTTONS {
                if gamepad.is_pressed(button) {
                    current.insert(ControllerButton { button, id });
                }
            }
        }

        Self {
            input,
            previous,
            current,
        }
    }
}

impl<'a> Input<'a> {
    pub fn button_pressed(&self, button: Button) -> bool {
        match button {
            Button::Controller(button) => self.current.contains(&button),
            Button::Keyboard(key) => self.input.key_held(key),
            Button::Mouse(mouse) => self.input.mouse_held(mouse),
        }
    }

    pub fn button_released(&self, button: Button) -> bool {
        !self.button_pressed(button)
    }

    pub fn button_just_presed(&self, button: Button) -> bool {
        match button {
            Button::Controller(button) => {
                !self.previous.contains(&button) && self.current.contains(&button)
            }
            Button::Keyboard(key) => self.input.key_pressed(key),
            Button::Mouse(mouse) => self.input.mouse_pressed(mouse),
        }
    }

    pub fn button_just_released(&self, button: Button) -> bool {
        match button {
            Button::Controller(button) => {
                self.previous.contains(&button) && !self.current.contains(&button)
            }
            Button::Keyboard(key) => self.input.key_released(key),
            Button::Mouse(mouse) => self.input.mouse_released(mouse),
        }
    }

    /// If a key is held, it will begin to repeat when using this function.
    /// This only works on keyboard keys, because why would it work on
    /// controller buttons?
    pub fn ui_key_pressed(&self, keycode: KeyboardButton) -> bool {
        self.input.key_pressed_os(keycode)
    }

    pub fn delta(&self) -> std::time::Duration {
        self.input.delta_time().unwrap_or(std::time::Duration::ZERO)
    }
}
