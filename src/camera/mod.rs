use crate::{angle::Angle, Point3, Ray, Vec3};

/// The point that the image is seen from.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    /// The unit vector along the negative x-axis in the final image.
    u: Vec3,
    /// The unit vector along the positive y-axis in the final image.
    v: Vec3,
    /// The unit vector that is directly forward from the camera.
    #[allow(unused)]
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// Creates a new camera.
    pub fn new(orientation: Orientation, structure: Structure) -> Self {
        let h = (structure.vertical_fov / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = structure.aspect_ratio * viewport_height;

        let w = (orientation.origin - orientation.look_at).normalized();
        let u = orientation.up.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = orientation.origin;
        let horizontal = structure.focus_distance * viewport_width * u;
        let vertical = structure.focus_distance * viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - structure.focus_distance * w;
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: structure.aperture_width / 2.,
        }
    }

    /// Gets a ray from the camera to the viewport coordinates `(u, v)`.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let fuzzed = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * fuzzed.x() + self.v * fuzzed.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
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

/// The structure of the camera.
#[derive(Clone, Copy, Debug)]
pub struct Structure {
    /// The maximum possible angle between the projections of two rays captured by the camera onto
    /// a vertical plane perpendicular to the focal plane.
    pub vertical_fov: Angle,
    /// The width of the image divided by its height.
    pub aspect_ratio: f64,
    /// The width of the camera's aperture.
    pub aperture_width: f64,
    /// The distance from the camera's lens to the plane that is in perfect focus.
    pub focus_distance: f64,
}
