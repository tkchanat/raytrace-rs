use crate::{aabb::*, material::*, math::*, ray::*};

// Helper function
fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}

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
        let create_hit_by_distance = |distance: f64| -> Option<RayHit> {
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
                uv: get_sphere_uv(&((point - self.center) / self.radius)),
                front_face,
            });
        };
        if discriminant > 0.0 {
            let alpha = (-half_b - discriminant.sqrt()) / a;
            if alpha < t_max && alpha > t_min {
                return create_hit_by_distance(alpha);
            }
            let beta = (-half_b + discriminant.sqrt()) / a;
            if beta < t_max && beta > t_min {
                return create_hit_by_distance(beta);
            }
        }
        None
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

// Moving Sphere
pub struct MovingSphere<T: Material> {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: T,
}
impl<T: Material> MovingSphere<T> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: T,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}
impl<T: Material> Hittable for MovingSphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let center = self.center(ray.time());
        let oc = *ray.origin() - center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        let create_hit_by_distance = |distance: f64| -> Option<RayHit> {
            let point = ray.at(distance);
            let outward_normal = (point - center) / self.radius;
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
                uv: get_sphere_uv(&((point - center) / self.radius)),
                front_face,
            });
        };
        if discriminant > 0.0 {
            let alpha = (-half_b - discriminant.sqrt()) / a;
            if alpha < t_max && alpha > t_min {
                return create_hit_by_distance(alpha);
            }
            let beta = (-half_b + discriminant.sqrt()) / a;
            if beta < t_max && beta > t_min {
                return create_hit_by_distance(beta);
            }
        }
        None
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center0 - Vec3::new(self.radius, self.radius, self.radius),
            self.center0 + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center1 - Vec3::new(self.radius, self.radius, self.radius),
            self.center1 + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(surrounding_box(&box0, &box1))
    }
}
