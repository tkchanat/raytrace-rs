use crate::{material::*, math::*, ray::*};

// Sphere
pub struct Sphere<T: Material> {
    center: Point3,
    radius: f64,
    material: T,
}
impl<T: Material> Sphere<T> {
    pub fn new(center: Point3, radius: f64, material: T) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}
impl<T: Material> Hittable for Sphere<T> {
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
                let outward_normal = (point - self.center) / self.radius;
                let front_face = dot(ray.direction(), &outward_normal) < 0.0;
                let normal = if front_face {
                    outward_normal
                } else {
                    -outward_normal
                };
                return Some(RayHit {
                    point,
                    distance,
                    material: &self.material,
                    normal,
                    front_face,
                });
            }
            let beta = (-half_b + discriminant.sqrt()) / a;
            if beta < t_max && beta > t_min {
                let distance = beta;
                let point = ray.at(distance);
                let outward_normal = (point - self.center) / self.radius;
                let front_face = dot(ray.direction(), &outward_normal) < 0.0;
                let normal = if front_face {
                    outward_normal
                } else {
                    -outward_normal
                };
                return Some(RayHit {
                    point,
                    distance,
                    material: &self.material,
                    normal,
                    front_face,
                });
            }
        }
        None
    }
}
