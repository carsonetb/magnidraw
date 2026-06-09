use keydraw::{Command, DrawKey, SimpleCommand, data::Vertex};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    Color, Pos, Rect, Scale,
    sprite::Sprite,
    text::{Text, TextBatch},
};

/// The drawer visits the [`crate::Game`] during the render phase to accumulate
/// draw commands. It enables simplified drawing of things like rectangles,
/// sprites, text, and more in the future.
pub struct Drawer<'a> {
    basic_material: u32,
    rect_pipeline: u32,
    rect_db: HashMap<i32, SimpleCommand>,
    rectext_pipeline: u32,
    rectext_db: HashMap<i32, SimpleCommand>,
    sprite_pipeline: u32,
    sprite_db: HashMap<(i32, u32), SimpleCommand>,
    text_db: HashMap<i32, Vec<(&'a Text, Pos, Color)>>,
}

impl<'a> Drawer<'a> {
    const RECT_VERTICES: [Vertex; 4] = [
        Vertex {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.0, 1.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
        },
    ];

    const RECT_INDICES: [u16; 6] = [0, 2, 3, 0, 3, 1];

    /// Draw an image onto the screen.
    pub fn sprite(&mut self, z_index: i32, sprite: &Sprite, pos: Pos, scale: Scale, color: Color) {
        let mut data = bytemuck::cast_slice(&[
            pos.x,
            pos.y,
            sprite.width * scale.x,
            sprite.height * scale.y,
            color.r,
            color.g,
            color.b,
            color.a,
            0.0, // TODO: This can be customized in the future.
            0.0,
            1.0,
            1.0,
        ])
        .to_vec();
        if let Some(command) = self.sprite_db.get_mut(&(z_index, sprite.material)) {
            command.instances.append(&mut data);
        } else {
            self.sprite_db.insert(
                (z_index, sprite.material),
                SimpleCommand {
                    key: DrawKey::new(
                        z_index,
                        self.sprite_pipeline,
                        &[self.basic_material, sprite.material],
                    ),
                    vertices: Self::RECT_VERTICES.to_vec(),
                    indices: Self::RECT_INDICES.to_vec(),
                    instances: data,
                    stride: size_of::<f32>() as u32 * 12,
                },
            );
        }
    }

    /// Draw text onto the screen.
    pub fn text(&mut self, z_index: i32, text: &'a Text, pos: Pos, color: Color) {
        if let Some(texts) = self.text_db.get_mut(&z_index) {
            texts.push((text, pos, color));
        } else {
            self.text_db.insert(z_index, vec![(text, pos, color)]);
        }
    }

    /// Draw a basic colored rectangle onto the screen.
    pub fn rect(&mut self, z_index: i32, rect: Rect, color: Color) {
        let mut data = bytemuck::cast_slice(&[
            rect.pos.x,
            rect.pos.y,
            rect.size.w,
            rect.size.h,
            color.r,
            color.g,
            color.b,
            color.a,
        ])
        .to_vec();
        if let Some(command) = self.rect_db.get_mut(&z_index) {
            command.instances.append(&mut data);
        } else {
            self.rect_db.insert(
                z_index,
                SimpleCommand {
                    key: DrawKey::new(z_index, self.rect_pipeline, &[self.basic_material]),
                    vertices: Self::RECT_VERTICES.to_vec(),
                    indices: Self::RECT_INDICES.to_vec(),
                    instances: data,
                    stride: size_of::<f32>() as u32 * 8,
                },
            );
        }
    }

    /// Draw a more complex rectangle onto the screen, with border radii,
    /// border width, and border color.
    pub fn rect_ext(
        &mut self,
        z_index: i32,
        rect: Rect,
        color: Color,
        tl: f32,
        tr: f32,
        bl: f32,
        br: f32,
        border_width: f32,
        border_color: Color,
    ) {
        let [tl, tr, bl, br] =
            [tl, tr, bl, br].map(|n| n.min(rect.size.w / 2.0).min(rect.size.h / 2.0));

        let mut data = bytemuck::cast_slice(&[
            rect.pos.x,
            rect.pos.y,
            rect.size.w,
            rect.size.h,
            color.r,
            color.g,
            color.b,
            color.a,
            tl,
            tr,
            bl,
            br,
            border_width,
            border_color.r,
            border_color.g,
            border_color.b,
            border_color.a,
        ])
        .to_vec();

        if let Some(command) = self.rectext_db.get_mut(&z_index) {
            command.instances.append(&mut data);
        } else {
            self.rectext_db.insert(
                z_index,
                SimpleCommand {
                    key: DrawKey::new(z_index, self.rectext_pipeline, &[self.basic_material]),
                    vertices: Self::RECT_VERTICES.to_vec(),
                    indices: Self::RECT_INDICES.to_vec(),
                    instances: data,
                    stride: size_of::<f32>() as u32 * 17,
                },
            );
        }
    }

    pub(crate) fn new(
        basic_material: u32,
        rect_pipeline: u32,
        rectext_pipeline: u32,
        sprite_pipeline: u32,
    ) -> Self {
        Self {
            basic_material,
            rect_pipeline,
            rect_db: HashMap::new(),
            rectext_pipeline,
            rectext_db: HashMap::new(),
            sprite_pipeline,
            sprite_db: HashMap::new(),
            text_db: HashMap::new(),
        }
    }

    pub(crate) fn collect(
        self,
        font_system: Rc<RefCell<glyphon::FontSystem>>,
        swash_cache: Rc<RefCell<glyphon::SwashCache>>,
        text_atlas: Rc<RefCell<glyphon::TextAtlas>>,
        text_renderer: Rc<RefCell<glyphon::TextRenderer>>,
        viewport: &'a glyphon::Viewport,
    ) -> Vec<Command<'a>> {
        let mut out = vec![];

        for (_, command) in self.rect_db {
            out.push(Command::Simple(command));
        }

        for (_, command) in self.rectext_db {
            out.push(Command::Simple(command));
        }

        for (_, command) in self.sprite_db {
            out.push(Command::Simple(command));
        }

        for (z_index, texts) in self.text_db {
            out.push(Command::Complex(Box::new(TextBatch {
                z_index,
                font_system: font_system.clone(),
                swash_cache: swash_cache.clone(),
                text_atlas: text_atlas.clone(),
                text_renderer: text_renderer.clone(),
                viewport,
                texts,
            })));
        }

        out
    }
}
