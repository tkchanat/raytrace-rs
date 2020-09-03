mod math;
mod ray;
use math::*;
use ray::*;

fn write_color(color: &Color) {
    println!(
        "{} {} {}",
        (color.x() * 255.0) as i32,
        (color.y() * 255.0) as i32,
        (color.z() * 255.0) as i32
    );
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = *ray.origin() - center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(&oc, ray.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn ray_color(r: Ray) -> Color {
    let sphere_position = Point3::from(0.0, 0.0, -1.0);
    let t = hit_sphere(sphere_position, 0.5, &r);
    if t > 0.0 {
        let normal = normalize(&(r.at(t) - sphere_position));
        return Color::from(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) / 2.0;
    }
    let unit_direction = normalize(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Header
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::from(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(ray);
            write_color(&pixel_color);
        }
    }
}
