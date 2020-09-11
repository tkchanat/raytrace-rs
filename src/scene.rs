use crate::{camera::*, geometry::*, material::*, math::*, noise::*, ray::*, texture::*};
use std::sync::Arc;

pub fn ballz() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut objects = HittableList::new();

    let c1 = Color::new(0.2, 0.3, 0.1);
    let c2 = Color::new(0.9, 0.9, 0.9);
    let ground_material = Material::Lambertian(Texture::Checker(c1, c2));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::Lambertian(albedo.into());
                    let center2 = center + Vec3::new(0.0, random_range_double(0.0, 0.5), 0.0);
                    objects.add(Hittable::MovingSphere(
                        (center, center2),
                        0.2,
                        sphere_material,
                        (0.0, 1.0),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range_double(0.0, 0.5);
                    let sphere_material = Material::Metal(albedo, fuzz);
                    objects.add(Hittable::Sphere(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Material::Dielectric(1.5);
                    objects.add(Hittable::Sphere(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::Dielectric(1.5);
    objects.add(Hittable::Sphere(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1).into());
    objects.add(Hittable::Sphere(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Material::Metal(Color::new(0.7, 0.6, 0.5), 0.0);
    objects.add(Hittable::Sphere(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::UP,
        20.0,
        crate::ASPECT_RATIO,
        aperture,
        focus_distance,
        (0.0, 1.0),
    );

    (
        Arc::new(objects),
        Arc::new(camera),
        Color::new(0.7, 0.8, 1.0),
    )
}

pub fn two_spheres() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut objects = HittableList::new();

    let checker = Material::Lambertian(Texture::Checker(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    ));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker.clone(),
    ));

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::UP,
        20.0,
        crate::ASPECT_RATIO,
        aperture,
        focus_distance,
        (0.0, 1.0),
    );
    (
        Arc::new(objects),
        Arc::new(camera),
        Color::new(0.7, 0.8, 1.0),
    )
}

pub fn two_perlin_spheres() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut objects = HittableList::new();

    let perlin = Perlin::new();
    let checker = Material::Lambertian(Texture::Marble(perlin, 4.0));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker.clone(),
    ));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        checker.clone(),
    ));

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::UP,
        20.0,
        crate::ASPECT_RATIO,
        aperture,
        focus_distance,
        (0.0, 1.0),
    );
    (
        Arc::new(objects),
        Arc::new(camera),
        Color::new(0.7, 0.8, 1.0),
    )
}

pub fn earth() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut objects = HittableList::new();

    let earth_texture = Image::new("earthmap.jpg");
    let earth_material = Material::Lambertian(Texture::Image(earth_texture));
    let globe = Hittable::Sphere(Point3::new(0.0, 0.0, 0.0), 2.0, earth_material);
    objects.add(globe);
    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::UP,
        20.0,
        crate::ASPECT_RATIO,
        aperture,
        focus_distance,
        (0.0, 1.0),
    );
    (
        Arc::new(objects),
        Arc::new(camera),
        Color::new(0.7, 0.8, 1.0),
    )
}

pub fn simple_light() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut objects = HittableList::new();

    let perlin = Perlin::new();
    let checker = Material::Lambertian(Texture::Marble(perlin, 4.0));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker.clone(),
    ));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        checker.clone(),
    ));

    let rect_light = Material::DiffuseLight(Color::new(4.0, 4.0, 4.0).into());
    let sphere_light = Material::DiffuseLight(Color::new(4.0, 4.0, 4.0).into());
    objects.add(Hittable::XYRect((3.0, 5.0), (1.0, 3.0), -2.0, rect_light));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        sphere_light,
    ));

    // Camera
    let look_from = Point3::new(26.0, 3.0, 6.0);
    let look_at = Point3::new(0.0, 2.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::UP,
        20.0,
        crate::ASPECT_RATIO,
        aperture,
        focus_distance,
        (0.0, 1.0),
    );
    (Arc::new(objects), Arc::new(camera), Color::BLACK)
}

pub fn cornell_box() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut objects = HittableList::new();

    let red = Material::Lambertian(Color::new(0.65, 0.05, 0.05).into());
    let white = Material::Lambertian(Color::new(0.73, 0.73, 0.73).into());
    let green = Material::Lambertian(Color::new(0.12, 0.45, 0.15).into());
    let light = Material::DiffuseLight(Color::new(15.0, 15.0, 15.0).into());
    objects.add(Hittable::YZRect((0.0, 555.0), (0.0, 555.0), 555.0, green));
    objects.add(Hittable::YZRect((0.0, 555.0), (0.0, 555.0), 0.0, red));
    objects.add(Hittable::XZRect(
        (213.0, 343.0),
        (227.0, 332.0),
        554.0,
        light,
    ));
    objects.add(Hittable::XZRect(
        (0.0, 555.0),
        (0.0, 555.0),
        0.0,
        white.clone(),
    ));
    objects.add(Hittable::XZRect(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));
    objects.add(Hittable::XYRect(
        (0.0, 555.0),
        (0.0, 555.0),
        555.0,
        white.clone(),
    ));
    let mut box1 = Hittable::Cube(Cuboid::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Hittable::RotateY(Box::new(box1), 15.0);
    box1 = Hittable::Translate(Box::new(box1), Vec3::new(265.0, 0.0, 295.0));
    objects.add(box1);
    let mut box2 = Hittable::Cube(Cuboid::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Hittable::RotateY(Box::new(box2), -18.0);
    box2 = Hittable::Translate(Box::new(box2), Vec3::new(130.0, 0.0, 65.0));
    objects.add(box2);

    // Camera
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let focus_distance = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::UP,
        40.0,
        crate::ASPECT_RATIO,
        aperture,
        focus_distance,
        (0.0, 1.0),
    );
    (Arc::new(objects), Arc::new(camera), Color::BLACK)
}
