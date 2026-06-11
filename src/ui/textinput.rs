use std::{any::Any, cell::RefCell, ops::DerefMut};

use glyphon::{
    Action, Edit,
    cosmic_text::{Motion, Selection},
};
use winit::event::MouseButton;

use crate::{
    Button, Color, Cursor, KeyboardButton, Pos, Rect, Size, Text,
    ui::{Element, Message, MessageContent, get_id},
};

/// Params for a [`TextInput`].
#[derive(Debug, Clone, Copy)]
pub struct TextInputParams {
    /// Text box background color.
    pub color: Color,
    pub text_color: Color,
    /// Color of hint text if present.
    pub hint_color: Color,
    /// Color of the highlight when the user drags the mouse.
    pub selection_color: Color,
    pub cursor_color: Color,
    pub cursor_width: f32,
    /// Internal padding of the input, between the rectangle and the text.
    /// Ordered left, right, top, bottom.
    pub padding: [f32; 4],
    /// External margin of the input, outside the rectangle. Ordered left,
    /// right, top, bottom.
    pub margin: [f32; 4],
    /// Text box border radii.
    pub radii: [f32; 4],
    /// Text box border width.
    pub border_width: f32,
    /// Text box border color.
    pub border_color: Color,
}

/// A box in which the user can type text.
#[derive(Clone)]
pub struct TextInput {
    pub params: TextInputParams,
    pub text: Text,
    /// The hint is displayed when `text` is empty.
    pub hint: Option<Text>,
    selecting: bool,
    highlighted: bool,
    rect: RefCell<Rect>,
    id: u32,
}

impl TextInput {
    pub fn new(text: Text, params: TextInputParams, hint: Option<Text>) -> Self {
        Self {
            params,
            text,
            hint,
            selecting: false,
            highlighted: false,
            rect: RefCell::new(Rect::new(0.0, 0.0, 0.0, 0.0)),
            id: get_id(),
        }
    }
}

