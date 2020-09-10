use crate::{math::*, ray::*};

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

// BHV Node
pub struct BHVNode {
    bounding_box: AABB,
    left: Box<dyn Hittable + Send + Sync>,
    right: Box<dyn Hittable + Send + Sync>,
}
impl BHVNode {
    pub fn new(list: HittableList, time0: f64, time1: f64) -> Box<dyn Hittable + Send + Sync> {
        BHVNode::construct(list.objects(), time0, time1)
    }
    fn construct(
        mut objects: Vec<Box<dyn Hittable + Sync + Send>>,
        time0: f64,
        time1: f64,
    ) -> Box<dyn Hittable + Send + Sync> {
        let axis = random_range_int(0, 2);
        let comparator = |a: &Box<dyn Hittable + Send + Sync>,
                          b: &Box<dyn Hittable + Send + Sync>|
         -> std::cmp::Ordering {
            let box_a = a.bounding_box(0.0, 0.0);
            let box_b = b.bounding_box(0.0, 0.0);
            if box_a.is_none() || box_b.is_none() {
                panic!("No bounding box in BVHNode constructor!");
            }
            box_a.unwrap().min()[axis]
                .partial_cmp(&box_b.unwrap().min()[axis])
                .unwrap()
        };
        objects.sort_by(comparator);
        let left;
        let right;
        let object_span = objects.len();
        match object_span {
            0 => panic!("No objects in leaf node!"),
            1 => objects.remove(0),
            2 => {
                right = objects.remove(1);
                left = objects.remove(0);
                Box::new(BHVNode {
                    bounding_box: surrounding_box(
                        &left.bounding_box(time0, time1).unwrap(),
                        &right.bounding_box(time0, time1).unwrap(),
                    ),
                    left,
                    right,
                })
            }
            _ => {
                let mut vec_right = objects;
                let vec_left = vec_right.split_off(object_span / 2);
                left = BHVNode::construct(vec_left, time0, time1);
                right = BHVNode::construct(vec_right, time0, time1);
                Box::new(BHVNode {
                    bounding_box: surrounding_box(
                        &left.bounding_box(time0, time1).unwrap(),
                        &right.bounding_box(time0, time1).unwrap(),
                    ),
                    left,
                    right,
                })
            }
        }
    }
}
impl Hittable for BHVNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(ray, t_min, t_max);
        let t = match &hit_left {
            Some(hit) => hit.distance(),
            None => t_min,
        };
        let hit_right = self.right.hit(ray, t, t_max);
        if hit_left.is_some() {
            return hit_left;
        } else if hit_right.is_some() {
            return hit_right;
        }
        None
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}
