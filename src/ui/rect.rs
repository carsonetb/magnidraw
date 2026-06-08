use crate::{
    Color, Drawer, Engine, EngineState, Rect, Size,
    ui::{Element, get_id},
};

/// A simple colored UI rect.
pub struct UIRect {
    pub color: Color,
    id: u32,
}

impl UIRect {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            id: get_id(),
        }
    }
}

impl Element for UIRect {
    fn render<'frame, 'app: 'frame>(
        &self,
        _engine: &mut Engine,
        _state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        drawer.rect(z_index, rect, self.color);
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn children(&mut self) -> Vec<&Box<dyn Element>> {
        Vec::new()
    }

    fn min_size(&self) -> Size {
        Size::new(0.0, 0.0)
    }
}
