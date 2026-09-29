use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use clap::Parser;

mod error;

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
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            writeln!(out, "{ir} {ig} {ib}")?;
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
