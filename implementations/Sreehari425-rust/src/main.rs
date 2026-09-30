use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use clap::Parser;

mod color;
mod error;
pub mod object;
pub mod vec3;

use color::write_color;
use object::Object;
use ray::Ray;
use vec3::{Color, Point3};

#[derive(Parser, Debug)]
#[command(version, about = "A CPU ray tracer")]
struct CpuTracer {
    /// Path to the output PPM image
    #[arg(short, long, value_name = "FILE", default_value = "image.ppm")]
    output: PathBuf,

    /// Swap the black object and white background
    #[arg(short, long)]
    invert_colors: bool,
}

fn ray_color(ray: Ray, object: Object, background: Color) -> Color {
    if object.hit_distance(ray).is_some() {
        object.color()
    } else {
        background
    }
}

fn render(output: File, invert_colors: bool) -> error::Result<()> {
    let mut out = BufWriter::new(output);

    let camera = camera::Camera::new(400, 16.0 / 9.0);
    let (object_color, background) = if invert_colors {
        (Color::new(1.0, 1.0, 1.0), Color::new(0.0, 0.0, 0.0))
    } else {
        (Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0))
    };
    let sphere = Object::sphere(Point3::new(0.0, 0.0, -1.0), 0.5, object_color);

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", camera.image_width(), camera.image_height())?;
    writeln!(out, "255")?;

    for y in 0..camera.image_height() {
        eprintln!("Scanlines remaining: {}", camera.image_height() - y);
        for x in 0..camera.image_width() {
            let ray = camera.ray_for_pixel(x, y);
            write_color(&mut out, ray_color(ray, sphere, background))?;
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

    render(file, args.invert_colors)?;

    Ok(())
}
