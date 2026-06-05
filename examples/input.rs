use magnidraw::{
    AnyAxis, AxisDirection, Button, Color, ControllerAxis, Drawer, EngineState, Game, Input, Pos,
    Rect, keycode,
};

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
        if input.button_pressed(keycode!(KeyA)) {
            self.character.x -= 100.0 * delta;
        }
        if input.button_pressed(keycode!(KeyD)) {
            self.character.x += 100.0 * delta;
        }
        if input.button_pressed(keycode!(KeyW)) {
            self.character.y -= 100.0 * delta;
        }
        if input.button_pressed(keycode!(KeyS)) {
            self.character.y += 100.0 * delta;
        }
        let controllers = input.controllers();
        if !controllers.is_empty() {
            let controller = controllers[0];
            let horiz = ControllerAxis::new(controller, AnyAxis::LeftStickX, AxisDirection::Neg);
            let vert = ControllerAxis::new(controller, AnyAxis::LeftStickY, AxisDirection::Neg);
            self.character.x +=
                100.0 * input.axis(Button::Axis(horiz), Button::Axis(horiz.inverse())) * delta;
            if input.button_pressed(Button::Axis(vert)) {
                self.character.y += 100.0 * delta;
            }
            if input.button_pressed(Button::Axis(vert.inverse())) {
                self.character.y -= 100.0 * delta;
            }
        }
    }
}

fn main() {
    magnidraw::run(Box::new(InputDemo::new()));
}
