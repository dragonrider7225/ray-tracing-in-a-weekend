use std::{
    fmt::{self, Display, Formatter},
    ops::{Div, DivAssign, Index, Mul, MulAssign, Range},
};

use rand::{
    distributions::{Standard, Uniform},
    prelude::Distribution,
    Rng,
};
use rayon::prelude::ParallelIterator;

use crate::Vec3;

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

    /// Averages the samples to produce a single color.
    pub fn merge_samples(samples: impl ParallelIterator<Item = Self>) -> Self {
        let (num_samples, sum) = samples
            .map(|sample| (1., Vec3::from(sample)))
            .reduce(Default::default, |(c1, s1), (c2, s2)| (c1 + c2, s1 + s2));
        Self {
            r: sum.x() / num_samples,
            g: sum.y() / num_samples,
            b: sum.z() / num_samples,
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

    /// Interpolates linearly from `self` to `other`. If `t <= 0.0`, returns `self`. If `t >= 1.0`,
    /// returns `other`. Otherwise, each component of the returned `Color` is `1-t` times that
    /// component of `self` plus `t` times that component of `other`.
    pub fn interpolate(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0., 1.);
        Self::new(
            (1. - t) * self.r + t * other.r,
            (1. - t) * self.g + t * other.g,
            (1. - t) * self.b + t * other.b,
        )
    }

    /// Multiplies the corresponding channels of the two colors together.
    pub fn attenuate(&self, rhs: &Self) -> Self {
        Self::new(
            self.red() * rhs.red(),
            self.green() * rhs.green(),
            self.blue() * rhs.blue(),
        )
    }

    /// Generates a random color where each channel is uniformly-distributed in the given range.
    pub fn random(range: Range<f64>) -> Self {
        Uniform::from(range).sample(&mut rand::thread_rng())
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

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let mut channels = <&Self as Distribution<f64>>::sample_iter(self, rng).take(3);
        Color::new(
            channels.next().unwrap(),
            channels.next().unwrap(),
            channels.next().unwrap(),
        )
    }
}

impl Distribution<Color> for Uniform<f64> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let mut channels = <&Self as Distribution<f64>>::sample_iter(self, rng).take(3);
        Color::new(
            channels.next().unwrap(),
            channels.next().unwrap(),
            channels.next().unwrap(),
        )
    }
}
