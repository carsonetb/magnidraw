use magnidraw::{Color, Drawer, EngineState, Game, Rect};

pub struct SpriteDemo {}

impl Game for SpriteDemo {
    fn render<'d, 's: 'd>(
        &'s mut self,
        _engine: &mut magnidraw::Engine,
        _state: &mut EngineState,
        drawer: &mut Drawer<'d>,
    ) {
        // Drawing a rectangle is much like it is done in other rendering
        // libraries!
        for i in 0..40 {
            drawer.rect(
                0,
                Rect::new(
                    100.0 + i as f32 * 10.0,
                    100.0 + i as f32 * 10.0,
                    500.0,
                    600.0,
                ),
                Color::rgba(
                    0.0 + i as f32 * (1.0 / 40.0),
                    (1.0 - i as f32 * (1.0 / 2.0)).abs(),
                    (1.0 - i as f32 * (1.0 / 2.0)).abs(),
                    0.01,
                ),
            );
        }
        drawer.rect_ext(
            1,
            Rect::new(100.0, 100.0, 500.0, 600.0),
            Color::rgb(1.0, 1.0, 1.0),
            20.0,
            40.0,
            50.0,
            400.0,
        );
    }
}

fn main() {
    magnidraw::run(Box::new(SpriteDemo {}));
}
