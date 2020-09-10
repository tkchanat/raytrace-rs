use crate::{camera::*, geometry::*, material::*, math::*, noise::*, ray::*, texture::*};
use std::sync::Arc;

pub fn ballz() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut objects = HittableList::new();

    let c1 = Color::new(0.2, 0.3, 0.1);
    let c2 = Color::new(0.9, 0.9, 0.9);
    let ground_material = Lambertian::from_texture(Texture::Checker(c1, c2));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

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
                    let sphere_material = Lambertian::from_color(albedo);
                    let center2 = center + Vec3::new(0.0, random_range_double(0.0, 0.5), 0.0);
                    objects.add(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_range_double(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    objects.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    objects.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::from_color(Color::new(0.4, 0.2, 0.1));
    objects.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    objects.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

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

    let checker = Lambertian::from_texture(Texture::Checker(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker.clone(),
    )));

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
    let checker = Lambertian::from_texture(Texture::Marble(perlin, 4.0));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        checker.clone(),
    )));

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
    let earth_material = Lambertian::from_texture(Texture::Image(earth_texture));
    let globe = Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_material));
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
    let checker = Lambertian::from_texture(Texture::Marble(perlin, 4.0));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        checker.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        checker.clone(),
    )));

    let rect_light = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));
    let sphere_light = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));
    objects.add(Box::new(XYRect::new(
        (3.0, 5.0),
        (1.0, 3.0),
        -2.0,
        rect_light,
    )));
    objects.add(Box::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        sphere_light,
    )));

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

    let red = Lambertian::from_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::from_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::from_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));
    objects.add(Box::new(YZRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green)));
    objects.add(Box::new(YZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red)));
    objects.add(Box::new(XZRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light)));
    objects.add(Box::new(XZRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone())));
    objects.add(Box::new(XZRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone())));
    objects.add(Box::new(XYRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone())));

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