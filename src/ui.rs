use std::{cell::RefCell, rc::Rc};

use crate::{Color, Drawer, Engine, EngineState, Input, Rect, Size};

pub trait Element {
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState) {
        let _ = (engine, state);
    }

    fn render<'frame>(
        &self,
        engine: &mut Engine,
        state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        let _ = (engine, state, drawer, z_index, rect);
    }

    fn update(&mut self, engine: &mut Engine, state: &mut EngineState, input: &Input) {
        let _ = (engine, state, input);
    }

    fn children(&mut self) -> Vec<Rc<RefCell<dyn Element>>>;

    fn min_size(&self) -> Size;
}

pub struct Container {
    pub rect: Rect,
    pub z_index: i32,
    pub name: String,
    pub element: Rc<RefCell<dyn Element>>,
}

impl Container {
    pub fn new(rect: Rect, z_index: i32, name: String, element: Rc<RefCell<dyn Element>>) -> Self {
        Self {
            rect,
            z_index,
            name,
            element,
        }
    }

    pub(crate) fn render<'frame>(
        &self,
        engine: &mut Engine,
        state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
    ) {
        let element = self.element.borrow();
        let min = element.min_size();
        if self.rect.size.w < min.w || self.rect.size.h < min.h {
            println!(
                "Cannot render container '{}' properly. Its width or height is smaller than the minimum size of the element it contains.",
                self.name
            );
            return;
        }

        element.render(engine, state, drawer, self.z_index, self.rect);
    }

    pub(crate) fn update(&mut self, engine: &mut Engine, state: &mut EngineState, input: &Input) {
        self.element.borrow_mut().update(engine, state, input);
    }
}

pub struct UIRect {
    pub color: Color,
}

impl UIRect {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Element for UIRect {
    fn render<'frame>(
        &self,
        _engine: &mut Engine,
        _state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        drawer.rect(z_index, rect, self.color);
    }

    fn children(&mut self) -> Vec<Rc<RefCell<dyn Element>>> {
        Vec::new()
    }

    fn min_size(&self) -> Size {
        Size::new(0.0, 0.0)
    }
}
