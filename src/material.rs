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
        let scattered = Ray::new(
            hit.point,
            reflected + Vec3::random_in_unit_sphere() * self.roughness,
        );
        let attenuation = self.albedo;
        if dot(scattered.direction(), &hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

// Dielectric
pub struct Dielectric {
    refractive_index: f64,
}
impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<(Ray, Color)> {
        let attenuation = Color::WHITE;
        let etai_over_etat = if hit.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = normalize(ray.direction());

        let cos_theta = dot(&(-unit_direction), &hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        // Total Internal Reflection
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &hit.normal);
            let scattered = Ray::new(hit.point, reflected);
            Some((scattered, attenuation))
        } else if random_double() < reflect_prob {
            let reflected = reflect(&unit_direction, &hit.normal);
            let scattered = Ray::new(hit.point, reflected);
            Some((scattered, attenuation))
        } else {
            let refracted = refract(&unit_direction, &hit.normal, etai_over_etat);
            let scattered = Ray::new(hit.point, refracted);
            Some((scattered, attenuation))
        }
    }
}
