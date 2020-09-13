use crate::{math::*, ray::*, texture::*};

#[derive(Clone)]
pub enum Material {
    Lambertian(Texture),
    Metal(Color, f64),
    Dielectric(f64),
    DiffuseLight(Texture),
    Isotropic(Texture),
}

pub fn scatter(material: &Material, ray: &Ray, hit: &RayHit) -> Option<(Ray, Color)> {
    match material {
        Material::Lambertian(texture) => {
            let scatter_direction = *hit.normal() + Vec3::random_unit_vector();
            let scattered = Ray::new(*hit.point(), scatter_direction, None);
            let attenuation = sample_texture(&texture, hit.uv().0, hit.uv().1, hit.point());
            Some((scattered, attenuation))
        }
        Material::Metal(albedo, roughness) => {
            let reflected = reflect(&normalize(ray.direction()), hit.normal());
            let scattered = Ray::new(
                *hit.point(),
                reflected + Vec3::random_in_unit_sphere() * *roughness,
                None,
            );
            let attenuation = *albedo;
            if dot(scattered.direction(), hit.normal()) > 0.0 {
                Some((scattered, attenuation))
            } else {
                None
            }
        }
        Material::Dielectric(refractive_index) => {
            let attenuation = Color::WHITE;
            let etai_over_etat = if hit.front_face() {
                1.0 / refractive_index
            } else {
                *refractive_index
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
        Material::Isotropic(texture) => {
            let scattered = Ray::new(
                *hit.point(),
                Vec3::random_in_unit_sphere(),
                Some(ray.time()),
            );
            let attenuation = sample_texture(texture, hit.uv().0, hit.uv().1, hit.point());
            Some((scattered, attenuation))
        }
        _ => None,
    }
}
pub fn emitted(material: &Material, u: f64, v: f64, p: &Point3) -> Color {
    match material {
        Material::DiffuseLight(texture) => sample_texture(&texture, u, v, p),
        _ => Color::BLACK,
    }
}
