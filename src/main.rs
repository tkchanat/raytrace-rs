mod geometry;
mod material;
mod math;
mod ray;
use geometry::*;
use material::*;
use math::*;
use ray::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 32;
const MAX_DEPTH: i32 = 50;

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    fn new() -> Self {
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const FOCAL_LENGTH: f64 = 1.0;
        let origin = Point3::from(0.0, 0.0, 0.0);
        let horizontal = Vec3::from(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::from(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, FOCAL_LENGTH);
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

fn write_color(color: &Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = 255.0 * clamp((color.x() * scale).sqrt(), 0.0, 1.0);
    let g = 255.0 * clamp((color.y() * scale).sqrt(), 0.0, 1.0);
    let b = 255.0 * clamp((color.z() * scale).sqrt(), 0.0, 1.0);
    println!("{} {} {}", r as i32, g as i32, b as i32);
}

fn ray_color<T>(ray: &Ray, world: &HittableList<T>, depth: i32) -> Color
where
    T: Hittable,
{
    if depth <= 0 {
        return Color::BLACK;
    }
    if let Some(hit) = world.hit(ray, 0.001, INIFINITY) {
        let (scattered, attenuation) = hit.material.scatter(ray, &hit);
        return attenuation * ray_color(&scattered, world, depth - 1);
    }
    let unit_direction = normalize(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Header
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    // World
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Color::from(0.5, 0.5, 0.5));
    let material_center = Lambertian::new(Color::from(0.5, 0.5, 0.5));
    // let material_left   = Metal(color(0.8, 0.8, 0.8));
    // let material_right  = Metal(color(0.8, 0.6, 0.2));
    world.add(Sphere::from(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
        &material_center,
    ));
    world.add(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    ));

    // Camera
    let camera = Camera::new();

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::BLACK;
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
