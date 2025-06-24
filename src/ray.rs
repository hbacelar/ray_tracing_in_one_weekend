use crate::vec3::{Point, Vec3};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Ray {
    pub dir: Vec3,
    pub origin: Point,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self {
        Self { dir, origin }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
}
