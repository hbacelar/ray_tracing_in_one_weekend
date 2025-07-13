use crate::{
    aabb::AABB, hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, vec3::{Point, Vec3}
};

pub struct Sphere<'a, T> {
    center: Ray,
    radius: f64,
    material: &'a T,
    bbox: AABB,

}

impl<'a, T> Sphere<'a, T> {
    pub fn new(center: Point, radius: f64, material: &'a T) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bbox = AABB::new_from_points(center - r_vec, center + r_vec);

        Self {
            center: Ray::new(center, Vec3::default()),
            radius,
            material,
            bbox
        }
    }

    pub fn new_moving(center1: Point, center2: Point, radius: f64, material: &'a T) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bbox_1 = AABB::new_from_points(center1 - r_vec, center1 + r_vec);
        let bbox_2 = AABB::new_from_points(center2 - r_vec, center2 + r_vec);
        let bbox = AABB::new_from_boxes(&bbox_1, &bbox_2);

        Self {
            center: Ray::new(center1, center2 - center1),
            radius,
            material,
            bbox
        }
    }
}

impl<'a, T> Hittable<T> for Sphere<'a, T> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord<T>> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;
        if !ray_t.contains(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - current_center) / self.radius;
        Some(HitRecord::new(ray, p, root, outward_normal, self.material))
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
