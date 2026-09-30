use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use clap::Parser;

mod color;
mod error;
pub mod vec3;

use color::write_color;
use vec3::Color;

#[derive(Parser, Debug)]
#[command(version, about = "A CPU ray tracer")]
struct CpuTracer {
    /// Path to the output PPM image
    #[arg(short, long, value_name = "FILE", default_value = "image.ppm")]
    output: PathBuf,
}

fn ray_color(ray: ray::Ray) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn render(output: File) -> error::Result<()> {
    let mut out = BufWriter::new(output);

    let camera = camera::Camera::new(400, 16.0 / 9.0);

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", camera.image_width(), camera.image_height())?;
    writeln!(out, "255")?;

    for y in 0..camera.image_height() {
        eprintln!("Scanlines remaining: {}", camera.image_height() - y);
        for x in 0..camera.image_width() {
            let ray = camera.ray_for_pixel(x, y);
            write_color(&mut out, ray_color(ray))?;
        }
    }
    eprintln!("Done.");
    Ok(())
}

pub mod camera;
pub mod ray;

fn main() -> error::Result<()> {
    let args = CpuTracer::parse();

    let file = File::create(&args.output)?;

    render(file)?;

    Ok(())
}
