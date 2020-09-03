use crate::math::*;
use crate::ray::*;

// Sphere
pub struct Sphere {
    center: Point3,
    radius: f64,
}
impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point3::new(),
            radius: 1.0,
        }
    }
    pub fn from(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut RayHit) -> bool {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let alpha = (-half_b - discriminant.sqrt()) / a;
            if alpha < t_max && alpha > t_min {
                hit.t = alpha;
                hit.p = ray.at(hit.t);
                hit.set_face_normal(ray, &normalize(&(hit.p - self.center)));
                return true;
            }
            let beta = (-half_b + discriminant.sqrt()) / a;
            if beta < t_max && beta > t_min {
                hit.t = beta;
                hit.p = ray.at(hit.t);
                hit.set_face_normal(ray, &normalize(&(hit.p - self.center)));
                return true;
            }
        }
        false
    }
}
