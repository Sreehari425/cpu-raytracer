use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

mod color;
mod error;
pub mod object;
pub mod vec3;

use color::write_color;
use object::Object;
use ray::Ray;
use vec3::{Color, Point3};

#[derive(Clone, Copy, Debug, ValueEnum)]
#[value(rename_all = "kebab-case")]
enum ObjectKind {
    Sphere,
    BlackHole,
}

#[derive(Parser, Debug)]
#[command(version, about = "A CPU ray tracer")]
struct CpuTracer {
    /// Path to the output PPM image
    #[arg(short, long, value_name = "FILE", default_value = "image.ppm")]
    output: PathBuf,

    /// Swap the black object and white background
    #[arg(short, long)]
    invert_colors: bool,

    /// Objects to render; repeat to include multiple objects
    #[arg(
        long = "object",
        value_enum,
        action = clap::ArgAction::Append,
        value_name = "OBJECT_NAME"
    )]
    objects: Vec<ObjectKind>,
}

fn ray_color(ray: Ray, objects: &[Object], background: Color, invert_colors: bool) -> Color {
    let color = objects
        .iter()
        .filter_map(|object| object.hit(ray, background))
        .min_by(|left, right| left.0.total_cmp(&right.0))
        .map_or(background, |(_, color)| color);

    if invert_colors {
        Color::new(1.0 - color.x(), 1.0 - color.y(), 1.0 - color.z())
    } else {
        color
    }
}

fn render(output: File, args: &CpuTracer) -> error::Result<()> {
    let mut out = BufWriter::new(output);

    let camera = camera::Camera::new(400, 16.0 / 9.0);
    let background = Color::new(1.0, 1.0, 1.0);
    let selected_objects = if args.objects.is_empty() {
        vec![ObjectKind::Sphere, ObjectKind::BlackHole]
    } else {
        args.objects.clone()
    };
    let objects: Vec<Object> = selected_objects
        .into_iter()
        .map(|kind| match kind {
            ObjectKind::Sphere => {
                Object::sphere(Point3::new(0.0, 0.0, -1.0), 0.5, Color::new(0.0, 0.0, 0.0))
            }
            ObjectKind::BlackHole => Object::black_hole_image(Point3::new(1.9, 0.0, -2.0), 0.85),
        })
        .collect();

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", camera.image_width(), camera.image_height())?;
    writeln!(out, "255")?;

    for y in 0..camera.image_height() {
        eprintln!("Scanlines remaining: {}", camera.image_height() - y);
        for x in 0..camera.image_width() {
            let ray = camera.ray_for_pixel(x, y);
            write_color(
                &mut out,
                ray_color(ray, &objects, background, args.invert_colors),
            )?;
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

    render(file, &args)?;

    Ok(())
}
