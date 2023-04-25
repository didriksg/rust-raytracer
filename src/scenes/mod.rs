use rand::random;

use crate::data_structs::vec3::{Color, Point3, Vec3};
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::Material;
use crate::materials::metal::Metal;
use crate::objects::HittableList;
use crate::objects::moving_sphere::MovingSphere;
use crate::objects::sphere::Sphere;

pub fn movable_one_weekend() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random::<f64>();
            let center = Point3::new(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
            let mut is_movable = false;

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Material::Lambertian(Lambertian::new(albedo));
                    is_movable = true;

                    material
                } else if choose_material < 0.95 {
                    let albedo = Color::random_with_limits(0.5, 1.0);
                    let fuzz = random::<f64>() / 2.0;

                    Material::Metal(Metal::new(albedo, fuzz))
                } else {
                    Material::Dielectric(Dielectric::new(1.5))
                };

                if is_movable {
                    let end_center = center + Vec3::new(0.0, random::<f64>() / 2.0, 0.0);
                    world.add(MovingSphere::new(center, end_center, 0.0, 1.0, 0.2, material));
                } else {
                    world.add(Sphere::new(center, 0.2, material));
                };

            }
        }
    }

    let material_dielectric = Material::Dielectric(Dielectric::new(1.5));
    let material_lambertian = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material_metal = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_dielectric));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_lambertian));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_metal));
    world.add(Sphere::new(Point3::new(4.0, 0.7, 2.5), 0.7, material_dielectric));
    world.add(Sphere::new(Point3::new(4.0, 0.7, 2.5), -0.65, material_dielectric));

    world
}

pub fn one_weekend_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand::random::<f64>();
            let center = Point3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();

                    Material::Lambertian(Lambertian::new(albedo))
                } else if choose_material < 0.95 {
                    let albedo = Color::random_with_limits(0.5, 1.0);
                    let fuzz = rand::random::<f64>() / 2.0;

                    Material::Metal(Metal::new(albedo, fuzz))
                } else {
                    Material::Dielectric(Dielectric::new(1.5))
                };

                world.add(Sphere::new(center, 0.2, material));
            }
        }
    }

    let material_dielectric = Material::Dielectric(Dielectric::new(1.5));
    let material_lambertian = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material_metal = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_dielectric));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_lambertian));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_metal));
    world.add(Sphere::new(Point3::new(4.0, 0.7, 2.5), 0.7, material_dielectric));
    world.add(Sphere::new(Point3::new(4.0, 0.7, 2.5), -0.65, material_dielectric));

    world
}