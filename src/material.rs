use crate::{math::*, ray::*};

// Traits
pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> (Ray, Color);
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
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> (Ray, Color) {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(hit.point, scatter_direction);
        let attenuation = self.albedo;
        (scattered, attenuation)
    }
}
