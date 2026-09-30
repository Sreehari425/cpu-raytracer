use crate::ray::Ray;
use crate::vec3::{Color, Point3};

#[derive(Clone, Copy, Debug)]
pub enum Object {
    Sphere {
        center: Point3,
        radius: f64,
        color: Color,
    },
    BlackHoleImage {
        center: Point3,
        half_extent: f64,
    },
}

impl Object {
    pub const fn sphere(center: Point3, radius: f64, color: Color) -> Self {
        Self::Sphere {
            center,
            radius,
            color,
        }
    }

    pub const fn black_hole_image(center: Point3, half_extent: f64) -> Self {
        Self::BlackHoleImage {
            center,
            half_extent,
        }
    }

    /// Returns the nearest positive hit distance and the object's color at the hit.
    pub fn hit(self, ray: Ray, background: Color) -> Option<(f64, Color)> {
        match self {
            Self::Sphere {
                center,
                radius,
                color,
            } => {
                let origin_to_center = ray.origin() - center;
                let a = ray.direction().dot(ray.direction());
                let half_b = origin_to_center.dot(ray.direction());
                let c = origin_to_center.dot(origin_to_center) - radius * radius;
                let discriminant = half_b * half_b - a * c;

                if discriminant < 0.0 {
                    return None;
                }

                let sqrt_discriminant = discriminant.sqrt();
                let near = (-half_b - sqrt_discriminant) / a;
                let far = (-half_b + sqrt_discriminant) / a;
                let t = if near > 0.0 {
                    near
                } else if far > 0.0 {
                    far
                } else {
                    return None;
                };

                Some((t, color))
            }
            Self::BlackHoleImage {
                center,
                half_extent,
            } => {
                // The image plane faces the camera and is parallel to the XY plane.
                let t = (center.z - ray.origin().z) / ray.direction().z;
                if t <= 0.0 {
                    return None;
                }

                let point = ray.at(t);
                let u = (point.x - center.x) / half_extent;
                let v = (point.y - center.y) / half_extent;
                if u.abs() > 1.0 || v.abs() > 1.0 {
                    return None;
                }

                Some((t, black_hole_pixel(u, v, background)))
            }
        }
    }
}

fn black_hole_pixel(u: f64, v: f64, background: Color) -> Color {
    let radius = (u * u + v * v).sqrt();
    if radius > 0.82 {
        return background;
    }

    if radius < 0.23 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let ring = (-((radius - 0.39) / 0.075).powi(2)).exp();
    let horizontal_beaming = (0.35 + 0.65 * ((u / radius + 1.0) * 0.5)).clamp(0.0, 1.0);
    let glow = (ring * horizontal_beaming).clamp(0.0, 1.0);
    let disk = Color::new(glow, glow, glow);

    if glow > 0.015 {
        return disk;
    }

    if is_star(u, v) {
        Color::new(0.82, 0.82, 0.82)
    } else {
        Color::new(0.005, 0.008, 0.02)
    }
}

fn is_star(u: f64, v: f64) -> bool {
    let grid_x = (u + 1.0) * 52.0;
    let grid_y = (v + 1.0) * 52.0;
    let cell_x = grid_x.floor() as i32;
    let cell_y = grid_y.floor() as i32;
    let local_x = grid_x.fract();
    let local_y = grid_y.fract();

    let mut hash =
        (cell_x as u32).wrapping_mul(0x9E37_79B9) ^ (cell_y as u32).wrapping_mul(0x85EB_CA6B);
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x7FEB_352D);
    hash ^= hash >> 15;

    let star_x = ((hash & 0xffff) as f64) / 65536.0;
    let star_y = (((hash >> 16) & 0xffff) as f64) / 65536.0;
    let dx = local_x - star_x;
    let dy = local_y - star_y;
    (hash % 23 == 0) && dx * dx + dy * dy < 0.003
}
