use std::cell::RefCell;

use winit::event::MouseButton;

use crate::{
    Button, Color, Cursor, Drawer, Engine, EngineState, Input, Rect, Size,
    ui::{Direction, Element, Message, get_id},
};

#[derive(Clone, Copy)]
pub struct SeparatorParams {
    pub width: f32,
    pub color: Color,
}

/// Separates two elements by some factor. Optionally, the separation line can
/// be made draggable so the user can modify this factor.
#[derive(Clone)]
pub struct Separator {
    params: SeparatorParams,
    /// If this stacks vertically or horizontally.
    pub direction: Direction,
    /// From 0.0-1.0, the percentage of the separation line from the left/top
    /// side to the right/bottom side.
    pub factor: f32,
    pub first: Option<Box<dyn Element>>,
    pub second: Option<Box<dyn Element>>,
    pub draggable: bool,
    rect: RefCell<Rect>,
    sep_rect: Rect,
    dragged: bool,
    id: u32,
}

impl Separator {
    pub fn new(
        params: SeparatorParams,
        direction: Direction,
        factor: f32,
        first: Option<Box<dyn Element>>,
        second: Option<Box<dyn Element>>,
        draggable: bool,
    ) -> Self {
        Self {
            params,
            direction,
            factor,
            first,
            second,
            draggable,
            rect: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
            sep_rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            dragged: false,
            id: get_id(),
        }
    }
}

impl Element for Separator {
    fn setup(&mut self, engine: &mut Engine, state: &mut EngineState) {
        if let Some(first) = &mut self.first {
            first.setup(engine, state);
        }

        if let Some(second) = &mut self.second {
            second.setup(engine, state);
        }
    }

    fn render<'frame, 'app: 'frame>(
        &'app self,
        engine: &mut Engine,
        state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        let first_rect = match self.direction {
            Direction::Horizontal => Rect::new(
                rect.pos.x,
                rect.pos.y,
                rect.size.w * self.factor - self.params.width / 2.0,
                rect.size.h,
            ),
            Direction::Vertical => Rect::new(
                rect.pos.x,
                rect.pos.y,
                rect.size.w,
                rect.size.h * self.factor - self.params.width / 2.0,
            ),
        };

        let second_rect = match self.direction {
            Direction::Horizontal => Rect::new(
                rect.pos.x + rect.size.w * self.factor + self.params.width / 2.0,
                rect.pos.y,
                rect.size.w * (1.0 - self.factor) - self.params.width,
                rect.size.h,
            ),
            Direction::Vertical => Rect::new(
                rect.pos.x,
                rect.pos.y + rect.size.h * self.factor + self.params.width / 2.0,
                rect.size.w,
                rect.size.h * (1.0 - self.factor) - self.params.width,
            ),
        };

        match &self.first {
            Some(element) => element.render(engine, state, drawer, z_index, first_rect),
            None => (),
        }

        match &self.second {
            Some(element) => {
                element.render(engine, state, drawer, z_index, second_rect);
            }
            None => (),
        }

        drawer.rect(z_index, self.sep_rect, self.params.color);

        self.rect.replace(rect);
    }

    fn update(
        &mut self,
        engine: &mut Engine,
        state: &mut EngineState,
        input: &Input,
    ) -> Vec<Message> {
        let mut out = Vec::new();

        if let Some(first) = &mut self.first {
            out.append(&mut first.update(engine, state, input));
        }

        if let Some(second) = &mut self.second {
            out.append(&mut second.update(engine, state, input));
        }

        let rect = self.rect.borrow();
        self.sep_rect = match self.direction {
            Direction::Horizontal => Rect::new(
                rect.pos.x + rect.size.w * self.factor - self.params.width / 2.0,
                rect.pos.y,
                self.params.width,
                rect.size.h,
            ),
            Direction::Vertical => Rect::new(
                rect.pos.x,
                rect.pos.y + rect.size.h * self.factor - self.params.width / 2.0,
                rect.size.w,
                self.params.width,
            ),
        };

        if self.draggable
            && let Some(pos) = input.mouse_pos()
            && pos.inside(self.sep_rect)
        {
            match self.direction {
                Direction::Horizontal => state.set_cursor(Cursor::EwResize),
                Direction::Vertical => state.set_cursor(Cursor::NsResize),
            }

            if input.button_just_pressed(Button::Mouse(MouseButton::Left)) {
                self.dragged = true;
            }
        }

        if input.button_released(Button::Mouse(MouseButton::Left)) {
            self.dragged = false;
        }

        if self.dragged
            && input.button_pressed(Button::Mouse(MouseButton::Left))
            && let Some(pos) = input.mouse_pos()
        {
            let old = self.factor;
            self.factor = match self.direction {
                Direction::Horizontal => {
                    let left = rect.pos.x;
                    let right = rect.pos.x + rect.size.w;
                    (pos.x - left) / (right - left)
                }
                Direction::Vertical => {
                    let top = rect.pos.y;
                    let bottom = rect.pos.y + rect.size.h;
                    (pos.y - top) / (bottom - top)
                }
            };

            let (first_max, second_max) = match self.direction {
                Direction::Horizontal => {
                    (rect.size.w * self.factor, rect.size.w * (1.0 - self.factor))
                }
                Direction::Vertical => {
                    (rect.size.h * self.factor, rect.size.h * (1.0 - self.factor))
                }
            };

            let first_min = self
                .first
                .as_ref()
                .map_or(Size::new(0.0, 0.0), |f| f.min_size());
            let second_min = self
                .second
                .as_ref()
                .map_or(Size::new(0.0, 0.0), |s| s.min_size());

            if match self.direction {
                Direction::Horizontal => first_min.w > first_max || second_min.w > second_max,
                Direction::Vertical => first_min.h > first_max || second_min.h > second_max,
            } {
                self.factor = old;
            }
        }

        out
    }

    fn children(&self) -> Vec<&Box<dyn Element>> {
        let mut out = Vec::new();

        if let Some(first) = &self.first {
            out.push(first);
        }

        if let Some(second) = &self.second {
            out.push(second);
        }

        out
    }

    fn children_mut(&mut self) -> Vec<&mut Box<dyn Element>> {
        let mut out = Vec::new();

        if let Some(first) = &mut self.first {
            out.push(first);
        }

        if let Some(second) = &mut self.second {
            out.push(second);
        }

        out
    }

    fn min_size(&self) -> Size {
        let first = self
            .first
            .as_ref()
            .map_or(Size::new(0.0, 0.0), |f| f.min_size());
        let second = self
            .second
            .as_ref()
            .map_or(Size::new(0.0, 0.0), |s| s.min_size());
        match self.direction {
            // TODO: This is wrong
            Direction::Horizontal => Size::new(
                (first.w + first.w / self.factor * (1.0 - self.factor))
                    .max(second.w + second.w / (1.0 - self.factor) * self.factor),
                first.h.max(second.h),
            ),
            Direction::Vertical => Size::new(
                first.w.max(second.w),
                (first.h + first.h / self.factor * (1.0 - self.factor))
                    .max(second.h + second.h / (1.0 - self.factor) * self.factor),
            ),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
