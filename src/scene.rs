use crate::{aabb::*, camera::*, geometry::*, material::*, math::*, noise::*, texture::*};
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

pub fn cornell_smoke() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut objects = HittableList::new();

    let red = Material::Lambertian(Color::new(0.65, 0.05, 0.05).into());
    let white = Material::Lambertian(Color::new(0.73, 0.73, 0.73).into());
    let green = Material::Lambertian(Color::new(0.12, 0.45, 0.15).into());
    let light = Material::DiffuseLight(Color::new(7.0, 7.0, 7.0).into());
    let smoke = Material::Isotropic(Color::new(0.0, 0.0, 0.0).into());
    let fog = Material::Isotropic(Color::new(1.0, 1.0, 1.0).into());
    objects.add(Hittable::YZRect((0.0, 555.0), (0.0, 555.0), 555.0, green));
    objects.add(Hittable::YZRect((0.0, 555.0), (0.0, 555.0), 0.0, red));
    objects.add(Hittable::XZRect(
        (113.0, 443.0),
        (127.0, 432.0),
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
    box1 = Hittable::ConstantMedium(Box::new(box1), 0.01, smoke);
    objects.add(box1);
    let mut box2 = Hittable::Cube(Cuboid::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Hittable::RotateY(Box::new(box2), -18.0);
    box2 = Hittable::Translate(Box::new(box2), Vec3::new(130.0, 0.0, 65.0));
    box2 = Hittable::ConstantMedium(Box::new(box2), 0.01, fog);
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

pub fn final_scene() -> (Arc<HittableList>, Arc<Camera>, Color) {
    // World
    let mut boxes1 = HittableList::new();
    let ground = Material::Lambertian(Color::new(0.48, 0.83, 0.53).into());

    const BOXES_PER_SIDE: i32 = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_range_double(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Hittable::Cube(Cuboid::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();
    objects.add(boxes1.to_bhv(0.0, 1.0));

    let light = Material::DiffuseLight(Color::new(7.0, 7.0, 7.0).into());
    objects.add(Hittable::XZRect(
        (123.0, 423.0),
        (147.0, 412.0),
        554.0,
        light,
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Material::Lambertian(Color::new(0.7, 0.3, 0.1).into());
    objects.add(Hittable::MovingSphere(
        (center1, center2),
        50.0,
        moving_sphere_material,
        (0.0, 1.0),
    ));

    objects.add(Hittable::Sphere(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Material::Dielectric(1.5),
    ));
    objects.add(Hittable::Sphere(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Material::Metal(Color::new(0.8, 0.8, 0.9), 10.0),
    ));

    let boundary = Hittable::Sphere(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Material::Dielectric(1.5),
    );
    objects.add(boundary);
    let boundary = Hittable::Sphere(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Material::Dielectric(1.5),
    );
    objects.add(Hittable::ConstantMedium(
        Box::new(boundary),
        0.2,
        Material::Isotropic(Color::new(0.2, 0.4, 0.9).into()),
    ));
    let boundary = Hittable::Sphere(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Material::Dielectric(1.5),
    );
    objects.add(Hittable::ConstantMedium(
        Box::new(boundary),
        0.0001,
        Material::Isotropic(Color::new(1.0, 1.0, 1.0).into()),
    ));

    let emat = Material::Lambertian(Texture::Image(Image::new("earthmap.jpg")));
    objects.add(Hittable::Sphere(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    ));
    let pertext = Texture::Marble(Perlin::new(), 0.1);
    objects.add(Hittable::Sphere(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Material::Lambertian(pertext),
    ));

    let mut boxes2 = HittableList::new();
    let white = Material::Lambertian(Color::new(0.73, 0.73, 0.73).into());
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Hittable::Sphere(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }

    let mut boxes2 = boxes2.to_bhv(0.0, 1.0);
    boxes2 = Hittable::RotateY(Box::new(boxes2), 15.0);
    boxes2 = Hittable::Translate(Box::new(boxes2), Vec3::new(-100.0, 270.0, 395.0));
    objects.add(boxes2);

    // Camera
    let look_from = Point3::new(478.0, 278.0, -600.0);
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
