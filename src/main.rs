mod geometry;
mod material;
mod math;
mod ray;
use geometry::*;
use material::*;
use math::*;
use ray::*;
use std::sync::Arc;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 64;
const MAX_DEPTH: i32 = 8;

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}
// unsafe impl Sync for Camera {}
// unsafe impl Send for Camera {}
impl Camera {
    fn new(
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
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
        }
    }
    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let ray_direction = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * ray_direction.x() + self.v * ray_direction.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range_double(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn write_color(color: &Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = 255.0 * clamp((color.x() * scale).sqrt(), 0.0, 1.0);
    let g = 255.0 * clamp((color.y() * scale).sqrt(), 0.0, 1.0);
    let b = 255.0 * clamp((color.z() * scale).sqrt(), 0.0, 1.0);
    println!("{} {} {}", r as i32, g as i32, b as i32);
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::BLACK;
    }
    if let Some(hit) = world.hit(ray, 0.001, INIFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::BLACK;
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
    let world = Arc::new(random_scene());

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.1;
    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        Vec3::UP,
        20.0,
        ASPECT_RATIO,
        aperture,
        focus_distance,
    ));

    // Render
    let start_time = chrono::Local::now();
    for j in (0..IMAGE_HEIGHT).rev() {
        let mut threads = Vec::new();
        eprintln!("Scan line {} / {}", IMAGE_HEIGHT - j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let camera = camera.clone();
            let world = world.clone();
            let handle = std::thread::spawn(move || -> Color {
                let mut pixel_color = Color::BLACK;
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH)
                }
                pixel_color
            });
            threads.push(handle);
        }
        for thread in threads {
            let pixel_color = thread.join().unwrap();
            write_color(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    let end_time = chrono::Local::now();
    eprintln!(
        "Elapsed Time: {:?}",
        end_time.signed_duration_since(start_time)
    );
}
