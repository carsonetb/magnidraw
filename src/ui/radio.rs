use std::cell::RefCell;

use crate::{
    Button, Color, Cursor, MouseButton, Pos, Rect, Scale, Size, Sprite, Text,
    ui::{Element, get_id},
};

/// Params for [`Radio`].
#[derive(Debug, Clone, Copy)]
pub struct RadioParams {
    pub text_color: Color,
    /// Color of the radio when it is toggled on.
    pub on_color: Color,
    /// Color of the radio when it is toggled off.
    pub off_color: Color,
    pub on_sprite: Sprite,
    pub off_sprite: Sprite,
    /// External margin of the element, outside the rectangle. Ordered left,
    /// right, top, bottom.
    pub margin: [f32; 4],
    /// Space between the toggle icon and the text.
    pub icon_text_padding: f32,
}

/// A row of radio buttons where only one can be selected at a time.
#[derive(Clone)]
pub struct Radio {
    /// All the options in this column.
    pub options: Vec<Text>,
    /// The index of the option which is selected.
    pub selected: usize,
    params: RadioParams,
    rect: RefCell<Rect>,
    id: u32,
}

impl Radio {
    pub fn new(params: RadioParams, options: Vec<Text>, selected: usize) -> Self {
        Self {
            options,
            selected,
            params,
            rect: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
            id: get_id(),
        }
    }
}

impl Element for Radio {
    fn render<'frame, 'app: 'frame>(
        &'app self,
        _engine: &mut crate::Engine,
        _state: &mut crate::EngineState,
        drawer: &mut crate::Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        for (i, option) in self.options.iter().enumerate() {
            let text_size = option.size();
            let on_size = self.params.on_sprite.size;
            let off_size = self.params.off_sprite.size;
            let indie_height = on_size.h.max(off_size.h).max(text_size.h);
            let height = indie_height * (i as f32);

            let (sprite, color) = if i == self.selected {
                (self.params.on_sprite, self.params.on_color)
            } else {
                (self.params.off_sprite, self.params.off_color)
            };

            drawer.sprite(
                z_index,
                &sprite,
                Pos::new(
                    rect.pos.x + self.params.margin[0],
                    rect.pos.y + self.params.margin[2] + height,
                ),
                Scale::ONE,
                color,
            );
            drawer.text(
                z_index,
                option,
                Pos::new(
                    rect.pos.x
                        + self.params.margin[0]
                        + sprite.size.w
                        + self.params.icon_text_padding,
                    rect.pos.y + self.params.margin[0] + height + indie_height / 2.0
                        - text_size.h / 2.0,
                ),
                self.params.text_color,
            );
        }

        self.rect.replace(rect);
    }

    fn update(
        &mut self,
        _engine: &mut crate::Engine,
        state: &mut crate::EngineState,
        input: &crate::Input,
    ) -> Vec<super::Message> {
        let rect = *self.rect.borrow();

        if let Some(pos) = input.mouse_pos()
            && pos.inside(rect)
        {
            let mut inside = false;
            for (i, text) in self.options.iter().enumerate() {
                let text_size = text.size();
                let on_size = self.params.on_sprite.size;
                let off_size = self.params.off_sprite.size;
                let indie_height = on_size.h.max(off_size.h).max(text_size.h);
                let height = indie_height * (i as f32);

                let sprite = if i == self.selected {
                    self.params.on_sprite
                } else {
                    self.params.off_sprite
                };

                let rect = Rect::new(
                    rect.pos.x + self.params.margin[0],
                    rect.pos.y + self.params.margin[2] + height,
                    sprite.size.w + self.params.icon_text_padding + text_size.w,
                    sprite.size.h.max(text_size.h),
                );

                if pos.inside(rect) {
                    inside = true;

                    if input.button_just_pressed(Button::Mouse(MouseButton::Left)) {
                        self.selected = i;
                    }
                }
            }

            if inside {
                state.set_cursor(Cursor::Pointer);
            } else {
                state.set_cursor(Cursor::Default);
            }
        }

        Vec::new()
    }

    fn children(&self) -> Vec<&Box<dyn Element>> {
        Vec::new()
    }

    fn children_mut(&mut self) -> Vec<&mut Box<dyn Element>> {
        Vec::new()
    }

    fn min_size(&self) -> Size {
        let mut out = Size::new(0.0, 0.0);

        out.h += self.params.margin[2];
        for option in &self.options {
            let text_size = option.size();
            let on_size = self.params.on_sprite.size;
            let off_size = self.params.off_sprite.size;
            out.h += on_size.h.max(off_size.h).max(text_size.h);

            out.w = out.w.max(
                self.params.margin[0]
                    + on_size.w.max(off_size.w)
                    + self.params.icon_text_padding
                    + text_size.w
                    + self.params.margin[1],
            );
        }
        out.h += self.params.margin[3];

        out
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
