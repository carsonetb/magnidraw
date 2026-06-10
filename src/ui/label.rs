use std::{cell::RefCell, ops::Deref};

use glyphon::{Edit, cosmic_text::Selection};
use winit::event::MouseButton;

use crate::{
    Button, Color, Cursor, Drawer, Engine, EngineState, Input, KeyboardButton, Rect, Size, Text,
    TextAlign,
    ui::{Element, Message, get_id},
};

/// Params for [`Label`].
#[derive(Clone, Copy)]
pub struct LabelParams {
    /// The color of the text.
    pub color: Color,
    /// The color of the selection, or highlight. It should be mostly
    /// transparent.
    pub selection_color: Color,
}

/// A simple text element. The user can select text and copy it.
#[derive(Clone)]
pub struct Label {
    pub params: LabelParams,
    pub text: Text,
    pub align: TextAlign,
    selecting: bool,
    rect: RefCell<Rect>,
    id: u32,
}

impl Label {
    pub fn new(text: Text, align: TextAlign, params: LabelParams) -> Self {
        Self {
            params,
            text,
            rect: RefCell::new(Rect::new(200.0, 200.0, 200.0, 200.0)),
            align,
            selecting: false,
            id: get_id(),
        }
    }
}

impl Element for Label {
    fn render<'frame, 'app: 'frame>(
        &'app self,
        _engine: &mut Engine,
        _state: &mut EngineState,
        drawer: &mut Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        self.rect.replace(rect);

        drawer.text(z_index, &self.text, rect.pos, self.params.color);

        if let Some((start, end)) = self.text.editor.selection_bounds() {
            self.text.editor.with_buffer(|buffer| {
                for run in buffer.layout_runs() {
                    if run.line_i < start.line || run.line_i > end.line {
                        continue;
                    }

                    let start_byte = if run.line_i == start.line {
                        start.index
                    } else {
                        0
                    };

                    let end_byte = if run.line_i == end.line {
                        end.index
                    } else {
                        usize::MAX
                    };

                    let mut min_x: Option<f32> = None;
                    let mut max_x: Option<f32> = None;

                    for glyph in run.glyphs {
                        if glyph.start >= start_byte && glyph.start < end_byte {
                            let left = glyph.x;
                            let right = glyph.x + glyph.w;

                            min_x = Some(min_x.map_or(left, |m| m.min(left)));
                            max_x = Some(max_x.map_or(right, |m| m.max(right)));
                        }
                    }

                    if let Some(left) = min_x
                        && let Some(right) = max_x
                    {
                        let mut width = right - left;

                        // Trailing space.
                        if end_byte > run.glyphs.last().map(|g| g.end).unwrap_or(0) {
                            width += 8.0;
                        }

                        drawer.rect(
                            z_index,
                            Rect::new(
                                rect.pos.x + left,
                                rect.pos.y + run.line_top,
                                width,
                                run.line_height,
                            ),
                            self.params.selection_color,
                        );
                    } else if start_byte == 0 && end_byte == usize::MAX {
                        // Empty line, completely selected.
                        drawer.rect(
                            z_index,
                            Rect::new(rect.pos.x, rect.pos.y + run.line_top, 8.0, run.line_height),
                            self.params.selection_color,
                        );
                    }
                }
            })
        }
    }

    fn update(
        &mut self,
        engine: &mut Engine,
        state: &mut EngineState,
        input: &Input,
    ) -> Vec<Message> {
        let rect = *self.rect.borrow().deref();
        self.text.line_length = Some(rect.size.w);
        self.text.align(self.align);
        engine.reload_text(&mut self.text);

        if let Some(pos) = input.mouse_pos()
            && pos.inside(rect)
        {
            let mut set = false;
            for line in self.text.inner_buffer().layout_runs() {
                if pos.inside(Rect::new(
                    rect.pos.x,
                    rect.pos.y + line.line_top,
                    line.line_w,
                    line.line_height,
                )) {
                    set = true;
                    state.set_cursor(Cursor::Text);
                }
            }

            if self.selecting && input.button_pressed(Button::Mouse(MouseButton::Left)) {
                if let Some(end) = self
                    .text
                    .inner_buffer()
                    .hit(pos.x - rect.pos.x, pos.y - rect.pos.y)
                {
                    self.text.editor.set_selection(Selection::Normal(end));
                }
            } else {
                self.selecting = false;
            }

            if set {
                if input.button_just_pressed(Button::Mouse(MouseButton::Left)) {
                    if let Some(cursor) = self
                        .text
                        .inner_buffer()
                        .hit(pos.x - rect.pos.x, pos.y - rect.pos.y)
                    {
                        self.selecting = true;
                        self.text.editor.set_cursor(cursor);
                        self.text.editor.set_selection(Selection::Normal(cursor));
                    }
                }
            } else {
                state.set_cursor(Cursor::Default);
            }
        }

        if let Some(text) = self.text.editor.copy_selection()
            && (input.button_pressed(Button::Keyboard(KeyboardButton::ControlLeft))
                || input.button_pressed(Button::Keyboard(KeyboardButton::ControlRight)))
            && input.button_just_pressed(Button::Keyboard(KeyboardButton::KeyC))
        {
            engine.copy(text);
        }

        Vec::new()
    }

    fn children(&mut self) -> Vec<&Box<dyn Element>> {
        Vec::new()
    }

    fn min_size(&self) -> Size {
        self.text.size() - Size::new(150.0, 0.0)
    }

    fn id(&self) -> u32 {
        self.id
    }
}
