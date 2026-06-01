use std::{cell::RefCell, ops::DerefMut, rc::Rc, u32};

use keydraw::state::State;

use crate::{Color, Pos};

pub struct Text {
    buffer: glyphon::Buffer,
    pub text: String,
    pub font_size: f32,
    pub line_height: f32,
    pub line_length: Option<f32>,
}

impl Text {
    pub(crate) fn new(
        font_system: &mut glyphon::FontSystem,
        text: &str,
        font_size: f32,
        line_height: f32,
        line_length: Option<f32>,
    ) -> Self {
        let mut buffer =
            glyphon::Buffer::new(font_system, glyphon::Metrics::new(font_size, line_height));

        buffer.set_size(font_system, line_length, None);
        buffer.set_text(
            font_system,
            text,
            &glyphon::Attrs::new().family(glyphon::Family::SansSerif),
            glyphon::Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(font_system, false);

        Self {
            buffer,
            text: text.to_string(),
            font_size,
            line_height,
            line_length,
        }
    }

    pub(crate) fn set_text(&mut self, font_system: &mut glyphon::FontSystem, text: &str) {
        self.buffer.set_text(
            font_system,
            text,
            &glyphon::Attrs::new().family(glyphon::Family::SansSerif),
            glyphon::Shaping::Advanced,
            None,
        );
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
                buffer: &text.buffer,
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
