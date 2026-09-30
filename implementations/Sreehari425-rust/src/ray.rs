use crate::vec3::{Point3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub const fn origin(self) -> Point3 {
        self.origin
    }

    pub const fn direction(self) -> Vector3 {
        self.direction
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
