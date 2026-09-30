use crate::ray::Ray;
use crate::vec3::{Color, Point3};

#[derive(Clone, Copy, Debug)]
pub struct Object {
    center: Point3,
    radius: f64,
    color: Color,
}

impl Object {
    pub const fn sphere(center: Point3, radius: f64, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }

    pub const fn color(self) -> Color {
        self.color
    }

    /// Returns the nearest positive distance along the ray where it hits the sphere.
    pub fn hit_distance(self, ray: Ray) -> Option<f64> {
        let origin_to_center = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let half_b = origin_to_center.dot(ray.direction());
        let c = origin_to_center.dot(origin_to_center) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let nearest_root = (-half_b - sqrt_discriminant) / a;
        if nearest_root > 0.0 {
            return Some(nearest_root);
        }

        let farthest_root = (-half_b + sqrt_discriminant) / a;
        (farthest_root > 0.0).then_some(farthest_root)
    }
}
