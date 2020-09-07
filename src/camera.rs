use crate::{math::*, ray::*};

// Camera
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    shutter_time: (f64, f64), // shutter open/close time
}
impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
        shutter_time: (f64, f64),
    ) -> Self {
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(&(look_from - look_at));
        let u = normalize(&cross(&up, &w));
        let v = cross(&w, &u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;
        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            shutter_time,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let ray_direction = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * ray_direction.x() + self.v * ray_direction.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            Some(random_range_double(self.shutter_time.0, self.shutter_time.1)),
        )
    }
}
