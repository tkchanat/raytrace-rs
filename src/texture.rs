use crate::math::*;

// Texutre
#[derive(Clone)]
pub enum Texture {
    SolidColor(Color),
    Checker(Color, Color),
}
pub fn sample_texture(texture: &Texture, u: f64, v: f64, p: &Point3) -> Color {
    match texture {
        Texture::SolidColor(color_value) => *color_value,
        Texture::Checker(odd, even) => {
            let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
            if sines < 0.0 {
                *odd
            } else {
                *even
            }
        }
        _ => Color::BLACK,
    }
}
