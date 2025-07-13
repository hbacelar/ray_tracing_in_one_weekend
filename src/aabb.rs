use crate::{
    interval::Interval,
    ray::Ray,
    vec3::Point,
};

#[derive(Debug, Clone, Default)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABB { x, y, z }
    }

    pub fn new_from_points(a: Point, b: Point) -> Self {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));

        AABB { x, y, z }
    }

    pub fn new_from_boxes(a: &AABB, b: &AABB) -> Self {
        let x = Interval::new_enclose_both(&a.x, &b.x);
        let y = Interval::new_enclose_both(&a.y, &b.y);
        let z = Interval::new_enclose_both(&a.z, &b.z);

        AABB { x, y, z }
    }

    fn axis_interval(&self, n: usize, r: &Ray) -> (&Interval, f64, f64) {
        if n == 1 {
            (&self.y, r.origin.y, r.dir.y)
        } else if n == 2 {
            (&self.z, r.origin.z, r.dir.z)
        } else {
            (&self.x, r.origin.x, r.dir.x)
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        for i in 0..3 {
            let (axis, ray_orig_axis, ray_dir_axis) = self.axis_interval(i, r);
            let ad_inv = 1.0 / ray_dir_axis;

            let t0 = (axis.min - ray_orig_axis) * ad_inv;
            let t1 = (axis.max - ray_orig_axis) * ad_inv;

            // checks ray inverse direction
            if t0 < t1 {
                ray_t.min = ray_t.min.max(t0);
                ray_t.max = ray_t.max.min(t1);
            } else {
                ray_t.min = ray_t.min.max(t1);
                ray_t.max = ray_t.max.min(t0);
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}
