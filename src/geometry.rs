use crate::{aabb::*, material::*, math::*, ray::*};

// Helper function
fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}

// Hittable
pub enum Hittable {
    Sphere(Point3, f64, Material),
    MovingSphere((Point3, Point3), f64, Material, (f64, f64)),
    XYRect((f64, f64), (f64, f64), f64, Material),
    YZRect((f64, f64), (f64, f64), f64, Material),
    XZRect((f64, f64), (f64, f64), f64, Material),
    BHVNode(Box<BHVNode>),
}
pub fn ray_cast<'a>(obj: &'a Hittable, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit<'a>> {
    match obj {
        Hittable::Sphere(center, radius, material) => {
            let oc = *ray.origin() - *center;
            let a = ray.direction().length_squared();
            let half_b = dot(&oc, ray.direction());
            let c = oc.length_squared() - radius * radius;
            let discriminant = half_b * half_b - a * c;
            let create_hit_by_distance = |distance: f64| -> Option<RayHit> {
                let point = ray.at(distance);
                let outward_normal = (point - *center) / *radius;
                return Some(RayHit::new(
                    ray,
                    outward_normal,
                    distance,
                    &material,
                    get_sphere_uv(&((point - *center) / *radius)),
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
        Hittable::MovingSphere((center0, center1), radius, material, (time0, time1)) => {
            let center = |time| -> Point3 {
                *center0 + (*center1 - *center0) * ((time - *time0) / (*time1 - *time0))
            };
            let center = center(ray.time());
            let oc = *ray.origin() - center;
            let a = ray.direction().length_squared();
            let half_b = dot(&oc, ray.direction());
            let c = oc.length_squared() - radius * radius;
            let discriminant = half_b * half_b - a * c;
            let create_hit_by_distance = |distance: f64| -> Option<RayHit> {
                let point = ray.at(distance);
                let outward_normal = (point - center) / *radius;
                return Some(RayHit::new(
                    ray,
                    outward_normal,
                    distance,
                    &material,
                    get_sphere_uv(&((point - center) / *radius)),
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
        Hittable::XYRect((x0, x1), (y0, y1), k, material) => {
            let t = (k - ray.origin().z()) / ray.direction().z();
            if t < t_min || t > t_max {
                return None;
            }
            let x = ray.origin().x() + t * ray.direction().x();
            let y = ray.origin().y() + t * ray.direction().y();
            if x < *x0 || x > *x1 || y < *y0 || y > *y1 {
                return None;
            }
            let u = (x - *x0) / (*x1 - *x0);
            let v = (y - *y0) / (*y1 - *y0);
            let outward_normal = Vec3::new(0.0, 0.0, 1.0);
            Some(RayHit::new(ray, outward_normal, t, &material, (u, v)))
        }
        Hittable::XZRect((x0, x1), (z0, z1), k, material) => {
            let t = (k - ray.origin().y()) / ray.direction().y();
            if t < t_min || t > t_max {
                return None;
            }
            let x = ray.origin().x() + t * ray.direction().x();
            let z = ray.origin().z() + t * ray.direction().z();
            if x < *x0 || x > *x1 || z < *z0 || z > *z1 {
                return None;
            }
            let u = (x - *x0) / (*x1 - *x0);
            let v = (z - *z0) / (*z1 - *z0);
            let outward_normal = Vec3::new(0.0, 1.0, 0.0);
            Some(RayHit::new(ray, outward_normal, t, &material, (u, v)))
        }
        Hittable::YZRect((y0, y1), (z0, z1), k, material) => {
            let t = (k - ray.origin().x()) / ray.direction().x();
            if t < t_min || t > t_max {
                return None;
            }
            let y = ray.origin().y() + t * ray.direction().y();
            let z = ray.origin().z() + t * ray.direction().z();
            if y < *y0 || y > *y1 || z < *z0 || z > *z1 {
                return None;
            }
            let u = (y - *y0) / (*y1 - *y0);
            let v = (z - *z0) / (*z1 - *z0);
            let outward_normal = Vec3::new(1.0, 0.0, 0.0);
            Some(RayHit::new(ray, outward_normal, t, &material, (u, v)))
        }
        Hittable::BHVNode(node) => {
            if node.hit_check(ray, t_min, t_max) {
                return None;
            }
            let hit_left = ray_cast(node.left(), ray, t_min, t_max);
            let t = match &hit_left {
                Some(hit) => hit.distance(),
                None => t_min,
            };
            let hit_right = ray_cast(node.right(), ray, t, t_max);
            if hit_left.is_some() {
                return hit_left;
            } else if hit_right.is_some() {
                return hit_right;
            }
            None
        }
        _ => panic!("What are you expecting me to do with this hittable object!??"),
    }
}
pub fn get_bounding_box(obj: &Hittable, t0: f64, t1: f64) -> Option<AABB> {
    match obj {
        Hittable::Sphere(center, radius, material) => Some(AABB::new(
            *center - Vec3::new(*radius, *radius, *radius),
            *center + Vec3::new(*radius, *radius, *radius),
        )),
        Hittable::MovingSphere((center0, center1), radius, material, (time0, time1)) => {
            let box0 = AABB::new(
                *center0 - Vec3::new(*radius, *radius, *radius),
                *center0 + Vec3::new(*radius, *radius, *radius),
            );
            let box1 = AABB::new(
                *center1 - Vec3::new(*radius, *radius, *radius),
                *center1 + Vec3::new(*radius, *radius, *radius),
            );
            Some(surrounding_box(&box0, &box1))
        }
        Hittable::XYRect((x0, x1), (y0, y1), k, material) => Some(AABB::new(
            Point3::new(*x0, *y0, k - 0.0001),
            Point3::new(*x1, *y1, k + 0.0001),
        )),
        Hittable::XZRect((x0, x1), (z0, z1), k, material) => Some(AABB::new(
            Point3::new(*x0, k - 0.0001, *z0),
            Point3::new(*x1, k + 0.0001, *z1),
        )),
        Hittable::YZRect((y0, y1), (z0, z1), k, material) => Some(AABB::new(
            Point3::new(k - 0.0001, *y0, *z0),
            Point3::new(k + 0.0001, *y1, *z1),
        )),
        Hittable::BHVNode(node) => Some(node.bounding_box().clone()),
        _ => panic!("What are you expecting me to do with this hittable object!??"),
    }
}

// HittableList
pub struct HittableList {
    objects: Vec<Hittable>,
}
impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Hittable) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let mut closest_hit = None;
        let mut closest_distance = t_max;
        for object in &self.objects {
            if let Some(hit) = ray_cast(object, ray, t_min, closest_distance) {
                closest_distance = hit.distance();
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
            match get_bounding_box(object, t0, t1) {
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
    pub fn objects(self) -> Vec<Hittable> {
        self.objects
    }
}

// Cube
// pub struct Cube {
//     box_min: Point3,
//     box_max: Point3,
//     sides: HittableList,
// }
// impl Cube {
//     pub fn new(p0: Point3, p1: Point3, material: Box::<dyn Material>) -> Self {
//         let mut sides = HittableList::new();
//         sides.add(Box::new(XYRect::new(
//             (p0.x(), p1.x()),
//             (p0.y(), p1.y()),
//             p1.z(),
//             material,
//         )));
//         // sides.add(Box::new(XYRect::new(
//         //     (p0.x(), p1.x()),
//         //     (p0.y(), p1.y()),
//         //     p0.z(),
//         //     material.clone(),
//         // )));
//         // sides.add(Box::new(XZRect::new(
//         //     (p0.x(), p1.x()),
//         //     (p0.z(), p1.z()),
//         //     p1.y(),
//         //     material.clone(),
//         // )));
//         // sides.add(Box::new(XZRect::new(
//         //     (p0.x(), p1.x()),
//         //     (p0.z(), p1.z()),
//         //     p0.y(),
//         //     material.clone(),
//         // )));
//         // sides.add(Box::new(YZRect::new(
//         //     (p0.y(), p1.y()),
//         //     (p0.z(), p1.z()),
//         //     p1.x(),
//         //     material.clone(),
//         // )));
//         // sides.add(Box::new(YZRect::new(
//         //     (p0.y(), p1.y()),
//         //     (p0.z(), p1.z()),
//         //     p0.x(),
//         //     material.clone(),
//         // )));
//         Cube {
//             box_min: p0,
//             box_max: p1,
//             sides,
//         }
//     }
// }
// impl Hittable for Cube {
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
//         let t = (self.k - ray.origin().x()) / ray.direction().x();
//         if t < t_min || t > t_max {
//             return None;
//         }
//         let y = ray.origin().y() + t * ray.direction().y();
//         let z = ray.origin().z() + t * ray.direction().z();
//         if y < self.*y0 || y > self.*y1 || z < self.*z0 || z > self.*z1 {
//             return None;
//         }
//         let u = (y - self.*y0) / (self.*y1 - self.*y0);
//         let v = (z - self.*z0) / (self.*z1 - self.*z0);
//         let outward_normal = Vec3::new(1.0, 0.0, 0.0);
//         Some(RayHit::new(ray, outward_normal, t, &self.material, (u, v)))
//     }
//     fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
//         Some(AABB::new(
//             Point3::new(self.*y0, self.k - 0.0001, self.*z0),
//             Point3::new(self.*y1, self.k + 0.0001, self.*z1),
//         ))
//     }
// }
