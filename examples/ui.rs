use magnidraw::{
    Color, Engine, EngineState, Game, Rect,
    ui::{Container, Message, MessageContent, UIButton, UIButtonParams},
};

// Make sure to run this example with --example ui.

pub struct UIDemo {
    // Container is optional because we need to set it up in the setup function.
    container: Option<Container>,
}

impl UIDemo {
    fn new() -> Self {
        Self { container: None }
    }
}

impl Game for UIDemo {
    fn setup(&mut self, engine: &mut Engine, _state: &mut EngineState) {
        // Create the button with all its parameters.
        let button = Box::new(UIButton::new(UIButtonParams {
            text: Some(engine.load_text("Button Text", 32.0, None)),
            text_color: Color::BLACK,
            color: Color::WHITE,
            hover_color: Color::rgb(0.7, 0.6, 0.5),
            press_color: Color::rgb(0.5, 0.5, 0.5),
            radii: [10.0, 10.0, 10.0, 10.0],
            border_width: 2.0,
            border_color: Color::rgb(0.7, 0.7, 0.7),
        }));

        // Create the container which is a rectangle that contains the button
        // element.
        self.container = Some(Container::new(
            Rect::new(10.0, 10.0, 200.0, 300.0),
            0,
            "Rect".to_string(),
            button,
        ));
    }

    fn messages(&mut self, _engine: &mut Engine, _state: &mut EngineState, messages: Vec<Message>) {
        // Process all the messages to check if the button was pressed.
        let container = self.container.as_mut().unwrap();
        for message in messages {
            if message.from == container.element.id() {
                match message.content {
                    MessageContent::ButtonPress => container.rect.pos.x += 50.0,
                    _ => (),
                }
            }
        }
    }

    fn containers(&self) -> Vec<&Container> {
        match &self.container {
            Some(container) => vec![container],
            None => Vec::new(),
        }
    }

    fn containers_mut(&mut self) -> Vec<&mut Container> {
        match &mut self.container {
            Some(container) => vec![container],
            None => Vec::new(),
        }
    }
}

fn main() {
    magnidraw::run(Box::new(UIDemo::new()));
}
