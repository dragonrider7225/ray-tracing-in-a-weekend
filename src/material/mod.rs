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

/// A Metal material reflects (nearly) all light that hits it about its normal vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    /// Creates a new Metal material. The albedo is the amount of light in each channel that gets
    /// reflected in a scatter event. The fuzziness is how much the reflected rays "spread".
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

/// A Dielectric material reflects a portion of the light that hits it and refracts the rest. The
/// portion of the light that is reflected and the new angle of the refracted light depends on the
/// material's index of refraction.
///
/// In particular, when light passes from a material with refractive index `e1` into a material
/// with refractive index `e2`, at an angle of `theta1` from the outward-facing normal vector, the
/// angle `theta2` that the refracted ray makes with the inward-facing normal vector satisfies `e1
/// * theta1.sin() == e2 * theta2.sin()`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dielectric {
    albedo: Color,
    refractive_index: f64,
}

impl Dielectric {
    /// Creates a new Dielectric material. The albedo is the amount of light in each channel that
    /// gets reflected or refracted in a scatter event. The refractive index is correlated with the
    /// refracted rays' "resistance" to separation from the internally-facing normal vector.
    pub fn new(albedo: Color, refractive_index: f64) -> Self {
        Self {
            albedo,
            refractive_index,
        }
    }

    fn reflectance(&self, cos_theta: f64, refraction_ratio: f64) -> f64 {
        // Us Schlick's approximation for reflectance.
        let r0 = ((1. - refraction_ratio) / (1. + refraction_ratio)).powi(2);
        r0 + (1. - r0) * (1. - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &RayHit) -> Option<ScatterRecord> {
        let unit_direction = ray.direction().normalized();
        let cos_theta = unit_direction.dot(&hit_record.normal);
        let ray_inward = cos_theta < 0.;
        let (eta_from, eta_to) = if ray_inward {
            (1.0, self.refractive_index)
        } else {
            (self.refractive_index, 1.0)
        };
        let cannot_refract = eta_from / eta_to * (1. - cos_theta * cos_theta).sqrt() > 1.;
        if cannot_refract || self.reflectance(cos_theta, eta_from / eta_to) > rand::random() {
            Some(ScatterRecord {
                attenuation: self.albedo,
                direction: Ray::new(
                    hit_record.p,
                    unit_direction.reflect_about(&hit_record.normal),
                ),
            })
        } else {
            Some(ScatterRecord {
                attenuation: self.albedo,
                direction: Ray::new(
                    hit_record.p,
                    ray.direction()
                        .refract(&hit_record.normal, eta_from, eta_to),
                ),
            })
        }
    }

    fn name(&self) -> &'static str {
        "dielectric"
    }
}
