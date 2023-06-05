use std::{
    fmt::{self, Debug, Formatter},
    ops::RangeInclusive,
    sync::Arc,
};

use crate::{Material, Point3, Vec3};

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

    /// Checks whether the ray hits `h`.
    pub fn hits(&self, h: &dyn Hittable) -> Option<RayHit> {
        h.hit_by(self, 0.0..=f64::MAX)
    }
}

/// The intersection of a ray with a [`Hittable`] object.
#[derive(Clone)]
pub struct RayHit {
    /// The point on the surface of the [`Hittable`] object where the ray hit.
    pub p: Point3,
    /// The normal vector to the surface of the [`Hittable`] object at `p`.
    pub normal: Vec3,
    /// The material of the [`Hittable`] object at `p`.
    pub material: Arc<dyn Material>,
    /// The time at which the ray hit `p`.
    pub t: f64,
}

impl Debug for RayHit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RayHit")
            .field("p", &self.p)
            .field("normal", &self.normal)
            .field("material", &self.material.name())
            .field("t", &self.t)
            .finish()
    }
}

/// An object that can be hit by a [`Ray`].
pub trait Hittable: Send + Sync {
    /// Checks whether the ray hits this object no earlier than `valid_t.start()` and no later than
    /// `valid_t.end()`. If it does, returns the lowest such value of `t`.
    fn hit_by(&self, ray: &Ray, valid_t: RangeInclusive<f64>) -> Option<RayHit>;
}
