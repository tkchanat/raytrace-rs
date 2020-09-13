use crate::{geometry::*, math::*, ray::*};

// Helper Functions
pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );
    let big = Point3::new(
        box0.max().x().max(box1.max().x()),
        box0.max().y().max(box1.max().y()),
        box0.max().z().max(box1.max().z()),
    );
    return AABB::new(small, big);
}

// AABB
#[derive(Clone, Default, Debug)]
pub struct AABB {
    min: Point3,
    max: Point3,
}
impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        AABB { min, max }
    }
    pub fn min(&self) -> &Point3 {
        &self.min
    }
    pub fn max(&self) -> &Point3 {
        &self.max
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let divider = 1.0 / ray.direction()[i];
            let mut t0 = (self.min[i] - ray.origin()[i]) * divider;
            let mut t1 = (self.max[i] - ray.origin()[i]) * divider;
            if divider < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t1.min(t_max) <= t0.max(t_min) {
                return false;
            }
        }
        true
    }
}
