use crate::{
    aabb::AABB, interval::Interval, ray::Ray, vec3::{Point, Vec3}
};

pub struct HitRecord<'a, T> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a T,
}

impl<'a, T> HitRecord<'a, T> {
    pub fn new(ray: &Ray, p: Point, t: f64, outward_normal: Vec3, mat: &'a T) -> Self {
        let front_face = ray.dir.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
            material: mat,
        }
    }
}

pub trait Hittable<T> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<T>>;

    fn bounding_box(&self) -> AABB; 
}

impl<T: Hittable<M>, M> Hittable<M> for &[T] {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<M>> {
        let mut hit: Option<HitRecord<M>> = None;
        let mut closest_so_far = ray_t.max;

        for hittable in self.iter() {
            if let Some(hit_record) = hittable.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_record.t;
                hit = Some(hit_record);
            }
        }

        hit
    }

    fn bounding_box(&self) -> AABB {
        todo!()
    }
}
