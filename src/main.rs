//! The user interaction layer.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

use std::{
    io::{self, Write},
    sync::Arc,
};

use ray_tracing::{
    camera::Camera,
    object::{List, Sphere},
    ray::{Hittable, RayHit},
    Color, Point3, Ray,
};

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    match world.hit_by(ray, 0.0..=f64::INFINITY) {
        None => {
            let unit_direction = ray.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1., 1., 1.).interpolate(&Color::new(0.5, 0.7, 1.0), t)
        }
        Some(RayHit { normal, .. }) => {
            if normal.dot(ray.direction()) < 0. {
                Color::new(
                    (normal.x() + 1.) / 2.,
                    (normal.y() + 1.) / 2.,
                    (normal.z() + 1.) / 2.,
                )
            } else {
                Color::new(
                    (-normal.x() + 1.) / 2.,
                    (-normal.y() + 1.) / 2.,
                    (-normal.z() + 1.) / 2.,
                )
            }
        }
    }
}

fn write_static_ppm_image(out: &mut dyn Write) -> io::Result<()> {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as _;
    const SAMPLES_PER_PIXEL: usize = 100;

    let mut world = List::default();
    world.push(Arc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.push(Arc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new(ASPECT_RATIO);

    writeln!(out, "P3")?;
    writeln!(out, "{WIDTH} {HEIGHT}")?;
    writeln!(out, "255")?;
    for j in (0..HEIGHT).rev() {
        writeln!(io::stderr().lock(), "Scanlines remaining: {j}")?;
        for i in 0..WIDTH {
            let color = Color::merge_samples((0..SAMPLES_PER_PIXEL).map(|_| {
                let u = (i as f64 + rand::random::<f64>()) / (WIDTH - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (HEIGHT - 1) as f64;
                ray_color(&camera.get_ray(u, v), &world)
            }));
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
