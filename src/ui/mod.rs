use std::{
    any::Any,
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::{Drawer, Engine, EngineState, Input, Rect, Size};

mod button;
mod rect;

pub use button::*;
pub use rect::*;

static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn get_id() -> u32 {
    let out = ID_COUNTER.load(Ordering::Relaxed);
    ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    out
}

pub trait Element {
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState) {
        let _ = (engine, state);
    }

    fn render<'frame, 'app: 'frame>(
        &'app self,
        engine: &mut Engine,
        state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        let _ = (engine, state, drawer, z_index, rect);
    }

    fn update(
        &mut self,
        engine: &mut Engine,
        state: &mut EngineState,
        input: &Input,
    ) -> Vec<Message> {
        let _ = (engine, state, input);
        Vec::new()
    }

    fn children(&mut self) -> Vec<Rc<RefCell<dyn Element>>>;

    fn min_size(&self) -> Size;

    fn id(&self) -> u32;
}

pub struct Message {
    pub from: u32,
    pub content: MessageContent,
}

impl Message {
    pub fn new(from: &dyn Element, content: MessageContent) -> Self {
        Self {
            from: from.id(),
            content,
        }
    }
}

pub enum MessageContent {
    ButtonPress,
    Other(Box<dyn Any>),
}

pub struct Container {
    pub rect: Rect,
    pub z_index: i32,
    pub name: String,
    pub element: Box<dyn Element>,
}

impl Container {
    pub fn new(rect: Rect, z_index: i32, name: String, element: Box<dyn Element>) -> Self {
        Self {
            rect,
            z_index,
            name,
            element,
        }
    }

    pub(crate) fn render<'frame, 'app: 'frame>(
        &'app self,
        engine: &mut Engine,
        state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
    ) {
        let min = self.element.min_size();
        if self.rect.size.w < min.w || self.rect.size.h < min.h {
            println!(
                "Cannot render container '{}' properly. Its width or height is smaller than the minimum size of the element it contains.",
                self.name
            );
            return;
        }

        self.element
            .render(engine, state, drawer, self.z_index, self.rect);
    }

    pub(crate) fn update(
        &mut self,
        engine: &mut Engine,
        state: &mut EngineState,
        input: &Input,
    ) -> Vec<Message> {
        self.element.update(engine, state, input)
    }
}
