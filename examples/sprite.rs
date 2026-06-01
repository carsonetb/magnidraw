use magnidraw::{Color, Drawer, Engine, EngineState, Game, Pos, Scale, Sprite};

pub struct SpriteDemo {
    tux: Option<Sprite>,
}

impl Game for SpriteDemo {
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState) {
        self.tux = Some(engine.load_sprite(state, include_bytes!("tux.png")));
    }

    fn render<'d, 's: 'd>(&'s mut self, drawer: &mut Drawer<'d>) {
        drawer.sprite(
            0,
            self.tux.as_ref().unwrap(),
            Pos::new(10.0, 10.0),
            Scale::ONE,
            Color::WHITE,
        );
    }
}

fn main() {
    magnidraw::run(Box::new(SpriteDemo { tux: None }));
}
