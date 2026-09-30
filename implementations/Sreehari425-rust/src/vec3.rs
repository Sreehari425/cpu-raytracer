use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VectorKind;
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointKind;
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorKind;

//The reason i went with this apporch is beacuse Vector and Color Essentaily
// shared the same thing , yes i could have well duplicated (i didnt want to )
// So yeah :)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3<Kind> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    kind: PhantomData<Kind>,
}

pub type Vector3 = Vec3<VectorKind>;
pub type Point3 = Vec3<PointKind>;
pub type Color = Vec3<ColorKind>;

impl<Kind: Copy> Neg for Vec3<Kind> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Vector3) -> Point3 {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Point3> for Vector3 {
    type Output = Point3;
    fn add(self, rhs: Point3) -> Point3 {
        rhs + self
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Vector3) -> Point3 {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub for Point3 {
    type Output = Vector3;
    fn sub(self, rhs: Point3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<Kind: Copy> Mul for Vec3<Kind> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<Kind: Copy> Mul<f64> for Vec3<Kind> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<Kind: Copy> Mul<Vec3<Kind>> for f64 {
    type Output = Vec3<Kind>;
    fn mul(self, rhs: Vec3<Kind>) -> Vec3<Kind> {
        rhs * self
    }
}

impl<Kind: Copy> Div<f64> for Vec3<Kind> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        self * (1.0 / rhs)
    }
}

impl<Kind> Index<usize> for Vec3<Kind> {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of bounds: {index}"),
        }
    }
}

impl<Kind> IndexMut<usize> for Vec3<Kind> {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bounds: {index}"),
        }
    }
}

impl<Kind: Copy> Vec3<Kind> {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            kind: PhantomData,
        }
    }

    pub const fn x(self) -> f64 {
        self.x
    }
    pub const fn y(self) -> f64 {
        self.y
    }
    pub const fn z(self) -> f64 {
        self.z
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
