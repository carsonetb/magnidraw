#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

impl Scale {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ONE: Self = Self::new(1.0, 1.0);
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos: Pos,
    pub size: Size,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            pos: Pos::new(x, y),
            size: Size::new(w, h),
        }
    }

    pub fn new_basic(pos: Pos, size: Size) -> Self {
        Self { pos, size }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
}
