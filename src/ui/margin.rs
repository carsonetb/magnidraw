use std::cell::RefCell;

use crate::{
    Cursor, Engine, Rect, Size,
    ui::{Element, get_id},
};

/// A margin container, giving the element it contains gaps around the sides.
///
/// For [`crate::ui::UIButton`], [`crate::ui::Radio`], [`crate::ui::TextInput`],
/// and [`crate::ui::Toggle`] this use case is already covered internally.
#[derive(Clone)]
pub struct Margin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub contains: Box<dyn Element>,
    rect: RefCell<Rect>,
    inside: RefCell<Rect>,
    id: u32,
}

impl Margin {
    pub fn new(contains: Box<dyn Element>, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
            contains,
            rect: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
            inside: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
            id: get_id(),
        }
    }
}

impl Element for Margin {
    fn setup(&mut self, engine: &mut Engine) {
        self.contains.setup(engine);
    }

    fn render<'frame, 'app: 'frame>(
        &'app self,
        engine: &mut Engine,
        drawer: &mut crate::Drawer<'frame>,
        z_index: i32,
        rect: crate::Rect,
    ) {
        self.rect.replace(rect);

        let inside = Rect::new(
            rect.pos.x + self.left,
            rect.pos.y + self.top,
            rect.size.w - self.left - self.right,
            rect.size.h - self.top - self.bottom,
        );
        self.inside.replace(inside);
        self.contains.render(engine, drawer, z_index, inside);
    }

    fn update(&mut self, engine: &mut Engine, input: &crate::Input) -> Vec<super::Message> {
        if let Some(pos) = input.mouse_pos()
            && pos.inside(*self.rect.borrow())
            && !pos.inside(*self.inside.borrow())
        {
            engine.set_cursor(Cursor::Default);
        }

        self.contains.update(engine, input)
    }

    fn children(&self) -> Vec<&Box<dyn Element>> {
        vec![&self.contains]
    }

    fn children_mut(&mut self) -> Vec<&mut Box<dyn Element>> {
        vec![&mut self.contains]
    }

    fn min_size(&self) -> Size {
        let contains = self.contains.min_size();
        Size::new(
            self.left + contains.w + self.right,
            self.top + contains.h + self.bottom,
        )
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
