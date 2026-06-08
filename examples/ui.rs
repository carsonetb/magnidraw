use magnidraw::{
    Color, Engine, EngineState, Game, Input, Pos, Rect, TextAlign,
    ui::{
        Container, Label, Message, MessageContent, SepDirection, Separator, Theme, UIButton, UIRect,
    },
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

        let rect = Box::new(UIRect::new(Color::BLACK));

        let label = Box::new(Label::new(
            engine.load_text(
                "Someone should write an essay here: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean bibendum sem a nisi eleifend, at malesuada lectus tristique. Nulla pharetra auctor magna eget rutrum. Donec non malesuada odio, ac convallis turpis. Proin in orci sodales, molestie leo eget, malesuada nibh. Vivamus eu risus sollicitudin, rhoncus lectus vitae, facilisis sapien. Suspendisse quam orci, tincidunt eget semper sed, blandit nec sapien. Integer consectetur venenatis metus euismod cursus. Aliquam eu velit in diam placerat ornare. ",
                20.0,
                Some(theme.font),
            ),
            TextAlign::Center,
            Color::WHITE,
        ));

        let bottom = Box::new(Separator::new(
            theme.separators,
            SepDirection::Horizontal,
            0.3,
            Some(rect),
            Some(label),
        ));

        let stack = Box::new(Separator::new(
            theme.separators,
            magnidraw::ui::SepDirection::Vertical,
            0.8,
            Some(bottom),
            Some(button),
        ));

        // Create the container which is a rectangle that contains the button
        // element.
        self.container = Some(Container::new(
            Rect::new_basic(Pos::new(0.0, 0.0), state.window_size()),
            0,
            "UI".to_string(),
            stack,
        ));
    }

    fn update(&mut self, _engine: &mut Engine, state: &mut EngineState, _input: &Input) {
        self.container.as_mut().unwrap().rect =
            Rect::new_basic(Pos::new(0.0, 0.0), state.window_size())
    }

    fn messages(&mut self, _engine: &mut Engine, _state: &mut EngineState, messages: Vec<Message>) {
        // Process all the messages to check if the button was pressed.
        let container = self.container.as_mut().unwrap();
        for message in messages {
            if message.from == container.element.children()[0].id() {
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
