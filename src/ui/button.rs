use std::{cell::RefCell, ops::Deref};

use crate::{
    Button, Color, Cursor, Drawer, Engine, Input, MouseButton, Rect, Size, Text,
    ui::{Element, Message, MessageContent, Theme, get_id},
};

/// Use these to configure how a button looks.
#[derive(Clone, Copy)]
pub struct UIButtonParams {
    pub text_color: Color,
    /// Background color of the button.
    pub color: Color,
    /// Color of the button when the mouse is hovered over it, but not clicking
    /// it.
    pub hover_color: Color,
    /// Color of the button when the mouse is pressing the it.
    pub press_color: Color,
    /// Internal padding of the button, separating the edge of the button from
    /// the text. Ordered left, right, top, bottom.
    pub padding: [f32; 4],
    /// External margin of the button, outside the rectangle. Ordered left,
    /// right, top, bottom.
    pub margin: [f32; 4],
    /// Border radii. Ordered top left, top right, bottom left, bottom right.
    pub radii: [f32; 4],
    pub border_width: f32,
    pub border_color: Color,
}

/// A pressable button.
#[derive(Clone)]
pub struct UIButton {
    /// Optionally, the text displayed in the middle of the button.
    pub text: Option<Text>,
    params: UIButtonParams,
    is_hovered: bool,
    is_pressed: bool,
    rect: RefCell<Rect>,
    id: u32,
}

impl UIButton {
    pub fn new(params: UIButtonParams, text: Option<Text>) -> Self {
        Self {
            text,
            params,
            is_hovered: false,
            is_pressed: false,
            rect: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
            id: get_id(),
        }
    }

    pub fn with_theme(theme: &Theme, text: Option<Text>) -> Self {
        Self::new(theme.buttons, text)
    }
}

impl Element for UIButton {
    fn render<'frame, 'app: 'frame>(
        &'app self,
        _engine: &mut Engine,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        self.rect.replace(rect);

        let color = if self.is_pressed {
            self.params.press_color
        } else if self.is_hovered {
            self.params.hover_color
        } else {
            self.params.color
        };
        drawer.rect_ext(
            z_index,
            Rect::new(
                rect.pos.x + self.params.margin[0],
                rect.pos.y + self.params.margin[2],
                rect.size.w - self.params.margin[1] * 2.0,
                rect.size.h - self.params.margin[3] * 2.0,
            ),
            color,
            self.params.radii[0],
            self.params.radii[1],
            self.params.radii[2],
            self.params.radii[3],
            self.params.border_width,
            self.params.border_color,
        );

        match &self.text {
            Some(text) => {
                let size = text.size();
                drawer.text(
                    z_index,
                    &text,
                    rect.center() - (size / 2.0).as_pos(),
                    self.params.text_color,
                );
            }
            None => (),
        }
    }

    fn update(&mut self, engine: &mut Engine, input: &Input) -> Vec<Message> {
        let mouse_pos = match input.mouse_pos() {
            Some(pos) => pos,
            None => return Vec::new(),
        };

        self.is_hovered = false;
        self.is_pressed = false;

        let mut out = Vec::new();
        if mouse_pos.inside(self.rect.borrow().deref().clone()) {
            engine.set_cursor(Cursor::Pointer);

            if input.button_pressed(Button::Mouse(MouseButton::Left)) {
                self.is_pressed = true;
            } else {
                self.is_hovered = true;
            }

            if input.button_just_pressed(Button::Mouse(MouseButton::Left)) {
                out.push(Message::new(self, MessageContent::ButtonPress));
            }
        }

        out
    }

    fn apply_theme(&mut self, theme: Theme) {
        self.params = theme.buttons;
    }

    fn min_size(&self) -> crate::Size {
        match &self.text {
            Some(text) => {
                text.size()
                    + Size::new(
                        self.params.padding[0]
                            + self.params.padding[1]
                            + self.params.margin[0]
                            + self.params.margin[1],
                        self.params.padding[2]
                            + self.params.padding[3]
                            + self.params.margin[2]
                            + self.params.margin[3],
                    )
            }
            None => Size::new(
                self.params.radii[0]
                    + self.params.radii[1]
                    + self.params.padding[0]
                    + self.params.padding[1]
                    + self.params.margin[0]
                    + self.params.margin[1],
                self.params.radii[2]
                    + self.params.radii[3]
                    + self.params.padding[2]
                    + self.params.padding[3]
                    + self.params.margin[2]
                    + self.params.margin[3],
            ),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
