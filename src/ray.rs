use crate::math::*;

// Traits
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut RayHit) -> bool;
}

// Ray
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}
impl Ray {
    pub fn from(orig: Point3, dir: Vec3) -> Self {
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
#[derive(Debug, Copy, Clone)]
pub struct RayHit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
impl RayHit {
    pub fn new() -> Self {
        RayHit {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

// HittableList
pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}
impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn from(object: T) -> Self {
        HittableList {
            objects: vec![object],
        }
    }
    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut RayHit) -> bool {
        let mut temp_hit = RayHit::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_hit) {
                hit_anything = true;
                closest_so_far = temp_hit.t;
                *hit = temp_hit;
            }
        }
        hit_anything
    }
}
