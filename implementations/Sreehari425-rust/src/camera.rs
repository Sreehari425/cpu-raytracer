use crate::ray::Ray;
use crate::vec3::{Point3, Vector3};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    center: Point3,
    pixel00_location: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f64) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        let focal_length = 1.0;
        let center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            center,
            pixel00_location,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub const fn image_width(&self) -> usize {
        self.image_width
    }

    pub const fn image_height(&self) -> usize {
        self.image_height
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let pixel_center = self.pixel00_location
            + (x as f64 * self.pixel_delta_u)
            + (y as f64 * self.pixel_delta_v);
        Ray::new(self.center, pixel_center - self.center)
    }
}