impl Element for TextInput {
    fn render<'frame, 'app: 'frame>(
        &'app self,
        _engine: &mut crate::Engine,
        _state: &mut crate::EngineState,
        drawer: &mut crate::Drawer<'frame>,
        z_index: i32,
        rect: Rect,
    ) {
        self.rect.replace(rect);

        drawer.rect_ext(
            z_index,
            Rect::new(
                rect.pos.x + self.params.margin[0],
                rect.pos.y + self.params.margin[2],
                rect.size.w - self.params.margin[1] - self.params.margin[0],
                rect.size.h - self.params.margin[3] - self.params.margin[2],
            ),
            self.params.color,
            self.params.radii[0],
            self.params.radii[1],
            self.params.radii[2],
            self.params.radii[3],
            self.params.border_width,
            self.params.border_color,
        );

        let pos = rect.pos
            + Pos::new(
                self.params.margin[0] + self.params.padding[0],
                rect.size.h / 2.0 - self.text.line_height / 2.0,
            );
        if self.text.text.is_empty()
            && let Some(hint) = &self.hint
        {
            drawer.text(z_index, hint, pos, self.params.hint_color);
        } else {
            drawer.text(z_index, &self.text, pos, self.params.text_color);
        }

        // Yeah idk about this but it works
        if let Some((x, _)) = self.text.editor.cursor_position()
            && (rect.pos.x + x as f32)
                < rect.pos.x + rect.size.w
                    - self.params.margin[1]
                    - self.params.margin[0]
                    - self.params.padding[0]
        {
            drawer.rect(
                z_index + 1,
                Rect::new(
                    rect.pos.x + self.params.margin[0] + self.params.padding[0] + x as f32,
                    rect.pos.y + rect.size.h / 2.0 - (self.text.line_height - 10.0) / 2.0,
                    self.params.cursor_width,
                    self.text.line_height - 10.0,
                ),
                self.params.cursor_color,
            );
        }

        if let Some((start, end)) = self.text.editor.selection_bounds() {
            self.text.editor.with_buffer(|buffer| {
                if let Some(run) = buffer.layout_runs().next() {
                    let mut min_x = None;
                    let mut max_x = None;

                    for glyph in run.glyphs {
                        if glyph.start < start.index || glyph.end > end.index {
                            continue;
                        }

                        let left = glyph.x;
                        let right = glyph.x + glyph.w;

                        min_x = Some(min_x.map_or(left, |m: f32| m.min(left)));
                        max_x = Some(max_x.map_or(right, |m: f32| m.max(right)));
                    }

                    if let Some(left) = min_x
                        && let Some(right) = max_x
                    {
                        drawer.rect(
                            z_index + 1,
                            Rect::new(
                                rect.pos.x + left + self.params.margin[0] + self.params.padding[0],
                                rect.pos.y + rect.size.h / 2.0 - self.text.line_height / 2.0,
                                right - left,
                                run.line_height,
                            ),
                            self.params.selection_color,
                        );
                    }
                }
            })
        }
    }

    fn update(
        &mut self,
        engine: &mut crate::Engine,
        state: &mut crate::EngineState,
        input: &crate::Input,
    ) -> Vec<Message> {
        let rect = **&self.rect.borrow();

        let clip = Rect::new(
            rect.pos.x + self.params.margin[0],
            rect.pos.y + self.params.margin[2],
            rect.size.w - self.params.margin[0] - self.params.margin[1],
            rect.size.h - self.params.margin[2] - self.params.margin[3],
        );
        self.text.set_clip(clip);

        if let Some(pos) = input.mouse_pos()
            && pos.inside(rect)
        {
            let mut inside = false;
            if pos.inside(clip) {
                inside = true;
                state.set_cursor(Cursor::Text);
            } else {
                state.set_cursor(Cursor::Default);
            }

            let hit = self.text.inner_buffer().hit(
                pos.x - rect.pos.x - self.params.margin[0] - self.params.padding[0],
                pos.y - rect.pos.y - self.params.margin[1] - self.params.padding[0],
            );

            if inside {
                if input.button_just_pressed(Button::Mouse(MouseButton::Left))
                    && let Some(cursor) = hit
                {
                    self.selecting = true;
                    self.highlighted = true;
                    self.text.editor.set_cursor(cursor);
                    self.text.editor.set_selection(Selection::None);
                }
            }
            if self.selecting && input.button_pressed(Button::Mouse(MouseButton::Left)) {
                if let Some(end) = hit {
                    if end != self.text.editor.cursor() {
                        self.text.editor.set_selection(Selection::Normal(end));
                    } else {
                        self.text.editor.set_selection(Selection::None);
                    }
                }
            } else {
                self.selecting = false;
            }
        }

        let mut reload = false;

        let text = input.text_pressed();
        if !text.is_empty() {
            self.text.editor.insert_string(&text, None);
            reload = true;
        };

        {
            let font_system = engine.get_font_system();
            let mut borrowed = font_system.borrow_mut();
            let control = input.button_pressed(Button::Keyboard(KeyboardButton::ControlLeft))
                || input.button_pressed(Button::Keyboard(KeyboardButton::ControlRight));

            if input.ui_key_pressed(KeyboardButton::ArrowLeft) {
                self.text.editor.action(
                    borrowed.deref_mut(),
                    if control {
                        Action::Motion(Motion::LeftWord)
                    } else {
                        Action::Motion(Motion::Left)
                    },
                );
                self.highlighted = false;
            }

            if input.ui_key_pressed(KeyboardButton::ArrowRight) {
                self.text.editor.action(
                    borrowed.deref_mut(),
                    if control {
                        Action::Motion(Motion::RightWord)
                    } else {
                        Action::Motion(Motion::Right)
                    },
                );
                self.highlighted = false;
            }

            if input.ui_key_pressed(KeyboardButton::Backspace) {
                self.text
                    .editor
                    .action(borrowed.deref_mut(), Action::Backspace);
                reload = true;
            }
        }

        if reload {
            self.text.text = self.text.editor_text();
            engine.reload_text(&mut self.text);
        }

        if !self.highlighted {
            self.text.editor.set_selection(Selection::None);
        }

        let mut out = Vec::new();

        if input.button_just_pressed(Button::Keyboard(KeyboardButton::Enter)) {
            out.push(Message::new(
                self,
                MessageContent::TextInputSubmit(self.text.text.clone()),
            ));
        }

        out
    }

    fn children(&self) -> Vec<&Box<dyn Element>> {
        Vec::new()
    }

    fn children_mut(&mut self) -> Vec<&mut Box<dyn Element>> {
        Vec::new()
    }

    fn min_size(&self) -> Size {
        Size::new(
            self.params.margin[0]
                + self.params.margin[1]
                + self.params.padding[0]
                + self.params.padding[1],
            self.params.margin[2]
                + self.text.size().h
                + self.params.margin[3]
                + self.params.padding[2]
                + self.params.padding[3],
        )
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
