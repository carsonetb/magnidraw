use std::cell::RefCell;

use crate::{
    Color, Cursor, Drawer, Engine, Rect, Size,
    ui::{Element, get_id},
};

/// A simple colored UI rect.
#[derive(Clone)]
pub struct UIRect {
    pub color: Color,
    rect: RefCell<Rect>,
    id: u32,
}

impl UIRect {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            id: get_id(),
            rect: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
        }
    }
}

impl Element for UIRect {
    fn render<'frame, 'app: 'frame>(
        &self,
        _engine: &mut Engine,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        self.rect.replace(rect);
        drawer.rect(z_index, rect, self.color);
    }

    fn update(&mut self, engine: &mut Engine, input: &crate::Input) -> Vec<super::Message> {
        if let Some(pos) = input.mouse_pos()
            && pos.inside(*self.rect.borrow())
        {
            engine.set_cursor(Cursor::Default);
        }

        Vec::new()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn children(&self) -> Vec<&Box<dyn Element>> {
        Vec::new()
    }

    fn children_mut(&mut self) -> Vec<&mut Box<dyn Element>> {
        Vec::new()
    }

    fn min_size(&self) -> Size {
        Size::new(0.0, 0.0)
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
