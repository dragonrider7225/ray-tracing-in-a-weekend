use crate::{angle::Angle, Point3, Ray, Vec3};

/// The point that the image is seen from.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// Creates a new camera.
    pub fn new(vertical_fov: Angle, aspect_ratio: f64) -> Self {
        let h = (vertical_fov / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;
        let origin = Point3::default();
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        Self {
            origin,
            lower_left_corner: origin
                - horizontal / 2.
                - vertical / 2.
                - Vec3::new(0., 0., focal_length),
            horizontal,
            vertical,
        }
    }

    /// Gets a ray from the camera to the viewport coordinates `(u, v)`.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
