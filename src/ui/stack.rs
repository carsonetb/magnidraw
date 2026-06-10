use crate::{
    Rect, Size,
    ui::{Direction, Element, get_id},
};

/// Specifies the display behavior of elements in a [`Stack`] at the micro
/// level.
#[derive(Debug, Clone, Copy)]
pub enum StackItemMode {
    /// Displays the element at its minimum width/height.
    Compress,
    /// Expands the element as much as possible within the stack.
    Expand,
}

/// Specifies stacking behavior at a macro level for [`Stack`].
#[derive(Debug, Clone, Copy)]
pub enum StackMode {
    /// Squishes all elements to the left. Items marked as Expand will also be
    /// compressed.
    Left,
    /// Squishes all elements to the center. Items marked as Expand will also be
    /// compressed.
    Center,
    /// Squishes all elements to the right. Items marked as Expand will also be
    /// compressed.
    Right,
    /// Distributes all elements evenly. Items marked as Expand will also be
    /// compressed.
    Distribute,
    /// Delegates to the [`StackItemMode`] of the elements to decide the
    /// layout. If none are marked as Expand, items will be compressed to the
    /// left.
    Delegate,
}

/// Stacks an arbitrary number of elements.
#[derive(Clone)]
pub struct Stack {
    /// The direction in which the elements are stacked.
    pub direction: Direction,
    /// How the elements should be distributed.
    pub mode: StackMode,
    pub elements: Vec<(Box<dyn Element>, StackItemMode)>,
    id: u32,
}

impl Stack {
    pub fn new(
        direction: Direction,
        mode: StackMode,
        elements: Vec<(Box<dyn Element>, StackItemMode)>,
    ) -> Self {
        Self {
            direction,
            mode,
            elements,
            id: get_id(),
        }
    }

    fn direction_of(&self, what: Size) -> f32 {
        match self.direction {
            Direction::Horizontal => what.w,
            Direction::Vertical => what.h,
        }
    }
}

impl Element for Stack {
    fn setup(&mut self, engine: &mut crate::Engine, state: &mut crate::EngineState) {
        for (element, _) in &mut self.elements {
            element.setup(engine, state);
        }
    }

    fn render<'frame, 'app: 'frame>(
        &'app self,
        engine: &mut crate::Engine,
        state: &mut crate::EngineState,
        drawer: &mut crate::Drawer<'frame>,
        z_index: i32,
        rect: crate::Rect,
    ) {
        let mut required = 0.0;
        let mut num_expand = 0;
        for (element, mode) in &self.elements {
            match mode {
                StackItemMode::Compress => required += self.direction_of(element.min_size()),
                StackItemMode::Expand => num_expand += 1,
            }
        }

        'outer: loop {
            for (element, mode) in &self.elements {
                match mode {
                    StackItemMode::Expand => {
                        let expanded_size =
                            (self.direction_of(rect.size) - required) / (num_expand as f32);
                        if expanded_size <= 0.0 {
                            break 'outer;
                        }
                        let min = self.direction_of(element.min_size());
                        if min > expanded_size {
                            required += min;
                            num_expand -= 1;
                            continue 'outer;
                        }
                    }
                    _ => (),
                }
            }
            break;
        }

        let mut offset = 0.0;
        for (element, mode) in &self.elements {
            let size = match mode {
                StackItemMode::Expand => {
                    let expanded_size =
                        (self.direction_of(rect.size) - required) / (num_expand as f32);
                    let min = self.direction_of(element.min_size());
                    if min > expanded_size || expanded_size <= 0.0 {
                        min
                    } else {
                        expanded_size
                    }
                }
                StackItemMode::Compress => self.direction_of(element.min_size()),
            };

            element.render(
                engine,
                state,
                drawer,
                z_index,
                match self.direction {
                    Direction::Horizontal => {
                        Rect::new(rect.pos.x + offset, rect.pos.y, size, rect.size.h)
                    }
                    Direction::Vertical => {
                        Rect::new(rect.pos.x, rect.pos.y + offset, rect.size.w, size)
                    }
                },
            );

            offset += size;
        }
    }

    fn update(
        &mut self,
        engine: &mut crate::Engine,
        state: &mut crate::EngineState,
        input: &crate::Input,
    ) -> Vec<super::Message> {
        let mut messages = Vec::new();
        for (element, _) in &mut self.elements {
            messages.append(&mut element.update(engine, state, input));
        }
        messages
    }

    fn children(&mut self) -> Vec<&Box<dyn Element>> {
        self.elements.iter().map(|(element, _)| element).collect()
    }

    fn min_size(&self) -> crate::Size {
        let mut min = Size::new(0.0, 0.0);

        for (element, _) in &self.elements {
            let this = element.min_size();
            match self.direction {
                Direction::Horizontal => {
                    min.w += this.w;
                    if this.h > min.h {
                        min.h = this.h
                    }
                }
                Direction::Vertical => {
                    min.h += this.h;
                    if this.w > min.w {
                        min.w = this.w
                    }
                }
            }
        }

        min
    }

    fn id(&self) -> u32 {
        self.id
    }
}
