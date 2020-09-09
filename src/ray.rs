use crate::{aabb::*, material::*, math::*};

// Traits
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

// Ray
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}
impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: Option<f64>) -> Self {
        Ray {
            orig,
            dir,
            time: time.unwrap_or_default(),
        }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Point3 {
        &self.dir
    }
    pub fn time(&self) -> f64 {
        self.time
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

// RayHit
pub struct RayHit<'a> {
    pub point: Point3,
    pub distance: f64,
    pub material: &'a dyn Material,
    pub normal: Vec3,
    pub front_face: bool,
}

// HittableList
pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync + Send>>,
}
impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let mut closest_hit = None;
        let mut closest_distance = t_max;
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_distance) {
                closest_distance = hit.distance;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
    pub fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        let mut union_box = AABB::default();
        let mut first_box = true;
        for object in &self.objects {
            match object.bounding_box(t0, t1) {
                Some(x) => {
                    union_box = if first_box {
                        x
                    } else {
                        surrounding_box(&x, &union_box)
                    }
                }
                None => return None,
            }
        }
        Some(union_box)
    }
    pub fn len(&self) -> usize {
        self.objects.len()
    }
    pub fn objects(self) -> Vec<Box<dyn Hittable + Sync + Send>> {
        self.objects
    }
}
