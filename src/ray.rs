use crate::vec3::{Point, Vec3};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Ray {
    pub dir: Vec3,
    pub origin: Point,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Self {
        Self {
            dir,
            origin,
            time: 0.0,
        }
    }

    pub fn at_time(origin: Point, dir: Vec3, time: f64) -> Self {
        Self { dir, origin, time }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
}
