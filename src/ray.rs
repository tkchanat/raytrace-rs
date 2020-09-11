use crate::{aabb::*, material::*, math::*};

// // Traits
// pub trait Hittable {
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit>;
//     fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
// }

// Ray
#[derive(Copy, Clone)]
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
    point: Point3,
    distance: f64,
    material: &'a Material,
    normal: Vec3,
    uv: (f64, f64),
    front_face: bool,
}
impl<'a> RayHit<'a> {
    pub fn new(
        ray: &Ray,
        outward_normal: Vec3,
        distance: f64,
        material: &'a Material,
        uv: (f64, f64),
    ) -> Self {
        let point = ray.at(distance);
        let front_face = dot(ray.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        RayHit {
            point,
            distance,
            material,
            normal,
            uv,
            front_face,
        }
    }
    pub fn point(&self) -> &Point3 {
        &self.point
    }
    pub fn distance(&self) -> f64 {
        self.distance
    }
    pub fn material(&self) -> &'a Material {
        self.material
    }
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
    pub fn uv(&self) -> (f64, f64) {
        self.uv
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}