use magnidraw::{
    Engine, EngineState, Game, Rect,
    ui::{Container, Message, MessageContent, Theme, UIButton},
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
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState) {
        // Create the theme for our UI.
        let theme = Theme::nord();
        engine.apply_theme(state, theme);

        // Create the button with all its parameters.
        let button = Box::new(UIButton::with_theme(
            theme,
            Some(engine.load_text("Button Text", 32.0, Some(theme.font))),
        ));

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
