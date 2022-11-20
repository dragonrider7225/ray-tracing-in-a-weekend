//! The user interaction layer.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

use std::io::{self, Write};

use ray_tracing::{Color, Point3, Ray, Vec3};

fn ray_color(ray: &Ray) -> Color {
    if ray.hits_sphere(&Point3::new(0., 0., -1.), 0.5) {
        Color::new(1., 0., 0.)
    } else {
        let unit_direction = ray.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1., 1., 1.).interpolate(&Color::new(0.5, 0.7, 1.0), t)
    }
}

fn write_static_ppm_image(out: &mut dyn Write) -> io::Result<()> {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as _;

    let viewport_height = 2.;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    writeln!(out, "P3")?;
    writeln!(out, "{WIDTH} {HEIGHT}")?;
    writeln!(out, "255")?;
    for j in (0..HEIGHT).rev() {
        writeln!(io::stderr().lock(), "Scanlines remaining: {j}")?;
        for i in 0..WIDTH {
            let u = i as f64 / (WIDTH - 1) as f64;
            let v = j as f64 / (HEIGHT - 1) as f64;
            let color = ray_color(&Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            ));
            writeln!(out, "{color}")?;
        }
    }
    writeln!(io::stderr().lock(), "Done")?;
    Ok(())
}

fn main() -> io::Result<()> {
    write_static_ppm_image(&mut io::stdout().lock())?;
    Ok(())
}
