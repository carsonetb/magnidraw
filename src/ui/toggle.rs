use std::cell::RefCell;

use crate::{
    Button, Color, Cursor, MouseButton, Pos, Rect, Scale, Size, Sprite, Text,
    ui::{Element, Message, MessageContent, get_id},
};

#[derive(Debug, Clone, Copy)]
pub struct ToggleParams {
    pub text_color: Color,
    pub toggle_on_color: Color,
    pub toggle_off_color: Color,
    pub toggle_on: Sprite,
    pub toggle_off: Sprite,
}

#[derive(Clone)]
pub struct Toggle {
    pub text: Text,
    pub state: bool,
    params: ToggleParams,
    rect: RefCell<Rect>,
    id: u32,
}

impl Toggle {
    pub fn new(params: ToggleParams, text: Text, default: bool) -> Self {
        Self {
            text,
            state: default,
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

        drawer.sprite(z_index, &sprite, rect.pos, Scale::ONE, color);

        drawer.text(
            z_index,
            &self.text,
            rect.pos + Pos::new(sprite.size.w, rect.size.h / 2.0 - self.text.size().h / 2.0),
            self.params.text_color,
        );

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

            if pos.inside(Rect::new_basic(rect.pos, sprite.size)) {
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
        Size::new(on.w.max(off.w) + text.w, on.h.max(off.h).max(text.h))
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
