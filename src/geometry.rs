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
    Cube(Cuboid),
    BHVNode(Box<BHVNode>),
    Translate(Box<Hittable>, Vec3),
    RotateY(Box<Hittable>, f64),
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
                    point,
                    distance,
                    &material,
                    outward_normal,
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
                    point,
                    distance,
                    &material,
                    outward_normal,
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
            Some(RayHit::new(
                ray,
                ray.at(t),
                t,
                &material,
                outward_normal,
                (u, v),
            ))
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
            Some(RayHit::new(
                ray,
                ray.at(t),
                t,
                &material,
                outward_normal,
                (u, v),
            ))
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
            Some(RayHit::new(
                ray,
                ray.at(t),
                t,
                &material,
                outward_normal,
                (u, v),
            ))
        }
        Hittable::Cube(cuboid) => cuboid.sides().hit(ray, t_min, t_max),
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
        Hittable::Translate(object, offset) => {
            let moved_r = Ray::new(*ray.origin() - *offset, *ray.direction(), Some(ray.time()));
            if let Some(hit) = ray_cast(object, &moved_r, t_min, t_max) {
                return Some(RayHit::new(
                    &moved_r,
                    *hit.point() + *offset,
                    hit.distance(),
                    hit.material(),
                    *hit.normal(),
                    hit.uv(),
                ));
            }
            None
        }
        Hittable::RotateY(object, degree) => {
            let mut origin = *ray.origin();
            let mut direction = *ray.direction();
            let radians = degrees_to_radians(*degree);
            let sin_theta = radians.sin();
            let cos_theta = radians.cos();
            origin[0] = cos_theta * ray.origin()[0] - sin_theta * ray.origin()[2];
            origin[2] = sin_theta * ray.origin()[0] + cos_theta * ray.origin()[2];
            direction[0] = cos_theta * ray.direction()[0] - sin_theta * ray.direction()[2];
            direction[2] = sin_theta * ray.direction()[0] + cos_theta * ray.direction()[2];

            let rotated_r = Ray::new(origin, direction, Some(ray.time()));
            match ray_cast(object, &rotated_r, t_min, t_max) {
                Some(hit) => {
                    let mut p = *hit.point();
                    let mut normal = *hit.normal();
                    p[0] = cos_theta * hit.point()[0] + sin_theta * hit.point()[2];
                    p[2] = -sin_theta * hit.point()[0] + cos_theta * hit.point()[2];
                    normal[0] = cos_theta * hit.normal()[0] + sin_theta * hit.normal()[2];
                    normal[2] = -sin_theta * hit.normal()[0] + cos_theta * hit.normal()[2];
                    Some(RayHit::new(
                        &rotated_r,
                        p,
                        (p - origin).length(),
                        hit.material(),
                        normal,
                        hit.uv(),
                    ))
                }
                None => None,
            }
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
        Hittable::Cube(cuboid) => Some(AABB::new(*cuboid.min(), *cuboid.max())),
        Hittable::BHVNode(node) => Some(node.bounding_box().clone()),
        Hittable::Translate(object, offset) => match get_bounding_box(object, t0, t1) {
            Some(bounding_box) => Some(AABB::new(
                *bounding_box.min() + *offset,
                *bounding_box.max() + *offset,
            )),
            None => None,
        },
        Hittable::RotateY(object, degree) => {
            let radians = degrees_to_radians(*degree);
            let sin_theta = radians.sin();
            let cos_theta = radians.cos();
            match get_bounding_box(object, 0.0, 1.0) {
                Some(bounding_box) => {
                    let mut min = Point3::new(INIFINITY, INIFINITY, INIFINITY);
                    let mut max = Point3::new(-INIFINITY, -INIFINITY, -INIFINITY);
                    for i in 0..2 {
                        for j in 0..2 {
                            for k in 0..2 {
                                let x = i as f64 * bounding_box.max().x()
                                    + (1 - i) as f64 * bounding_box.min().x();
                                let y = j as f64 * bounding_box.max().y()
                                    + (1 - j) as f64 * bounding_box.min().y();
                                let z = k as f64 * bounding_box.max().z()
                                    + (1 - k) as f64 * bounding_box.min().z();
                                let newx = cos_theta * x + sin_theta * z;
                                let newz = -sin_theta * x + cos_theta * z;
                                let tester = Vec3::new(newx, y, newz);
                                for c in 0..3 {
                                    min[c] = min[c].min(tester[c]);
                                    max[c] = max[c].max(tester[c]);
                                }
                            }
                        }
                    }
                    Some(AABB::new(min, max))
                }
                None => None,
            }
        }
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

pub struct Cuboid {
    min: Point3,
    max: Point3,
    sides: HittableList,
}
impl Cuboid {
    pub fn new(min: Point3, max: Point3, material: Material) -> Self {
        let mut sides = HittableList::new();
        sides.add(Hittable::XYRect(
            (min.x(), max.x()),
            (min.y(), max.y()),
            max.z(),
            material.clone(),
        ));
        sides.add(Hittable::XYRect(
            (min.x(), max.x()),
            (min.y(), max.y()),
            min.z(),
            material.clone(),
        ));
        sides.add(Hittable::XZRect(
            (min.x(), max.x()),
            (min.z(), max.z()),
            max.y(),
            material.clone(),
        ));
        sides.add(Hittable::XZRect(
            (min.x(), max.x()),
            (min.z(), max.z()),
            min.y(),
            material.clone(),
        ));
        sides.add(Hittable::YZRect(
            (min.y(), max.y()),
            (min.z(), max.z()),
            max.x(),
            material.clone(),
        ));
        sides.add(Hittable::YZRect(
            (min.y(), max.y()),
            (min.z(), max.z()),
            min.x(),
            material.clone(),
        ));
        Cuboid { min, max, sides }
    }
    pub fn min(&self) -> &Point3 {
        &self.min
    }
    pub fn max(&self) -> &Point3 {
        &self.max
    }
    pub fn sides(&self) -> &HittableList {
        &self.sides
    }
}
