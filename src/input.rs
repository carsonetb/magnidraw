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

/// Representing any specific, valid method of input.
pub enum Button {
    Keyboard(KeyboardButton),
    Mouse(MouseButton),
    Controller(ControllerButton),
    Axis(ControllerAxis),
}

/// One of the controllers which is currently connected to the device.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Controller {
    id: gilrs::GamepadId,
}

/// A button on a specific controller, which may only be pressed or not pressed.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ControllerButton {
    button: gilrs::Button,
    on: Controller,
}

/// Either negative or positive axis direction.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AxisDirection {
    /// Left, up
    Neg = -1,
    /// Right, down
    Pos = 1,
}

/// An axis of a joystick on a specific controller, in any direction.
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

    /// Reverse the direction of this axis.
    pub fn inverse(mut self) -> Self {
        match self.direction {
            AxisDirection::Neg => self.direction = AxisDirection::Pos,
            AxisDirection::Pos => self.direction = AxisDirection::Neg,
        }
        self
    }
}

/// The input server, from which you may check for inputs from specific
/// [`Button`]s.
///
/// The server is recreated every frame, so the lifetime only lasts for a
/// single frame.
pub struct Input<'a> {
    _gilrs: &'a Gilrs,
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

    pub(crate) fn new(
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
            _gilrs: gilrs,
            prev_buttons,
            current_buttons,
            prev_axes,
            current_axes,
            controllers,
            axis_press_threshold,
            deadzone,
        }
    }

    /// Get a list of all the currently connected [`Controller`]s.
    pub fn controllers(&self) -> &[Controller] {
        &self.controllers
    }

    /// Check if a [`Button`] is currently pressed.
    ///
    /// If the button is an axis, we check if the value of the axis is greater
    /// than the axis press threshold. You can set the threshold via
    /// [`Input::set_axis_threshold`].
    pub fn button_pressed(&self, button: Button) -> bool {
        match button {
            Button::Controller(button) => self.current_buttons.contains(&button),
            Button::Axis(axis) => self.current_axis_pressed(axis),
            Button::Keyboard(key) => self.input.key_held(key),
            Button::Mouse(mouse) => self.input.mouse_held(mouse),
        }
    }

    /// Check if a [`Button`] is currently not pressed.
    ///
    /// This function is equal to the inverse of [`Input::button_pressed`].
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

    /// Check if a button wasn't pressed last frame, and now is.
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

    /// Check if a button was pressed last frame, and now isn't.
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

    /// Get the axis of a button.
    ///
    /// For controller, keyboard, and mouse buttons, this can only be 0.0 or
    /// 1.0. For joystick axes, this can be anywhere from 0.0 to 1.0.
    pub fn button_axis(&self, button: Button) -> f32 {
        let out = match button {
            Button::Controller(button) => self.prev_buttons.contains(&button) as i32 as f32,
            Button::Axis(axis) => {
                let raw = self.current_axes.get(&(axis.on, axis.axis)).unwrap_or(&0.0);
                match axis.direction {
                    AxisDirection::Neg => raw.min(0.0).abs(),
                    AxisDirection::Pos => raw.max(0.0),
                }
            }
            Button::Keyboard(key) => self.input.key_held(key) as i32 as f32,
            Button::Mouse(mouse) => self.input.mouse_held(mouse) as i32 as f32,
        };
        out
    }

    /// Get the axis from one button to another. The output of this function
    /// can be anywhere from -1.0 to 1.0. The argument `neg` will be negative,
    /// and the argument `pos` will be positive.
    pub fn axis(&self, neg: Button, pos: Button) -> f32 {
        let out = -self.button_axis(neg) + self.button_axis(pos);
        if out.abs() > self.deadzone { out } else { 0.0 }
    }

    /// Sets the threshold at which a joystick axis is considered pressed.
    pub fn set_axis_threshold(&mut self, value: f32) {
        self.axis_press_threshold = value;
    }

    /// Sets the threshold at which, if a joystick axis is less than the
    /// threshold, it will be set to 0.0, regardless of the actual value.
    pub fn set_deadzone(&mut self, value: f32) {
        self.deadzone = value;
    }

    /// If a key is held, it will begin to repeat when using this function.
    /// This only works on keyboard keys, because why would it work on
    /// controller buttons?
    pub fn ui_key_pressed(&self, keycode: KeyboardButton) -> bool {
        self.input.key_pressed_os(keycode)
    }

    /// Get the time since the previous frame.
    pub fn delta(&self) -> std::time::Duration {
        self.input.delta_time().unwrap_or(std::time::Duration::ZERO)
    }
}
