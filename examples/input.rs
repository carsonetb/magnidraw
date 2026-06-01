use magnidraw::{Color, Drawer, EngineState, Game, Input, KeyCode, Pos, Rect};

pub struct InputDemo {
    character: Pos,
}

impl InputDemo {
    fn new() -> Self {
        Self {
            character: Pos::new(10.0, 10.0),
        }
    }
}

impl Game for InputDemo {
    fn render<'d, 's: 'd>(
        &'s mut self,
        _engine: &mut magnidraw::Engine,
        _state: &mut EngineState,
        drawer: &mut Drawer<'d>,
    ) {
        drawer.rect(
            0,
            Rect::new(self.character.x, self.character.y, 20.0, 20.0),
            Color::WHITE,
        );
    }

    fn update(&mut self, _engine: &mut magnidraw::Engine, _state: &mut EngineState, input: &Input) {
        let delta = input.delta().as_secs_f32();
        println!("{}", 1.0 / delta);
        if input.key_pressed(KeyCode::KeyA) {
            self.character.x -= 100.0 * delta;
        }
        if input.key_pressed(KeyCode::KeyD) {
            self.character.x += 100.0 * delta;
        }
        if input.key_pressed(KeyCode::KeyW) {
            self.character.y -= 100.0 * delta;
        }
        if input.key_pressed(KeyCode::KeyS) {
            self.character.y += 100.0 * delta;
        }
    }
}

fn main() {
    magnidraw::run(Box::new(InputDemo::new()));
}
