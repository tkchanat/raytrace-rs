extern crate image;
use crate::{math::*, noise::*};
use image::GenericImageView;

// Texutre
#[derive(Clone)]
pub enum Texture {
    SolidColor(Color),
    Checker(Color, Color),
    Perlin(Perlin, f64),
    Marble(Perlin, f64),
    Image(Image),
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
        Texture::Image(image) => {
            if image.data.is_empty() {
                return Color::CYAN;
            }
            let u = clamp(u, 0.0, 1.0);
            let v = 1.0 - clamp(v, 0.0, 1.0);
            let mut i = (u * image.width as f64) as u32;
            let mut j = (v * image.height as f64) as u32;
            if i >= image.width {
                i = image.width - 1;
            }
            if j >= image.height {
                j = image.height - 1;
            }
            let color_scale = 1.0 / 255.0;
            let pixel = j as usize * image.bytes_per_scanline + i as usize * Image::BYTES_PER_PIXEL;
            let r = color_scale * image.data[pixel] as f64;
            let g = color_scale * image.data[pixel+1] as f64;
            let b = color_scale * image.data[pixel+2] as f64;
            Color::new(r, g, b)
        }
        _ => Color::BLACK,
    }
}

#[derive(Clone)]
pub struct Image {
    data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub bytes_per_scanline: usize,
}
impl Image {
    const BYTES_PER_PIXEL: usize = 3;

    pub fn new(file_name: &str) -> Self {
        let components_per_pixel = Image::BYTES_PER_PIXEL;
        let img = image::open(file_name).unwrap();
        let dimension = img.dimensions();
        eprintln!("dimensions {:?}, color: {:?}", dimension, img.color());
        let data = img.into_rgb().into_raw();
        assert_ne!(data.len(), 0, "Could not load texture!");
        Image {
            data,
            width: dimension.0,
            height: dimension.1,
            bytes_per_scanline: Image::BYTES_PER_PIXEL * dimension.0 as usize,
        }
    }
}
