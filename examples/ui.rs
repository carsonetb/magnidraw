use std::{cell::RefCell, rc::Rc};

use magnidraw::{
    Color, Game, Rect,
    ui::{Container, UIRect},
};

// Make sure to run this example with --example ui.
pub struct UIDemo {
    container: Container,
}

impl UIDemo {
    fn new() -> Self {
        Self {
            container: Container::new(
                Rect::new(10.0, 10.0, 200.0, 300.0),
                0,
                "Rect".to_string(),
                Rc::new(RefCell::new(UIRect::new(Color::WHITE))),
            ),
        }
    }
}

impl Game for UIDemo {
    fn containers(&self) -> Vec<&Container> {
        vec![&self.container]
    }

    fn containers_mut(&mut self) -> Vec<&mut Container> {
        vec![&mut self.container]
    }
}

fn main() {
    magnidraw::run(Box::new(UIDemo::new()));
}
