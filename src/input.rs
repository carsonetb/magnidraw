use winit::{event::MouseButton, keyboard::KeyCode};
use winit_input_helper::WinitInputHelper;

use crate::Pos;

pub struct Input<'a> {
    input: &'a WinitInputHelper,
}

impl<'a> Input<'a> {
    pub fn new(input: &'a WinitInputHelper) -> Self {
        Self { input }
    }
}

impl<'a> Input<'a> {
    pub fn key_just_pressed(&self, keycode: KeyCode) -> bool {
        self.input.key_pressed(keycode)
    }

    /// If a key is held, it will begin to repeat when using this function.
    pub fn ui_key_pressed(&self, keycode: KeyCode) -> bool {
        self.input.key_pressed_os(keycode)
    }

    pub fn key_just_released(&self, keycode: KeyCode) -> bool {
        self.input.key_released(keycode)
    }

    pub fn key_pressed(&self, keycode: KeyCode) -> bool {
        self.input.key_held(keycode)
    }

    pub fn key_released(&self, keycode: KeyCode) -> bool {
        !self.key_pressed(keycode)
    }

    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.input.mouse_pressed(button)
    }

    pub fn mouse_just_released(&self, button: MouseButton) -> bool {
        self.input.mouse_released(button)
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        !self.mouse_pressed(button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.input.mouse_held(button)
    }

    pub fn mouse_pos(&self) -> Option<Pos> {
        let (x, y) = self.input.cursor()?;
        Some(Pos::new(x, y))
    }

    pub fn delta(&self) -> std::time::Duration {
        self.input.delta_time().unwrap_or(std::time::Duration::ZERO)
    }
}
