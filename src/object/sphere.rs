use std::ops::RangeInclusive;

use crate::{
    ray::{Hittable, RayHit},
    Point3, Ray, Vec3,
};

/// A sphere.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    /// Creates a new sphere centered at `center` and with a radius of `radius`.
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.),
        }
    }

    /// Gets the center point of the sphere.
    pub fn center(&self) -> Point3 {
        self.center
    }

    /// Gets the radius of the sphere.
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Computes the normal vector at `p` assuming that `p` is on the surface of the sphere.
    fn normal(&self, p: Point3) -> Vec3 {
        (p - self.center()) / self.radius()
    }
}

impl Hittable for Sphere {
    fn hit_by(&self, ray: &Ray, valid_t: RangeInclusive<f64>) -> Option<RayHit> {
        let co = *ray.origin() - self.center();
        let a = ray.direction().length_squared();
        let half_b = co.dot(ray.direction());
        let c = co.length_squared() - self.radius().powi(2);
        let quarter_discriminant = half_b * half_b - a * c;
        if quarter_discriminant < 0. {
            None
        } else {
            let half_sdiscriminant = quarter_discriminant.sqrt();
            let t0 = (-half_b - half_sdiscriminant) / a;
            let t1 = t0 + 2. * half_sdiscriminant / a;
            if valid_t.contains(&t0) {
                let p = ray.at(t0);
                Some(RayHit {
                    p,
                    normal: self.normal(p),
                    t: t0,
                })
            } else if valid_t.contains(&t1) {
                let p = ray.at(t1);
                Some(RayHit {
                    p,
                    normal: self.normal(p),
                    t: t1,
                })
            } else {
                None
            }
        }
    }
}
