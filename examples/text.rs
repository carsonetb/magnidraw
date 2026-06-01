use magnidraw::{Color, Drawer, Engine, EngineState, Game, Pos, Text};

pub struct TextDemo {
    text: Option<Text>,
}

impl Game for TextDemo {
    fn setup(&mut self, engine: &mut Engine, _state: &mut EngineState) {
        self.text = Some(engine.load_text("Hello, World!", 50.0));
    }

    fn render<'d, 's: 'd>(&'s mut self, drawer: &mut Drawer<'d>) {
        drawer.text(
            0,
            self.text.as_ref().unwrap(),
            Pos::new(10.0, 10.0),
            Color::WHITE,
        );
    }
}

fn main() {
    magnidraw::run(Box::new(TextDemo { text: None }));
}
