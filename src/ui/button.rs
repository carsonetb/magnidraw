use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{
    Button, Color, Drawer, Engine, EngineState, Input, MouseButton, Rect, Size, Text,
    ui::{Element, Message, MessageContent, get_id},
};

pub struct UIButtonParams {
    pub text: Option<Text>,
    pub text_color: Color,
    pub color: Color,
    pub hover_color: Color,
    pub press_color: Color,
}

pub struct UIButton {
    params: UIButtonParams,
    is_hovered: bool,
    is_pressed: bool,
    rect: RefCell<Rect>,
    id: u32,
}

impl UIButton {
    pub fn new(params: UIButtonParams) -> Self {
        Self {
            params,
            is_hovered: false,
            is_pressed: false,
            rect: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
            id: get_id(),
        }
    }
}

impl Element for UIButton {
    fn render<'frame, 'app: 'frame>(
        &'app self,
        _engine: &mut Engine,
        _state: &mut EngineState,
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
        drawer.rect(z_index, rect, color);

        match &self.params.text {
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

    fn update(
        &mut self,
        _engine: &mut Engine,
        _state: &mut EngineState,
        input: &Input,
    ) -> Vec<Message> {
        let mouse_pos = match input.mouse_pos() {
            Some(pos) => pos,
            None => return Vec::new(),
        };

        self.is_hovered = false;
        self.is_pressed = false;

        let mut out = Vec::new();
        if mouse_pos.inside(self.rect.borrow().deref().clone()) {
            if input.button_pressed(Button::Mouse(MouseButton::Left)) {
                self.is_pressed = true;
            } else {
                self.is_hovered = true;
            }

            if input.button_just_presed(Button::Mouse(MouseButton::Left)) {
                out.push(Message::new(self, MessageContent::ButtonPress));
            }
        }

        out
    }

    fn children(&mut self) -> Vec<Rc<RefCell<dyn Element>>> {
        Vec::new()
    }

    fn min_size(&self) -> crate::Size {
        Size::new(0.0, 0.0)
    }

    fn id(&self) -> u32 {
        self.id
    }
}
