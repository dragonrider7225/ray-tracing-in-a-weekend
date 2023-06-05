use rand::random;

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

/// A dielectric material allows light to pass through it but will change the angle at its surface
/// according to its refractive index. The refractive index of air is defined to be 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    /// Creates a new Dielectric material. `refractive_index` is a measure of how much light is
    /// biased toward the inward normal when it enters the material.
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cos_theta: f64, refractive_ratio: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1. - refractive_ratio) / (1. + refractive_ratio)).powi(2);
        r0 + (1. - r0) * (1. - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &RayHit) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let (eta, eta_prime, normal) = if ray.direction().dot(&hit_record.normal) < 0. {
            (1., self.refractive_index, hit_record.normal)
        } else {
            (self.refractive_index, 1., -hit_record.normal)
        };
        let unit_direction = ray.direction().normalized();
        let reflectance =
            Self::reflectance(-unit_direction.dot(&normal.normalized()), eta / eta_prime);
        let direction = if reflectance > random::<f64>() {
            unit_direction.reflect_about(&normal)
        } else {
            unit_direction.refract(&normal, eta, eta_prime)
        };
        let direction = Ray::new(hit_record.p, direction);
        Some(ScatterRecord {
            attenuation,
            direction,
        })
    }

    fn name(&self) -> &'static str {
        "Dielectric"
    }
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
