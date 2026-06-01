mod drawers;
mod engine;
mod math;
mod sprite;
mod text;

pub use drawers::Drawer;
pub use engine::{Engine, EngineState};
pub use math::*;
pub use sprite::Sprite;
pub use text::Text;

use crate::engine::EngineHolder;

/// The core trait that programs that use `magnidraw` must implement.
pub trait Game {
    /// Runs at the beginning of the game. This is where you should setup your
    /// sprites and text, although they *can* be setup elsewhere (particularly,
    /// in the render function).
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState);
    /// The engine sends a [`Drawer`] to the game, to which the game can pass
    /// anything. Counter-semantically, this function does not do any rendering
    /// itself. The rendering happens later, when the `drawer` resolves to a
    /// bunch of [`keydraw::Command`]s, which are used for rendering.
    ///
    /// The `render` function uses lifetimes, which tell the borrow checker
    /// that `self` will definetely outlive `drawer`. This makes sense, because
    /// internally the Drawer object is destroyed every frame, while `self`
    /// lasts for the entire program.
    fn render<'d, 's: 'd>(&'s mut self, engine: &mut Engine, drawer: &mut Drawer<'d>);
}

/// Takes control of the thread and runs a [`Game`].
pub fn run(game: Box<dyn Game>) {
    keydraw::run(Box::new(EngineHolder::new(game))).unwrap();
}
