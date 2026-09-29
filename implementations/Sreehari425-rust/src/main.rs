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

fn render(output: File) -> error::Result<()> {
    let mut out = BufWriter::new(output);

    let width = 256;
    let height = 256;

    writeln!(out, "P3")?;
    writeln!(out, "{width} {height}")?;
    writeln!(out, "255")?;

    for j in 0..height {
        eprintln!("Scanlines remaining: {}", height - j);
        for i in 0..width {
            let pixel_color = Color::new(
                i as f64 / (width - 1) as f64,
                j as f64 / (height - 1) as f64,
                0.0,
            );
            write_color(&mut out, pixel_color)?;
        }
    }
    eprintln!("Done LOL");
    Ok(())
}

fn main() -> error::Result<()> {
    let args = CpuTracer::parse();

    let file = File::create(&args.output)?;

    render(file)?;

    Ok(())
}
