use std::{
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A 3D vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    /// Creates a new `Vec3` that represents the vector `(x, y, z)`.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Gets the x-coordinate of the vector.
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Gets the y-coordinate of the vector.
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Gets the z-coordinate of the vector.
    pub const fn z(&self) -> f64 {
        self.z
    }

    /// The square of the vector's length. This method is a better option for comparing lengths of
    /// vectors since `a > b => sqrt(a) > sqrt(b)` for all non-negative `a` and `b`.
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// The length of the vector. If you only want the length to compare it with the length of
    /// another vector, use [`length_squared`] instead.
    ///
    /// [`length_squared`]: Self::length_squared()
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// The dot product of `self` and `rhs`. Geometrically, this is equivalent to `self.length() *
    /// rhs.length() * theta.cos()` where `theta` is the angle between the vectors.
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// The cross product of `self` and `rhs`. Geometrically, this is the vector with length
    /// `self.length() * rhs.length() * theta.sin()` in the direction of your right thumb when you
    /// point your fingers in the direction of `self` and your palm in the direction of `rhs`
    /// (where `theta` is the angle between `self` and `rhs`).
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Scales the vector so that [`self.length_squared()`] is `1`.
    ///
    /// [`self.length_squared()`]: Self::length_squared()
    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    /// Produces the same vector as [`normalize()`] except in a new value instead of in-place.
    ///
    /// [`normalize()`]: Self::normalize()
    pub fn normalized(&self) -> Self {
        let mut ret = *self;
        ret.normalize();
        ret
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index: {index}"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index: {index}"),
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.x *= -1.;
        self.y *= -1.;
        self.z *= -1.;
        self
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
