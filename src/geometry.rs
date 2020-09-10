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
            return Some(RayHit::new(
                ray,
                outward_normal,
                distance,
                &self.material,
                get_sphere_uv(&((point - self.center) / self.radius)),
            ));
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
            return Some(RayHit::new(
                ray,
                outward_normal,
                distance,
                &self.material,
                get_sphere_uv(&((point - center) / self.radius)),
            ));
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

// XYRects
pub struct XYRect<T: Material> {
    x: (f64, f64),
    y: (f64, f64),
    k: f64,
    material: T,
}
impl<T: Material> XYRect<T> {
    pub fn new(x: (f64, f64), y: (f64, f64), k: f64, material: T) -> Self {
        XYRect { x, y, k, material }
    }
}
impl<T: Material> Hittable for XYRect<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();
        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return None;
        }
        let u = (x - self.x.0) / (self.x.1 - self.x.0);
        let v = (y - self.y.0) / (self.y.1 - self.y.0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        Some(RayHit::new(ray, outward_normal, t, &self.material, (u, v)))
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x.0, self.y.0, self.k - 0.0001),
            Point3::new(self.x.1, self.y.1, self.k + 0.0001),
        ))
    }
}

// XZRect
pub struct XZRect<T: Material> {
    x: (f64, f64),
    z: (f64, f64),
    k: f64,
    material: T,
}
impl<T: Material> XZRect<T> {
    pub fn new(x: (f64, f64), z: (f64, f64), k: f64, material: T) -> Self {
        XZRect { x, z, k, material }
    }
}
impl<T: Material> Hittable for XZRect<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let t = (self.k - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();
        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }
        let u = (x - self.x.0) / (self.x.1 - self.x.0);
        let v = (z - self.z.0) / (self.z.1 - self.z.0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        Some(RayHit::new(ray, outward_normal, t, &self.material, (u, v)))
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x.0, self.k - 0.0001, self.z.0),
            Point3::new(self.x.1, self.k + 0.0001, self.z.1),
        ))
    }
}

// YZRect
pub struct YZRect<T: Material> {
    y: (f64, f64),
    z: (f64, f64),
    k: f64,
    material: T,
}
impl<T: Material> YZRect<T> {
    pub fn new(y: (f64, f64), z: (f64, f64), k: f64, material: T) -> Self {
        YZRect { y, z, k, material }
    }
}
impl<T: Material> Hittable for YZRect<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let t = (self.k - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin().y() + t * ray.direction().y();
        let z = ray.origin().z() + t * ray.direction().z();
        if y < self.y.0 || y > self.y.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }
        let u = (y - self.y.0) / (self.y.1 - self.y.0);
        let v = (z - self.z.0) / (self.z.1 - self.z.0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        Some(RayHit::new(ray, outward_normal, t, &self.material, (u, v)))
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.y.0, self.k - 0.0001, self.z.0),
            Point3::new(self.y.1, self.k + 0.0001, self.z.1),
        ))
    }
}