use rand::random;

use crate::data_structs::vec3::{Color, Point3, Vec3};
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::Material;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::metal::Metal;
use crate::materials::textures::checker_texture::CheckerTexture;
use crate::materials::textures::image_texture::ImageTexture;
use crate::materials::textures::perlin::{NoiseTexture};
use crate::objects::hittables::cube::Cube;
use crate::objects::hittables::HittableList;
use crate::objects::hittables::moving_sphere::MovingSphere;
use crate::objects::hittables::rectangles::xy_rectangle::XyRectangle;
use crate::objects::hittables::rectangles::xz_rectangle::XzRectangle;
use crate::objects::hittables::rectangles::yz_rectangle::YzRectangle;
use crate::objects::hittables::sphere::Sphere;

fn movable_one_weekend() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Lambertian::new_texture(
        CheckerTexture::new_from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    )));

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random::<f64>();
            let center = Point3::new(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
            let mut is_movable = false;

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Material::Lambertian(Lambertian::from_color(albedo));
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
    let material_lambertian = Material::Lambertian(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    let material_metal = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_dielectric));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_lambertian));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_metal));
    // world.add(Sphere::new(Point3::new(4.0, 0.7, 2.5), 0.7, material_dielectric.clone()));
    // world.add(Sphere::new(Point3::new(4.0, 0.7, 2.5), -0.65, material_dielectric));

    world
}

fn one_weekend_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Lambertian::from_color(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand::random::<f64>();
            let center = Point3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();

                    Material::Lambertian(Lambertian::from_color(albedo))
                } else if choose_material < 0.95 {
                    let albedo = Color::random_with_limits(0.5, 1.0);
                    let fuzz = random::<f64>() / 2.0;

                    Material::Metal(Metal::new(albedo, fuzz))
                } else {
                    Material::Dielectric(Dielectric::new(1.5))
                };

                world.add(Sphere::new(center, 0.2, material));
            }
        }
    }

    let material_dielectric = Material::Dielectric(Dielectric::new(1.5));
    let material_lambertian = Material::Lambertian(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    let material_metal = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_dielectric.clone()));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_lambertian));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_metal));
    world.add(Sphere::new(Point3::new(4.0, 0.7, 2.5), 0.7, material_dielectric.clone()));
    world.add(Sphere::new(Point3::new(4.0, 0.7, 2.5), -0.65, material_dielectric));

    world
}

fn two_textured_spheres_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker_material = Lambertian::new_texture(
        CheckerTexture::new_from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, Material::Lambertian(checker_material.clone())));
    world.add(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, Material::Lambertian(checker_material)));

    world
}

fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();

    let perlin_texture = Lambertian::new_texture(NoiseTexture::new(4.0));

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian(perlin_texture.clone())));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Material::Lambertian(perlin_texture)));

    world
}

fn earth() -> HittableList {
    let mut world = HittableList::new();

    let earth_texture = Lambertian::new_texture(ImageTexture::new("src/image_textures/earthmap.jpg"));
    world.add(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, Material::Lambertian(earth_texture)));

    world
}

fn diffuse_light() -> HittableList {
    let mut world = two_perlin_spheres();

    let light_strength = 4.0;
    let diffuse_light = DiffuseLight::from_color(Color::new(light_strength, light_strength, light_strength));
    world.add(XyRectangle::new(3.0, 5.0, 1.0, 3.0, -2.0, Material::DiffuseLight(diffuse_light.clone())));
    world.add(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, Material::DiffuseLight(diffuse_light)));

    world
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Lambertian::from_color(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::from_color(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::from_color(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));

    world.add(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, Material::Lambertian(green.clone())));
    world.add(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, Material::Lambertian(red.clone())));
    world.add(XzRectangle::new(213.0, 343.0, 227.0, 332.0, 554.0, Material::DiffuseLight(light.clone())));
    world.add(XzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, Material::Lambertian(white.clone())));
    world.add(XzRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, Material::Lambertian(white.clone())));
    world.add(XyRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, Material::Lambertian(white.clone())));

    world.add(Cube::new(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        Material::Lambertian(white.clone())
    ));

    world.add(Cube::new(
        Point3::new(265.0, 0.0, 295.0),
        Point3::new(430.0, 330.0, 460.0),
        Material::Lambertian(white.clone())
    ));

    world
}


pub enum WorldEnum {
    OneWeekendScene,
    MovableWeekendScene,
    TwoTexturedSpheresScene,
    TwoPerlinSpheresScene,
    EarthScene,
    DiffuseLightScene,
    CornellBoxScene,
}

pub fn scene_selector(world: WorldEnum) -> HittableList {
    match world {
        WorldEnum::OneWeekendScene => one_weekend_scene(),
        WorldEnum::MovableWeekendScene => movable_one_weekend(),
        WorldEnum::TwoTexturedSpheresScene => two_textured_spheres_scene(),
        WorldEnum::TwoPerlinSpheresScene => two_perlin_spheres(),
        WorldEnum::EarthScene => earth(),
        WorldEnum::DiffuseLightScene => diffuse_light(),
        WorldEnum::CornellBoxScene => cornell_box(),
    }
}