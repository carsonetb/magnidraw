use magnidraw::{
    Color, Engine, EngineState, Game, Input, Pos, Rect, TextAlign,
    ui::{
        Container, Direction, Element, Label, Message, MessageContent, Radio, Separator, Stack,
        StackItemMode, StackMode, TextInput, Theme, Toggle, UIButton, UIRect, element_child,
    },
};

// Make sure to run this example with --example ui.

pub struct UIDemo {
    // Container is optional because we need to set it up in the setup function.
    container: Option<Container>,
    input_id: u32,
    text_id: u32,
}

impl UIDemo {
    fn new() -> Self {
        Self {
            container: None,
            input_id: 0,
            text_id: 0,
        }
    }
}

impl Game for UIDemo {
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState) {
        // Create the theme for our UI.
        let theme = Theme::catppuccin_latte(engine, state);
        engine.apply_theme(state, &theme);

        // Create the button with all its parameters.
        let text = engine.load_text(
            "Button Text",
            32.0,
            Some(theme.font),
            TextAlign::Center,
            None,
        );
        let button = Box::new(UIButton::with_theme(&theme, Some(text)));

        let rect = Box::new(UIRect::new(Color::BLACK));

        let mut stack: Vec<(Box<dyn Element>, StackItemMode)> = vec![(rect, StackItemMode::Expand)];
        for _ in 0..3 {
            stack.push((button.clone(), StackItemMode::Compress));
        }
        let input = Box::new(TextInput::new(
            engine.load_text("Hello!", 32.0, Some(theme.font), TextAlign::Left, None),
            theme.text_inputs,
            Some(engine.load_text("Something", 32.0, Some(theme.font), TextAlign::Left, None)),
        ));
        self.input_id = input.id();
        stack.push((input, StackItemMode::Compress));
        stack.push((
            Box::new(Toggle::new(
                theme.toggles,
                engine.load_text("Toggle me!", 28.0, Some(theme.font), TextAlign::Left, None),
                true,
                true,
            )),
            StackItemMode::Compress,
        ));
        stack.push((
            Box::new(Radio::new(
                theme.radios,
                vec![
                    engine.load_text("Radio me?", 28.0, Some(theme.font), TextAlign::Left, None),
                    engine.load_text("Or me ...", 28.0, Some(theme.font), TextAlign::Left, None),
                ],
                1,
            )),
            StackItemMode::Compress,
        ));
        let stack = Box::new(Stack::new(Direction::Vertical, StackMode::Delegate, stack));

        let label = Box::new(Label::new(
            engine.load_text(
                "Someone should write an essay here: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean bibendum sem a nisi eleifend, at malesuada lectus tristique. Nulla pharetra auctor magna eget rutrum. Donec non malesuada odio, ac convallis turpis. \n\nProin in orci sodales, molestie leo eget, malesuada nibh. Vivamus eu risus sollicitudin, rhoncus lectus vitae, facilisis sapien. Suspendisse quam orci, tincidunt eget semper sed, blandit nec sapien. Integer consectetur venenatis metus euismod cursus. Aliquam eu velit in diam placerat ornare. ",
                20.0,
                Some(theme.font), TextAlign::Center, None
            ),
            TextAlign::Right,
            theme.labels,
        ));
        self.text_id = label.id();

        let bottom = Box::new(Separator::new(
            theme.separators,
            Direction::Horizontal,
            0.3,
            Some(stack),
            Some(label),
            true,
        ));

        let sum = Box::new(Separator::new(
            theme.separators,
            Direction::Vertical,
            0.8,
            Some(bottom),
            Some(button),
            true,
        ));

        // Create the container which is a rectangle that contains the button
        // element.
        self.container = Some(Container::new(
            Rect::new_basic(Pos::new(0.0, 0.0), state.window_size()),
            0,
            "UI".to_string(),
            sum,
        ));
    }

    fn update(&mut self, _engine: &mut Engine, state: &mut EngineState, _input: &Input) {
        self.container.as_mut().unwrap().rect =
            Rect::new_basic(Pos::new(0.0, 0.0), state.window_size());
        state.set_window_min_size(self.container.as_ref().unwrap().element.min_size());
    }

    fn messages(&mut self, _engine: &mut Engine, _state: &mut EngineState, messages: Vec<Message>) {
        // Process all the messages to check if the button was pressed.
        let container = self.container.as_mut().unwrap();
        for message in messages {
            if message.from == self.input_id {
                match message.content {
                    MessageContent::TextInputSubmit(text) => {
                        let input =
                            element_child::<Label>(&mut container.element, self.text_id).unwrap();
                        input.text.text = input.text.text.clone() + &text;
                    }
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
