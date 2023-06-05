use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// An angle with units.
#[derive(Clone, Copy, Debug)]
pub enum Angle {
    /// An angle expressed in degrees.
    Degrees(f64),
    /// An angle expressed in radians.
    Radians(f64),
}

impl Angle {
    /// Expresses the given angle in degrees.
    pub fn to_degrees(self) -> Self {
        match self {
            Self::Degrees(_) => self,
            Self::Radians(r) => Self::Degrees(r.to_degrees()),
        }
    }

    /// Returns the size of the angle in degrees, converting if necessary.
    pub fn unwrap_degrees(self) -> f64 {
        match self.to_degrees() {
            Self::Degrees(d) => d,
            _ => unreachable!("Angle::to_degrees() should always produce Angle::Degrees"),
        }
    }

    /// Expresses the given angle in radians.
    pub fn to_radians(self) -> Self {
        match self {
            Self::Degrees(d) => Self::Radians(d.to_radians()),
            Self::Radians(_) => self,
        }
    }

    /// Returns the size of the angle in radians, converting if necessary.
    pub fn unwrap_radians(self) -> f64 {
        match self.to_radians() {
            Self::Radians(r) => r,
            _ => unreachable!("Angle::to_radians() should always produce Angle::Radians"),
        }
    }

    /// Computes the sine of the angle.
    pub fn sin(self) -> f64 {
        self.unwrap_radians().sin()
    }

    /// Computes the angle with the given sine.
    pub fn asin(s: f64) -> Self {
        Self::Radians(s.asin())
    }

    /// Computes the cosine of the angle.
    pub fn cos(self) -> f64 {
        self.unwrap_radians().cos()
    }

    /// Computes the angle with the given cosine.
    pub fn acos(c: f64) -> Self {
        Self::Radians(c.acos())
    }

    /// Computes the sine and cosine of the angle simultaneously.
    pub fn sin_cos(self) -> (f64, f64) {
        self.unwrap_radians().sin_cos()
    }

    /// Computes the tangent of the angle.
    pub fn tan(self) -> f64 {
        self.unwrap_radians().tan()
    }

    /// Computes the angle in the range [-pi/2 rad, pi/2 rad] with the given tangent.
    pub fn atan(t: f64) -> Self {
        Self::Radians(t.atan())
    }

    /// Computes the angle in the range (-pi rad, pi rad] made by the positive x-axis and the ray
    /// from the origin to the point (x, y). The origin is defined to have an atan of 0.
    pub fn atan2(x: f64, y: f64) -> Self {
        Self::Radians(y.atan2(x))
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Degrees(this), Self::Degrees(other)) => Self::Degrees(this + other),
            (Self::Radians(this), Self::Radians(other)) => Self::Radians(this + other),
            (Self::Degrees(_), _) => self.to_radians() + rhs,
            (_, Self::Degrees(_)) => self + rhs.to_radians(),
        }
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Div<f64> for Angle {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        match self {
            Self::Degrees(d) => Self::Degrees(d / rhs),
            Self::Radians(r) => Self::Radians(r / rhs),
        }
    }
}

impl DivAssign<f64> for Angle {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<f64> for Angle {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Self::Degrees(d) => Self::Degrees(d * rhs),
            Self::Radians(r) => Self::Radians(r * rhs),
        }
    }
}

impl MulAssign<f64> for Angle {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Degrees(d) => Self::Degrees(-d),
            Self::Radians(r) => Self::Radians(-r),
        }
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
