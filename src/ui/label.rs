use std::{cell::RefCell, ops::Deref};

use crate::{
    Color, Cursor, Drawer, Engine, EngineState, Input, Rect, Size, Text, TextAlign,
    ui::{Element, Message, get_id},
};

pub struct Label {
    pub text: Text,
    pub color: Color,
    pub align: TextAlign,
    rect: RefCell<Rect>,
    id: u32,
}

impl Label {
    pub fn new(text: Text, align: TextAlign, color: Color) -> Self {
        Self {
            text,
            color,
            rect: RefCell::new(Rect::new(200.0, 200.0, 200.0, 200.0)),
            align,
            id: get_id(),
        }
    }
}

impl Element for Label {
    fn render<'frame, 'app: 'frame>(
        &'app self,
        _engine: &mut Engine,
        _state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        self.rect.replace(rect);

        drawer.text(z_index, &self.text, rect.pos, self.color);
    }

    fn update(
        &mut self,
        engine: &mut Engine,
        state: &mut EngineState,
        input: &Input,
    ) -> Vec<Message> {
        let rect = *self.rect.borrow().deref();
        self.text.line_length = Some(rect.size.w);
        self.text.align(self.align);
        engine.reload_text(&mut self.text);

        if let Some(pos) = input.mouse_pos()
            && pos.inside(rect)
        {
            state.set_cursor(Cursor::Default);
        }

        Vec::new()
    }

    fn children(&mut self) -> Vec<&Box<dyn Element>> {
        Vec::new()
    }

    fn min_size(&self) -> Size {
        self.text.size()
    }

    fn id(&self) -> u32 {
        self.id
    }
}
