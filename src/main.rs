//! The user interaction layer.

#![warn(clippy::all)]
#![warn(missing_copy_implementations, missing_docs, rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn, missing_debug_implementations)]

use std::io::{self, Write};

use ray_tracing::Color;

fn write_static_ppm_image(out: &mut dyn Write) -> io::Result<()> {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    writeln!(out, "P3")?;
    writeln!(out, "{WIDTH} {HEIGHT}")?;
    writeln!(out, "255")?;
    for j in (0..HEIGHT).rev() {
        writeln!(io::stderr().lock(), "Scanlines remaining: {j}")?;
        for i in 0..WIDTH {
            let color = Color::new(
                f64::from(i) / f64::from(WIDTH - 1),
                f64::from(j) / f64::from(HEIGHT - 1),
                0.25,
            );
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
