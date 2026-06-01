use magnidraw::{Color, Drawer, Engine, EngineState, Game, Pos, Text};
use rand::seq::IndexedRandom;

pub struct TextDemo {
    text: Option<Text>,
}

impl Game for TextDemo {
    fn setup(&mut self, engine: &mut Engine, _state: &mut EngineState) {
        // We load the font here. The font's family name is the same as the one
        // used in CSS.
        engine.load_font(include_bytes!("CourierPrime-Regular.ttf"));
        // Here, we create some text. Text can be created at any time in a
        // program (where you have access to an Engine), but it is usually most
        // efficient to create one at the beginning of the program. You should
        // definetely avoid creating one every frame.
        //
        // You can change the text without recreating the object by using
        // Engine::reload_text.
        self.text = Some(engine.load_text("Hello, World!", 50.0, Some("Courier Prime")));
    }

    fn render<'d, 's: 'd>(
        &'s mut self,
        engine: &mut Engine,
        _state: &mut EngineState,
        drawer: &mut Drawer<'d>,
    ) {
        // Here we show an example of changing the text. We always append a
        // random number from 1..9 to the end of the string.
        self.text.as_mut().unwrap().text = format!(
            "Hello World!{}",
            (1..9).collect::<Vec<_>>().choose(&mut rand::rng()).unwrap()
        );
        // Reloading the text updates all its public properties.
        engine.reload_text(&mut self.text.as_mut().unwrap());
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
