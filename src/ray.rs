use crate::{material::*, math::*};

// Traits
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit>;
}

// Ray
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}
impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Point3 {
        &self.dir
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
    pub front_face: bool
}

// HittableList
pub struct HittableList<T> {
    objects: Vec<T>,
}
impl<T:Hittable> HittableList<T> {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: T) {
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
}
