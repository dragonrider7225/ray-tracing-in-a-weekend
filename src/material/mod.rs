use crate::{ray::RayHit, Color, Ray, Vec3};

/// A description of how rays scatter off of a surface.
pub trait Material {
    /// Scatters the given ray off of this material with the specified hit.
    fn scatter(&self, ray: &Ray, hit_record: &RayHit) -> Option<ScatterRecord>;

    /// The name of the material.
    fn name(&self) -> &'static str;
}

/// The information produced by calling [`Material::scatter()`].
#[derive(Clone, Copy, Debug)]
pub struct ScatterRecord {
    /// The amount by which each channel of the incoming color is attenuated.
    pub attenuation: Color,
    /// The direction that this particular ray is scattered.
    pub direction: Ray,
}

/// A Lambertian material appears equally bright from all angles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    /// Creates a new Lambertian material. The albedo is the amount of light in each channel that
    /// gets reflected in a scatter event.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &RayHit) -> Option<ScatterRecord> {
        let mut scatter_direction = Vec3::random_unit_vector()
            + if hit_record.normal.dot(ray.direction()) < 0. {
                hit_record.normal
            } else {
                -hit_record.normal
            };
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        Some(ScatterRecord {
            attenuation: self.albedo,
            direction: Ray::new(hit_record.p, scatter_direction),
        })
    }

    fn name(&self) -> &'static str {
        "lambertian"
    }
}

/// A Metal material reflects nearly all light that hits it about its normal vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    /// Creates a new Metal material. The albedo is the amount of light in each channel that gets
    /// reflected in a scatter event.
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self {
            albedo,
            fuzziness: fuzziness.clamp(0., 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &RayHit) -> Option<ScatterRecord> {
        let reflected = ray
            .direction()
            .normalized()
            .reflect_about(&hit_record.normal);
        Some(ScatterRecord {
            attenuation: self.albedo,
            direction: Ray::new(
                hit_record.p,
                reflected + self.fuzziness * Vec3::random_in_unit_sphere(),
            ),
        })
        .filter(|rec| {
            0. < rec
                .direction
                .direction()
                .dot(&(-hit_record.normal.dot(ray.direction()).signum() * hit_record.normal))
        })
    }

    fn name(&self) -> &'static str {
        "metal"
    }
}
