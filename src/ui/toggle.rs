use std::cell::RefCell;

use crate::{
    Button, Color, Cursor, MouseButton, Pos, Rect, Scale, Size, Sprite, Text,
    ui::{Element, Message, MessageContent, get_id},
};

/// Params for [`Toggle`].
#[derive(Debug, Clone, Copy)]
pub struct ToggleParams {
    pub text_color: Color,
    /// Color of the toggled on image.
    pub toggle_on_color: Color,
    /// Color of the toggled off image.
    pub toggle_off_color: Color,
    pub toggle_on: Sprite,
    pub toggle_off: Sprite,
    /// External margin of the toggle, outside the rectangle. Ordered left,
    /// right, top, bottom.
    pub margin: [f32; 4],
    /// Space between the toggle icon and the text.
    pub icon_text_padding: f32,
}

/// A type of button which can be toggled between on and off.
#[derive(Clone)]
pub struct Toggle {
    pub text: Text,
    /// Whether the toggle is toggled on (true) or toggled off (false).
    pub state: bool,
    /// Whether the toggle is on the right side of the text (true) or on the
    /// left side (false).
    pub flipped: bool,
    params: ToggleParams,
    rect: RefCell<Rect>,
    id: u32,
}

impl Toggle {
    pub fn new(params: ToggleParams, text: Text, default: bool, flipped: bool) -> Self {
        Self {
            text,
            state: default,
            flipped,
            params,
            rect: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
            id: get_id(),
        }
    }
}

impl Element for Toggle {
    fn render<'frame, 'app: 'frame>(
        &'app self,
        _engine: &mut crate::Engine,
        _state: &mut crate::EngineState,
        drawer: &mut crate::Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        let (sprite, color) = if self.state {
            (self.params.toggle_on, self.params.toggle_on_color)
        } else {
            (self.params.toggle_off, self.params.toggle_off_color)
        };

        let (sprite_pos, text_pos) = if self.flipped {
            (
                rect.pos
                    + Pos::new(
                        self.params.margin[0] + self.text.size().w + self.params.icon_text_padding,
                        self.params.margin[2],
                    ),
                rect.pos + Pos::new(self.params.margin[0], self.params.margin[2]),
            )
        } else {
            (
                rect.pos + Pos::new(self.params.margin[0], self.params.margin[2]),
                rect.pos
                    + Pos::new(
                        self.params.margin[0] + sprite.size.w + self.params.icon_text_padding,
                        rect.size.h / 2.0 - self.text.size().h / 2.0,
                    ),
            )
        };

        drawer.sprite(z_index, &sprite, sprite_pos, Scale::ONE, color);
        drawer.text(z_index, &self.text, text_pos, self.params.text_color);

        self.rect.replace(rect);
    }

    fn update(
        &mut self,
        _engine: &mut crate::Engine,
        state: &mut crate::EngineState,
        input: &crate::Input,
    ) -> Vec<Message> {
        let rect = *self.rect.borrow();

        if let Some(pos) = input.mouse_pos()
            && pos.inside(rect)
        {
            let sprite = if self.state {
                self.params.toggle_on
            } else {
                self.params.toggle_off
            };

            let toggle_rect = if self.flipped {
                Rect::new(
                    rect.pos.x
                        + self.params.margin[0]
                        + self.text.size().w
                        + self.params.icon_text_padding,
                    rect.pos.y + self.params.margin[2],
                    sprite.size.w,
                    sprite.size.h,
                )
            } else {
                Rect::new(
                    rect.pos.x + self.params.margin[0],
                    rect.pos.y + self.params.margin[2],
                    sprite.size.w,
                    sprite.size.h,
                )
            };

            if pos.inside(toggle_rect) {
                state.set_cursor(Cursor::Pointer);

                if input.button_just_pressed(Button::Mouse(MouseButton::Left)) {
                    self.state = !self.state;

                    return vec![if self.state {
                        Message::new(self, MessageContent::ToggleOn)
                    } else {
                        Message::new(self, MessageContent::ToggleOn)
                    }];
                }
            } else {
                state.set_cursor(Cursor::Default);
            }
        }

        Vec::new()
    }

    fn children(&self) -> Vec<&Box<dyn Element>> {
        Vec::new()
    }

    fn children_mut(&mut self) -> Vec<&mut Box<dyn Element>> {
        Vec::new()
    }

    fn min_size(&self) -> Size {
        let on = self.params.toggle_on.size;
        let off = self.params.toggle_off.size;
        let text = self.text.size();
        Size::new(
            on.w.max(off.w)
                + text.w
                + self.params.margin[0]
                + self.params.margin[1]
                + self.params.icon_text_padding,
            on.h.max(off.h).max(text.h) + self.params.margin[2] + self.params.margin[3],
        )
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
