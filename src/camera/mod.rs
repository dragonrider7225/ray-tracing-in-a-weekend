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
    pub fn new(orientation: Orientation, vertical_fov: Angle, aspect_ratio: f64) -> Self {
        let h = (vertical_fov / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (orientation.origin - orientation.look_at).normalized();
        let u = orientation.up.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = orientation.origin;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            origin,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - w,
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

/// The location and orientation of the camera.
#[derive(Clone, Copy, Debug)]
pub struct Orientation {
    /// The position of the camera.
    pub origin: Point3,
    /// A point that is directly in front of the camera.
    pub look_at: Point3,
    /// A vector that, when projected onto the view screen, points directly up.
    pub up: Vec3,
}
