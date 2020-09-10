use crate::{math::*, ray::*, texture::*};

// Traits
pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<(Ray, Color)>;
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::BLACK
    }
}

// Lambertian
#[derive(Clone)]
pub struct Lambertian {
    albedo: Texture,
}
impl Lambertian {
    pub fn from_color(albedo: Color) -> Self {
        Lambertian {
            albedo: Texture::SolidColor(albedo),
        }
    }
    pub fn from_texture(texture: Texture) -> Self {
        Lambertian { albedo: texture }
    }
}
impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<(Ray, Color)> {
        let scatter_direction = *hit.normal() + Vec3::random_unit_vector();
        let scattered = Ray::new(*hit.point(), scatter_direction, None);
        let attenuation = sample_texture(&self.albedo, hit.uv().0, hit.uv().1, &&hit.point());
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
        let reflected = reflect(&normalize(ray.direction()), hit.normal());
        let scattered = Ray::new(
            *hit.point(),
            reflected + Vec3::random_in_unit_sphere() * self.roughness,
            None,
        );
        let attenuation = self.albedo;
        if dot(scattered.direction(), hit.normal()) > 0.0 {
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
        let etai_over_etat = if hit.front_face() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = normalize(ray.direction());

        let cos_theta = dot(&(-unit_direction), hit.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        // Total Internal Reflection
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, hit.normal());
            let scattered = Ray::new(*hit.point(), reflected, None);
            Some((scattered, attenuation))
        } else if random_double() < reflect_prob {
            let reflected = reflect(&unit_direction, hit.normal());
            let scattered = Ray::new(*hit.point(), reflected, None);
            Some((scattered, attenuation))
        } else {
            let refracted = refract(&unit_direction, hit.normal(), etai_over_etat);
            let scattered = Ray::new(*hit.point(), refracted, None);
            Some((scattered, attenuation))
        }
    }
}

// Diffuse Light
pub struct DiffuseLight {
    emit: Texture,
}
impl DiffuseLight {
    pub fn from_color(emit: Color) -> Self {
        DiffuseLight {
            emit: Texture::SolidColor(emit),
        }
    }
    pub fn from_texture(texture: Texture) -> Self {
        DiffuseLight { emit: texture }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, hit: &RayHit) -> Option<(Ray, Color)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        sample_texture(&self.emit, u, v, p)
    }
}
