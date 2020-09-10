use crate::{math::*, noise::*};

// Texutre
#[derive(Clone)]
pub enum Texture {
    SolidColor(Color),
    Checker(Color, Color),
    Perlin(Perlin, f64),
    Marble(Perlin, f64),
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
        Texture::Perlin(noise, scale) => Color::WHITE * 0.5 * (1.0 + noise.noise(&(*p * *scale))),
        Texture::Marble(noise, scale) => {
            Color::WHITE * 0.5 * (1.0 + (scale * p.z() + 10.0 * noise.turbulence(&p, 7)).sin())
        }
        _ => Color::BLACK,
    }
}
