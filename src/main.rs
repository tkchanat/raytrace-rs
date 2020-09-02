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
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64 * 255.0;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64 * 255.0;
            let b = 0.25 * 255.0;
            println!("{} {} {}", r as i32, g as i32, b as i32);
        }
    }
}
