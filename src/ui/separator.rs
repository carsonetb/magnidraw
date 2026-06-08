use crate::{
    Color, Drawer, Engine, EngineState, Rect, Size,
    ui::{Element, get_id},
};

pub enum SepDirection {
    Horizontal,
    Vertical,
}

pub struct SeparatorParams {
    pub width: f32,
    pub color: Color,
}

pub struct Separator {
    params: SeparatorParams,
    pub direction: SepDirection,
    pub factor: f32,
    pub first: Option<Box<dyn Element>>,
    pub second: Option<Box<dyn Element>>,
    id: u32,
}

impl Separator {
    pub fn new(
        params: SeparatorParams,
        direction: SepDirection,
        first: Option<Box<dyn Element>>,
        second: Option<Box<dyn Element>>,
    ) -> Self {
        Self {
            params,
            direction,
            factor: 0.5,
            first,
            second,
            id: get_id(),
        }
    }
}

impl Element for Separator {
    fn render<'frame, 'app: 'frame>(
        &'app self,
        engine: &mut Engine,
        state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        let first_rect = match self.direction {
            SepDirection::Horizontal => Rect::new(
                rect.pos.x,
                rect.pos.y,
                rect.size.w * self.factor,
                rect.size.h,
            ),
            SepDirection::Vertical => Rect::new(
                rect.pos.x,
                rect.pos.y,
                rect.size.w,
                rect.size.h * self.factor,
            ),
        };

        let second_rect = match self.direction {
            SepDirection::Horizontal => Rect::new(
                rect.pos.x + rect.size.w * self.factor,
                rect.pos.y,
                rect.size.w * (1.0 - self.factor),
                rect.size.h,
            ),
            SepDirection::Vertical => Rect::new(
                rect.pos.x,
                rect.pos.y + rect.size.h * self.factor,
                rect.size.w,
                rect.size.h * (1.0 - self.factor),
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

        let rect = match self.direction {
            SepDirection::Horizontal => Rect::new(
                rect.pos.x + rect.size.w * self.factor - self.params.width / 2.0,
                rect.pos.y,
                rect.size.w,
                self.params.width,
            ),
            SepDirection::Vertical => Rect::new(
                rect.pos.x,
                rect.pos.y + rect.size.h * self.factor - self.params.width / 2.0,
                self.params.width,
                rect.size.h,
            ),
        };

        drawer.rect(z_index, rect, self.params.color);
    }

    fn children(&mut self) -> Vec<&Box<dyn Element>> {
        let mut out = Vec::new();

        if let Some(first) = &self.first {
            out.push(first);
        }

        if let Some(second) = &self.second {
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
            SepDirection::Horizontal => Size::new(first.w + second.w, first.h.max(second.h)),
            SepDirection::Vertical => Size::new(first.w.max(second.w), first.h + second.h),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
