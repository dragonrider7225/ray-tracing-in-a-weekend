//! The user interaction layer.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

use std::{
    io::{self, Write},
    sync::Arc,
};

use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};
use ray_tracing::{
    angle::Angle,
    camera::{Camera, Orientation, Structure},
    material::{Dielectric, Lambertian, Metal, ScatterRecord},
    object::{List, Sphere},
    ray::Hittable,
    Color, Point3, Ray, Vec3,
};

fn ray_color(ray: &Ray, world: &dyn Hittable, max_depth: usize) -> Color {
    if max_depth == 0 {
        return Color::new(0., 0., 0.);
    }
    match world.hit_by(ray, 0.001..=f64::INFINITY) {
        None => {
            let unit_direction = ray.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1., 1., 1.).interpolate(&Color::new(0.5, 0.7, 1.0), t)
        }
        Some(hit_record) => hit_record
            .material
            .scatter(ray, &hit_record)
            .map(
                |ScatterRecord {
                     attenuation,
                     direction,
                 }| {
                    ray_color(&direction, world, max_depth - 1).attenuate(&attenuation)
                },
            )
            .unwrap_or_default(),
    }
}

fn write_image(
    out: &mut dyn Write,
    width: u32,
    height: u32,
    samples_per_pixel: usize,
    camera: &Camera,
    world: &dyn Hittable,
    max_depth: usize,
) -> io::Result<()> {
    writeln!(out, "P3")?;
    writeln!(out, "{width} {height}")?;
    writeln!(out, "255")?;
    for j in (0..height).rev() {
        writeln!(io::stderr().lock(), "Scanlines remaining: {j}")?;
        for i in 0..width {
            let color = Color::merge_samples((0..samples_per_pixel).map(|_| {
                let u = (i as f64 + rand::random::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (height - 1) as f64;
                ray_color(&camera.get_ray(u, v), world, max_depth)
            }));
            // Gamma-correct for gamma=2.0.
            let color = Color::new(
                color.red().sqrt(),
                color.green().sqrt(),
                color.blue().sqrt(),
            );
            writeln!(out, "{color}")?;
        }
    }
    writeln!(io::stderr().lock(), "Done")?;
    Ok(())
}

fn random_scene() -> List {
    let mut world = List::default();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Arc::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    let material_weights = [16, 3, 1];
    let distribution = WeightedIndex::new(&material_weights).unwrap();
    let mut rng = rand::thread_rng();
    for a in (-11..11).map(f64::from) {
        for b in (-11..11).map(f64::from) {
            let center = Point3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());
            if (center - Point3::new(4., 0.2, 0.)).length_squared() < 0.81 {
                continue;
            }
            match distribution.sample(&mut rng) {
                0 => {
                    let albedo = rng.gen::<Color>().attenuate(&rng.gen());
                    let material = Arc::new(Lambertian::new(albedo));
                    world.push(Arc::new(Sphere::new(center, 0.2, material)));
                }
                1 => {
                    let albedo = Color::random(0.5..1.);
                    let fuzziness = 0.5 * rng.gen::<f64>();
                    let material = Arc::new(Metal::new(albedo, fuzziness));
                    world.push(Arc::new(Sphere::new(center, 0.2, material)));
                }
                2 => {
                    let material = Arc::new(Dielectric::new(1.5));
                    world.push(Arc::new(Sphere::new(center, 0.2, material)));
                }
                n => unreachable!("Unknown material ID {n}"),
            };
        }
    }

    let material = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(Point3::new(0., 1., 0.), 1., material)));

    let material = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material,
    )));

    let material = Arc::new(Metal::new(Color::new(0.4, 0.2, 0.1), 0.));
    world.push(Arc::new(Sphere::new(Point3::new(4., 1., 0.), 1., material)));

    world
}

fn static_scene() -> List {
    let mut world = List::default();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left_material = Arc::new(Dielectric::new(1.5));
    let right_material = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        ground_material,
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        center_material,
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        Arc::clone(&left_material),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.45,
        left_material,
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        right_material,
    )));
    world
}

fn write_random_ppm_image(out: &mut dyn Write) -> io::Result<()> {
    const ASPECT_RATIO: f64 = 3. / 2.;
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as _;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: usize = 50;

    let world = random_scene();

    let camera = Camera::new(
        Orientation {
            origin: Point3::new(13., 2., 3.),
            look_at: Default::default(),
            up: Vec3::new(0., 1., 0.),
        },
        Structure {
            vertical_fov: Angle::Degrees(20.),
            aspect_ratio: ASPECT_RATIO,
            aperture_width: 0.1,
            focus_distance: 10.,
        },
    );

    write_image(
        out,
        WIDTH,
        HEIGHT,
        SAMPLES_PER_PIXEL,
        &camera,
        &world,
        MAX_DEPTH,
    )
}

fn write_static_ppm_image(out: &mut dyn Write) -> io::Result<()> {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as _;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;

    let world = static_scene();

    let camera_origin = Point3::new(3., 3., 2.);
    let look_at = Point3::new(0., 0., -1.);
    let camera = Camera::new(
        Orientation {
            origin: camera_origin,
            look_at,
            up: Vec3::new(0., 1., 0.),
        },
        Structure {
            vertical_fov: Angle::Degrees(20.),
            aspect_ratio: ASPECT_RATIO,
            aperture_width: 2.,
            focus_distance: (camera_origin - look_at).length(),
        },
    );

    write_image(
        out,
        WIDTH,
        HEIGHT,
        SAMPLES_PER_PIXEL,
        &camera,
        &world,
        MAX_DEPTH,
    )
}

fn main() -> io::Result<()> {
    write_random_ppm_image(&mut io::stdout().lock())?;
    Ok(())
}
