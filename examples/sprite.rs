use magnidraw::{Color, Drawer, Engine, EngineState, Game, Pos, Scale, Sprite};

pub struct SpriteDemo {
    tux: Option<Sprite>,
}

impl Game for SpriteDemo {
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState) {
        // Load an image. Like with text, you should avoid loading images every
        // frame, it just *feels* inefficient. The load_sprite function
        // requires access to engine state because it creates and modifies
        // uniform buffers.
        //
        // Unfortunately, due to this limitation, it's not currently possible
        // to load images dynamically in the render function. This is a
        // limitation with the lower-level KeyDraw library and will hopefully
        // be fixed in the future.
        self.tux = Some(engine.load_sprite(state, include_bytes!("tux.png")));
        // We can change the background color to make tux stand out a little
        // more.
        engine.set_clear_color(state, Color::rgb(0.8, 0.7, 0.6));
    }

    fn render<'d, 's: 'd>(&'s mut self, _engine: &mut Engine, drawer: &mut Drawer<'d>) {
        // Drawing the sprite is relatively simple.
        drawer.sprite(
            0,
            self.tux.as_ref().unwrap(),
            Pos::new(10.0, 10.0),
            Scale::ONE,
            // Draw a pink tux!
            Color::rgb(1.0, 0.0, 0.5),
        );
    }
}

fn main() {
    magnidraw::run(Box::new(SpriteDemo { tux: None }));
}
