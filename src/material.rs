use crate::{math::*, ray::*};

// Traits
pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<(Ray, Color)>;
}

// Lambertian
pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<(Ray, Color)> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(hit.point, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

// Metal
pub struct Metal {
    albedo: Color,
    roughness: f64,
}
impl Metal {
    pub fn new(albedo: Color, roughness: f64) -> Self {
        Metal { albedo, roughness }
    }
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<(Ray, Color)> {
        let reflected = reflect(&normalize(ray.direction()), &hit.normal);
        let scattered = Ray::new(hit.point, reflected + Vec3::random_in_unit_sphere() * self.roughness);
        let attenuation = self.albedo;
        if dot(scattered.direction(), &hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
