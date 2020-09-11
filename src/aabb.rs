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

// BHV Node
pub struct BHVNode {
    bounding_box: AABB,
    left: Hittable,
    right: Hittable,
}
impl BHVNode {
    pub fn new(mut list: HittableList, time0: f64, time1: f64) -> Hittable {
        BHVNode::construct(list.objects(), time0, time1)
    }
    pub fn construct(mut objects: Vec<Hittable>, time0: f64, time1: f64) -> Hittable {
        let axis = random_range_int(0, 2);
        let comparator = |a: &Hittable, b: &Hittable| -> std::cmp::Ordering {
            let box_a = get_bounding_box(a, 0.0, 0.0);
            let box_b = get_bounding_box(b, 0.0, 0.0);
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
                Hittable::BHVNode(Box::new(BHVNode {
                    bounding_box: surrounding_box(
                        &get_bounding_box(&left, time0, time1).unwrap(),
                        &get_bounding_box(&right, time0, time1).unwrap(),
                    ),
                    left,
                    right,
                }))
            }
            _ => {
                let mut vec_right = objects;
                let vec_left = vec_right.split_off(object_span / 2);
                left = BHVNode::construct(vec_left, time0, time1);
                right = BHVNode::construct(vec_right, time0, time1);
                Hittable::BHVNode(Box::new(BHVNode {
                    bounding_box: surrounding_box(
                        &get_bounding_box(&left, time0, time1).unwrap(),
                        &get_bounding_box(&right, time0, time1).unwrap(),
                    ),
                    left,
                    right,
                }))
            }
        }
    }
    pub fn hit_check(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        self.bounding_box.hit(ray, t_min, t_max)
    }
    pub fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
    pub fn left(&self) -> &Hittable {
        &self.left
    }
    pub fn right(&self) -> &Hittable {
        &self.right
    }
}
