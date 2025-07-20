use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct BVHNode<T> {
    left: Box<T>,
    right: Box<T>,
    bbox: AABB,
}

impl<T> BVHNode<T> {
    pub fn new_from_hittable_list() -> Self {
        todo!()
    }
}

impl<M, T: Hittable<M>> Hittable<M> for BVHNode<T> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<M>> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        let t_max = hit_left.as_ref().map_or(ray_t.max, |rec| rec.t);
        let hit_right = self.right.hit(ray, Interval::new(ray_t.min, t_max));

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
