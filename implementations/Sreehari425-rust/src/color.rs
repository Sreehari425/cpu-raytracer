use std::io::{self, Write};

use crate::vec3::Color;

pub fn write_color(mut out: impl Write, pixel_color: Color) -> io::Result<()> {
    let r = (255.999 * pixel_color.x()) as u32;
    let g = (255.999 * pixel_color.y()) as u32;
    let b = (255.999 * pixel_color.z()) as u32;
    writeln!(out, "{r} {g} {b}")
}
