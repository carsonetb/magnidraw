use std::collections::{HashMap, HashSet};

use gilrs::Gilrs;
use winit_input_helper::WinitInputHelper;

use crate::{AnyAxis, KeyboardButton, MouseButton};

#[macro_export]
macro_rules! __keycode {
    ($name:ident) => {
        magnidraw::Button::Keyboard(magnidraw::KeyboardButton::$name)
    };
}

pub use __keycode as keycode;

pub enum Button {
    Keyboard(KeyboardButton),
    Mouse(MouseButton),
    Controller(ControllerButton),
    Axis(ControllerAxis),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Controller {
    id: gilrs::GamepadId,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ControllerButton {
    button: gilrs::Button,
    on: Controller,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AxisDirection {
    Neg = -1,
    Pos = 1,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ControllerAxis {
    axis: AnyAxis,
    direction: AxisDirection,
    on: Controller,
}

impl ControllerAxis {
    pub fn new(on: Controller, axis: AnyAxis, direction: AxisDirection) -> Self {
        Self {
            axis,
            direction,
            on,
        }
    }

    pub fn inverse(mut self) -> Self {
        match self.direction {
            AxisDirection::Neg => self.direction = AxisDirection::Pos,
            AxisDirection::Pos => self.direction = AxisDirection::Neg,
        }
        self
    }
}

pub struct Input<'a> {
    gilrs: &'a Gilrs,
    input: &'a WinitInputHelper,
    prev_buttons: HashSet<ControllerButton>,
    current_buttons: HashSet<ControllerButton>,
    prev_axes: HashMap<(Controller, AnyAxis), f32>,
    current_axes: HashMap<(Controller, AnyAxis), f32>,
    controllers: Vec<Controller>,
    axis_press_threshold: f32,
    deadzone: f32,
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

    const AXES: [gilrs::Axis; 8] = [
        gilrs::Axis::LeftStickX,
        gilrs::Axis::LeftStickY,
        gilrs::Axis::LeftZ,
        gilrs::Axis::RightStickX,
        gilrs::Axis::RightStickY,
        gilrs::Axis::RightZ,
        gilrs::Axis::DPadX,
        gilrs::Axis::DPadY,
    ];

    pub fn new(
        input: &'a WinitInputHelper,
        gilrs: &'a mut Gilrs,
        prev_buttons: HashSet<ControllerButton>,
        prev_axes: HashMap<(Controller, AnyAxis), f32>,
        axis_press_threshold: f32,
        deadzone: f32,
    ) -> Self {
        while let Some(gilrs::Event { .. }) = gilrs.next_event() {}

        let mut controllers = Vec::new();
        let mut current_buttons = HashSet::new();
        for (id, gamepad) in gilrs.gamepads() {
            let controller = Controller { id };
            controllers.push(controller);
            for button in Self::BUTTONS {
                if gamepad.is_pressed(button) {
                    current_buttons.insert(ControllerButton {
                        button,
                        on: controller,
                    });
                }
            }
        }

        let mut current_axes = HashMap::new();

        for (id, gamepad) in gilrs.gamepads() {
            for axis in Self::AXES {
                current_axes.insert((Controller { id }, axis), gamepad.value(axis));
            }
        }

        Self {
            input,
            gilrs,
            prev_buttons,
            current_buttons,
            prev_axes,
            current_axes,
            controllers,
            axis_press_threshold,
            deadzone,
        }
    }
}

impl<'a> Input<'a> {
    pub fn controllers(&self) -> &[Controller] {
        &self.controllers
    }

    pub fn button_pressed(&self, button: Button) -> bool {
        match button {
            Button::Controller(button) => self.current_buttons.contains(&button),
            Button::Axis(axis) => self.current_axis_pressed(axis),
            Button::Keyboard(key) => self.input.key_held(key),
            Button::Mouse(mouse) => self.input.mouse_held(mouse),
        }
    }

    pub fn button_released(&self, button: Button) -> bool {
        !self.button_pressed(button)
    }

    fn prev_axis_pressed(&self, axis: ControllerAxis) -> bool {
        let prev = self.prev_axes.get(&(axis.on, axis.axis)).unwrap_or(&0.0);
        let thresh = &(self.axis_press_threshold * (axis.direction as i32 as f32));
        prev.abs() > thresh.abs() && prev.signum() == thresh.signum()
    }

    fn current_axis_pressed(&self, axis: ControllerAxis) -> bool {
        let prev = self.current_axes.get(&(axis.on, axis.axis)).unwrap_or(&0.0);
        let thresh = &(self.axis_press_threshold * (axis.direction as i32 as f32));
        prev.abs() > thresh.abs() && prev.signum() == thresh.signum()
    }

    pub fn button_just_presed(&self, button: Button) -> bool {
        match button {
            Button::Controller(button) => {
                !self.prev_buttons.contains(&button) && self.current_buttons.contains(&button)
            }
            Button::Axis(axis) => !self.prev_axis_pressed(axis) && self.current_axis_pressed(axis),
            Button::Keyboard(key) => self.input.key_pressed(key),
            Button::Mouse(mouse) => self.input.mouse_pressed(mouse),
        }
    }

    pub fn button_just_released(&self, button: Button) -> bool {
        match button {
            Button::Controller(button) => {
                self.prev_buttons.contains(&button) && !self.current_buttons.contains(&button)
            }
            Button::Axis(axis) => self.prev_axis_pressed(axis) && !self.current_axis_pressed(axis),
            Button::Keyboard(key) => self.input.key_released(key),
            Button::Mouse(mouse) => self.input.mouse_released(mouse),
        }
    }

    pub fn button_axis(&self, button: Button) -> f32 {
        let out = match button {
            Button::Controller(button) => self.prev_buttons.contains(&button) as i32 as f32,
            Button::Axis(axis) => {
                *self.current_axes.get(&(axis.on, axis.axis)).unwrap_or(&0.0)
                    * (axis.direction as i32 as f32)
            }
            Button::Keyboard(key) => self.input.key_held(key) as i32 as f32,
            Button::Mouse(mouse) => self.input.mouse_held(mouse) as i32 as f32,
        };
        out
    }

    pub fn axis(&self, neg: Button, pos: Button) -> f32 {
        let out = -self.button_axis(neg) + self.button_axis(pos);
        if out.abs() > self.deadzone { out } else { 0.0 }
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
