use crate::{material::*, math::*, ray::*};

// Sphere
pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: &'a dyn Material,
}
impl<'a> Sphere<'a> {
    pub fn from(center: Point3, radius: f64, material: &'a dyn Material) -> Self {
        Sphere { center, radius, material }
    }
}
impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let alpha = (-half_b - discriminant.sqrt()) / a;
            if alpha < t_max && alpha > t_min {
                let distance = alpha;
                let point = ray.at(distance);
                let outward_normal = normalize(&(point - self.center));
                let normal = -outward_normal * dot(ray.direction(), &outward_normal).signum();
                return Some(RayHit {
                    point,
                    distance,
                    material: self.material,
                    normal,
                });
            }
            let beta = (-half_b + discriminant.sqrt()) / a;
            if beta < t_max && beta > t_min {
                let distance = beta;
                let point = ray.at(distance);
                let outward_normal = normalize(&(point - self.center));
                let normal = -outward_normal * dot(ray.direction(), &outward_normal).signum();
                return Some(RayHit {
                    point,
                    distance,
                    material: self.material,
                    normal,
                });
            }
        }
        None
    }
}
