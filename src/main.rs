mod math;
use math::*;

fn write_color(color: &Color) {
    println!("{} {} {}", (color.x() * 255.0) as i32, (color.y() * 255.0) as i32, (color.z() * 255.0) as i32);
}

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    // Header
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let color = Color::from(i as f64 / (IMAGE_WIDTH - 1) as f64, j as f64 / (IMAGE_HEIGHT - 1) as f64, 0.25);
            write_color(&color);
        }
    }
}
