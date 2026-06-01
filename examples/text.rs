use magnidraw::{Color, Drawer, Engine, EngineState, Game, Pos, Text};
use rand::seq::IndexedRandom;

pub struct TextDemo {
    text: Option<Text>,
}

impl Game for TextDemo {
    fn setup(&mut self, engine: &mut Engine, _state: &mut EngineState) {
        // Here, we create some text. Text can be created at any time in a
        // program (where you have access to an Engine), but it is usually most
        // efficient to create one at the beginning of the program. You should
        // definetely avoid creating one every frame.
        //
        // You can change the text without recreating the object by using
        // Engine::change_text.
        self.text = Some(engine.load_text("Hello, World!", 50.0));
    }

    fn render<'d, 's: 'd>(&'s mut self, engine: &mut Engine, drawer: &mut Drawer<'d>) {
        // Here we show an example of changing the text. We always append a
        // random number from 1..9 to the end of the string.
        engine.change_text(
            &mut self.text.as_mut().unwrap(),
            &format!(
                "Hello World!{}",
                (1..9).collect::<Vec<_>>().choose(&mut rand::rng()).unwrap()
            ),
        );
        // Drawing the text is relatively simple, we provide it to the drawer.
        // The lifetimes let us give the drawer a reference to the text,
        // because the borrow checker knows that the text (owned by self) will
        // last longer than the drawer.
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
