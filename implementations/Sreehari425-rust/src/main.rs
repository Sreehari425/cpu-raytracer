use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "A CLI to process a file", long_about = None)]
struct CpuTracer {
    /// Path to the target file (image ppm)
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,
}

fn main() {
    let args = CpuTracer::parse();

    println!("Target file path: {:?}", args.output);

    todo!("implment bruvh {:?}", args.output);
}
