use std::{
    any::Any,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::{Drawer, Engine, EngineState, Input, Rect, Size};

mod button;
mod rect;
mod separator;
mod theme;

pub use button::*;
pub use rect::*;
pub use separator::*;
pub use theme::*;

static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Create a new, incremental ID for UI elements.
pub fn get_id() -> u32 {
    let out = ID_COUNTER.load(Ordering::Relaxed);
    ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    out
}

/// Elements are the core of the UI system. They are contained by either a
/// [`Container`] or another Element. They have a set of child Elements, which
/// it is responsible for drawing.
pub trait Element {
    /// Similar to the function [`crate::Game::setup`], this function is called
    /// when the Element is first registered.
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState) {
        let _ = (engine, state);
    }

    /// Similar to the function [`crate::Game::render`], this function is
    /// called when the element should render. `self` is immutable, so if for
    /// some reason you need to transfer data out of the `render` function,
    /// you'll probably have to use a [`std::cell::RefCell`].
    ///
    /// Here the Element must call the render function on its children.
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

    /// Similar to the function [`crate::Game::update`], the Element can capture
    /// input and store it for use later in the render function.
    ///
    /// You also may return some [`Message`]s which will be accumulated so
    /// the [`crate::Game`] can process them.
    fn update(
        &mut self,
        engine: &mut Engine,
        state: &mut EngineState,
        input: &Input,
    ) -> Vec<Message> {
        let _ = (engine, state, input);
        Vec::new()
    }

    /// Set the theme of this Element, propogating to all child Elements.
    fn apply_theme(&mut self, theme: Theme) {
        let _ = theme;
    }

    /// All the children of this Element. An Element may have any number of
    /// children.
    fn children(&mut self) -> Vec<&Box<dyn Element>>;

    /// Minimum size of this Element. The minimum size of children should be
    /// taken account if they are present.
    fn min_size(&self) -> Size;

    /// ID of this element, for sending [`Message`]s. You can use the [`get_id`]
    /// function to easily make one.
    fn id(&self) -> u32;
}

/// A Message which is passed up from an [`Element`] to be processed by the
/// [`crate::Game`].
pub struct Message {
    /// ID of the [`Element`] which sent the Message.
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

/// The action performed, or other message. Regrettably, it seems the only way
/// to pass custom messages in an extensible way is to use [`std::any::Any`].
/// Alternatively, you may store data in the [`Element`] itself, and query it
/// later directly.
pub enum MessageContent {
    ButtonPress,
    Other(Box<dyn Any>),
}

/// The top level of a UI tree.
pub struct Container {
    /// Bounds of this UI panel.
    pub rect: Rect,
    /// Z-index where all these [`Element`]s will be drawn. Note that it is not
    /// disallowed or even discouraged for Elements to use a z-index which is
    /// more or less than this.
    pub z_index: i32,
    /// Name of the container, for error handling.
    pub name: String,
    /// The top level element. This element is owned by the Container, for
    /// lifetime reasons, but the Container can be owned by the Game, so it's
    /// not a big deal.
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

    /// Set the theme of all [`Element`]s in this Container.
    pub fn apply_theme(&mut self, theme: Theme) {
        self.element.apply_theme(theme);
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
