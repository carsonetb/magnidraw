use std::{cell::RefCell, ops::DerefMut, rc::Rc, u32};

use glyphon::Edit;
use keydraw::state::State;

use crate::{Color, Pos, Size, TextAlign};

pub struct Text {
    pub editor: glyphon::Editor<'static>,
    pub text: String,
    pub font_size: f32,
    pub line_height: f32,
    pub line_length: Option<f32>,
    pub family: Option<&'static str>,
}

impl Text {
    pub fn inner_buffer(&self) -> &glyphon::Buffer {
        match self.editor.buffer_ref() {
            glyphon::cosmic_text::BufferRef::Owned(buffer) => buffer,
            _ => panic!(),
        }
    }

    pub fn inner_buffer_mut(&mut self) -> &mut glyphon::Buffer {
        match self.editor.buffer_ref_mut() {
            glyphon::cosmic_text::BufferRef::Owned(buffer) => buffer,
            _ => panic!(),
        }
    }

    pub fn size(&self) -> Size {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        for run in self.inner_buffer().layout_runs() {
            width = width.max(run.line_w);
            height += run.line_height;
        }

        Size::new(width, height)
    }

    pub fn align(&mut self, align: TextAlign) {
        for line in self.inner_buffer_mut().lines.iter_mut() {
            line.set_align(Some(align));
        }
    }

    pub(crate) fn new(
        font_system: &mut glyphon::FontSystem,
        text: &str,
        font_size: f32,
        line_height: f32,
        line_length: Option<f32>,
        family: Option<&'static str>,
    ) -> Self {
        let buffer =
            glyphon::Buffer::new(font_system, glyphon::Metrics::new(font_size, line_height));

        let editor = glyphon::Editor::new(buffer);

        let mut out = Self {
            editor,
            text: text.to_string(),
            font_size,
            line_height,
            line_length,
            family,
        };

        out.reload(font_system);
        out
    }

    pub(crate) fn reload(&mut self, font_system: &mut glyphon::FontSystem) {
        let attrs = &glyphon::Attrs::new().family(if let Some(family) = self.family {
            glyphon::Family::Name(&family)
        } else {
            glyphon::Family::SansSerif
        });
        let text = self.text.clone();
        let line_length = self.line_length;
        let buffer = self.inner_buffer_mut();
        buffer.set_size(line_length, None);
        buffer.set_text(&text, attrs, glyphon::Shaping::Advanced, None);
        buffer.shape_until_scroll(font_system, false);
    }
}

pub(crate) struct TextBatch<'a> {
    pub(crate) z_index: i32,
    pub(crate) font_system: Rc<RefCell<glyphon::FontSystem>>,
    pub(crate) swash_cache: Rc<RefCell<glyphon::SwashCache>>,
    pub(crate) text_atlas: Rc<RefCell<glyphon::TextAtlas>>,
    pub(crate) text_renderer: Rc<RefCell<glyphon::TextRenderer>>,
    pub(crate) viewport: &'a glyphon::Viewport,
    pub(crate) texts: Vec<(&'a Text, Pos, Color)>,
}

impl<'a> keydraw::ComplexCommand for TextBatch<'a> {
    fn key(&self) -> keydraw::DrawKey {
        keydraw::DrawKey::new(self.z_index, u32::MAX, &[])
    }

    fn prepare(&mut self, state: &State) {
        let text_areas = self
            .texts
            .iter()
            .map(|(text, pos, color)| glyphon::TextArea {
                buffer: &text.inner_buffer(),
                left: pos.x,
                top: pos.y,
                scale: 1.0,
                bounds: glyphon::TextBounds {
                    left: 0,
                    top: 0,
                    right: state.config.width as i32,
                    bottom: state.config.height as i32,
                },
                default_color: glyphon::Color::rgba(
                    (color.r * 255.0) as u8,
                    (color.g * 255.0) as u8,
                    (color.b * 255.0) as u8,
                    (color.a * 255.0) as u8,
                ),
                custom_glyphs: &[],
            })
            .collect::<Vec<_>>();

        self.text_renderer
            .borrow_mut()
            .prepare(
                &state.device,
                &state.queue,
                self.font_system.borrow_mut().deref_mut(),
                self.text_atlas.borrow_mut().deref_mut(),
                self.viewport,
                text_areas,
                self.swash_cache.borrow_mut().deref_mut(),
            )
            .unwrap();
    }

    fn render<'pass>(&'pass self, pass: &mut wgpu::RenderPass<'pass>) {
        self.text_renderer
            .borrow_mut()
            .render(
                self.text_atlas.borrow_mut().deref_mut(),
                self.viewport,
                pass,
            )
            .unwrap();
    }
}
