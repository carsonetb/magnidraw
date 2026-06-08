use std::ops::{Add, Div, Mul, Sub};

/// Represents a position in 2D space, in pixels.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<Pos> for Pos {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for Pos {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<Pos> for Pos {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Div<f32> for Pos {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Check if this point is inside a [`Rect`].
    pub fn inside(&self, rect: Rect) -> bool {
        self.x > rect.pos.x
            && self.x < rect.pos.x + rect.size.w
            && self.y > rect.pos.y
            && self.y < rect.pos.y + rect.size.h
    }
}

/// Representing a 2D size, in pixels.
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }

    pub fn as_pos(self) -> Pos {
        Pos::new(self.w, self.h)
    }
}

impl Mul<Size> for Size {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.w * rhs.w, self.h * rhs.h)
    }
}

impl Mul<f32> for Size {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.w * rhs, self.h * rhs)
    }
}

impl Div<Size> for Size {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.w / rhs.w, self.h / rhs.h)
    }
}

impl Div<f32> for Size {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.w / rhs, self.h / rhs)
    }
}

/// Represents a 2D scale.
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

/// Represents a rectangle in 2D space.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    /// The top-left corner of the rectangle.
    pub pos: Pos,
    /// The diameters of the rectangle.
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

    pub fn center(&self) -> Pos {
        Pos::new(
            self.pos.x + self.size.w / 2.0,
            self.pos.y + self.size.h / 2.0,
        )
    }
}

/// A simple RGB color, from 0.0-1.0.
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

    pub const fn rgb255(r: f32, g: f32, b: f32) -> Self {
        Self::rgb(r / 255.0, g / 255.0, b / 255.0)
    }

    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
}
