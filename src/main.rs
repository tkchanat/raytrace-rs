const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 64;
const MAX_DEPTH: i32 = 32;

mod aabb;
mod camera;
mod geometry;
mod material;
mod math;
mod ray;
mod texture;
mod scene;
mod noise;
use math::*;
use ray::*;

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
    let (world, camera) = scene::earth();

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
