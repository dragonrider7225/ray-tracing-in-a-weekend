use std::{
    fmt::{self, Display, Formatter},
    ops::{Div, DivAssign, Index, Mul, MulAssign},
};

/// An RGB color. The intensity of each component is in the range `[0.0, 1.0]`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    /// Create a new color with the specified components.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r.clamp(0., 1.),
            g: g.clamp(0., 1.),
            b: b.clamp(0., 1.),
        }
    }

    /// Gets the red part of the color.
    pub const fn red(&self) -> f64 {
        self.r
    }

    /// Gets the green part of the color.
    pub const fn green(&self) -> f64 {
        self.g
    }

    /// Gets the blue part of the color.
    pub const fn blue(&self) -> f64 {
        self.b
    }

    /// Sets the red part of the color.
    pub fn set_red(&mut self, r: f64) {
        self.r = r.clamp(0., 1.);
    }

    /// Sets the green part of the color.
    pub fn set_green(&mut self, g: f64) {
        self.g = g.clamp(0., 1.);
    }

    /// Sets the blue part of the color.
    pub fn set_blue(&mut self, b: f64) {
        self.b = b.clamp(0., 1.);
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.r * 255.999) as u32,
            (self.g * 255.999) as u32,
            (self.b * 255.999) as u32,
        )
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("Invalid index: {index}"),
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, mut rhs: Color) -> Self::Output {
        rhs *= self;
        rhs
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.set_red(self.r * rhs);
        self.set_green(self.g * rhs);
        self.set_blue(self.b * rhs);
    }
}
