use crate::{Point3, Vec3};

/// The path of a light ray.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// Creates a new ray starting at `origin` and traveling by `direction` per unit time.
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// The position of the ray at time 0.
    pub const fn origin(&self) -> &Point3 {
        &self.origin
    }

    /// The difference between the position of the ray at time 1 and the position of the ray at
    /// time 0.
    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }

    /// The position of the ray at time `time`.
    pub fn at(&self, time: f64) -> Point3 {
        self.origin + time * self.direction
    }

    /// Checks whether the ray passes through the interior of the sphere with radius `radius`
    /// centered at `center`.
    pub fn hits_sphere(&self, center: &Point3, radius: f64) -> bool {
        let co = self.origin() - center;
        let a = self.direction().length_squared();
        let b = 2. * co.dot(self.direction());
        let c = co.length_squared() - radius * radius;
        let discriminant = b * b - 4. * a * c;
        discriminant > 0.
    }
}
