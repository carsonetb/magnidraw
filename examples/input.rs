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
        // Drawing is simple, for explanations see the rect example.
        drawer.rect(
            0,
            Rect::new(self.character.x, self.character.y, 20.0, 20.0),
            Color::WHITE,
        );
    }

    fn update(&mut self, _engine: &mut magnidraw::Engine, _state: &mut EngineState, input: &Input) {
        // We get the time since the last frame so that no matter the framerate,
        // the character always moves at the same speed. This is a common
        // pattern in game development.
        let delta = input.delta().as_secs_f32();
        // We can use this to, for example, print the framerate.
        println!("{}", 1.0 / delta);
        // Check for keypresses, and move the character accordingly.
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
        // Get a list of controllers, so that if the user has a controller
        // plugged in, they can control the player with that.
        let controllers = input.controllers();
        if !controllers.is_empty() {
            // Only use the first controller, if this was a multiplayer game
            // you could use multiple controllers.
            let controller = controllers[0];
            // The controller axes that we care about.
            let horiz = ControllerAxis::new(controller, AnyAxis::LeftStickX, AxisDirection::Neg);
            let vert = ControllerAxis::new(controller, AnyAxis::LeftStickY, AxisDirection::Neg);
            // This is a fairly unorthodox movement system. Horizontal movement
            // is controlled by the value of the joystick, but vertical
            // movement is binary, controlled by whether the joystick's value
            // is above a certain threshold. This is just to detail different
            // capabilities of controller input.
            self.character.x +=
                100.0 * input.axis(Button::Axis(horiz), Button::Axis(horiz.inverse())) * delta;
            if input.button_pressed(Button::Axis(vert)) {
                self.character.y += 100.0 * delta;
            }
            // Inversing a controller axis changes the direction it checks for,
            // from negative to positive or from positive to negative.
            if input.button_pressed(Button::Axis(vert.inverse())) {
                self.character.y -= 100.0 * delta;
            }
        }
    }
}

fn main() {
    magnidraw::run(Box::new(InputDemo::new()));
}
