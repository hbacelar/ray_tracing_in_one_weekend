use crate::{
    ray::Ray,
    vec::{self, Vec3},
};

#[derive(Debug)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, outward_normal: Vec3, r: &Ray, t: f64) -> Self {
        let front_face = vec::dot(&r.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
