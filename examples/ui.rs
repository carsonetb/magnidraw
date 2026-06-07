use magnidraw::{
    Color, Engine, EngineState, Game, Rect,
    ui::{Container, Message, MessageContent, UIButton, UIButtonParams},
};

// Make sure to run this example with --example ui.
pub struct UIDemo {
    container: Option<Container>,
}

impl UIDemo {
    fn new() -> Self {
        Self { container: None }
    }
}

impl Game for UIDemo {
    fn setup(&mut self, engine: &mut Engine, _state: &mut EngineState) {
        let button = Box::new(UIButton::new(UIButtonParams {
            text: Some(engine.load_text("Button Text", 32.0, None)),
            text_color: Color::BLACK,
            color: Color::WHITE,
            hover_color: Color::rgb(0.7, 0.6, 0.5),
            press_color: Color::rgb(0.5, 0.5, 0.5),
        }));

        self.container = Some(Container::new(
            Rect::new(10.0, 10.0, 200.0, 300.0),
            0,
            "Rect".to_string(),
            button,
        ));
    }

    fn messages(&mut self, _engine: &mut Engine, _state: &mut EngineState, messages: Vec<Message>) {
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
