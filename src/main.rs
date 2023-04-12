use std::time::Instant;

use image::{RgbImage};
use rayon::prelude::*;

use objects::camera::Camera;

use crate::data_structs::ray::ray_color;
use crate::data_structs::vec3::{Color, Point3, Vec3};
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::Material;
use crate::materials::metal::Metal;
use crate::objects::HittableList;
use crate::objects::sphere::Sphere;

mod materials;
mod objects;
mod data_structs;

// Image. Change these params to get faster, but lower quality renders.
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 500;
const MAX_DEPTH: usize = 50;

fn random_scene() -> HittableList {
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
    world.add(Sphere::new(Point3::new(4.0, 0.8, 2.5), 0.8, material_dielectric));
    world.add(Sphere::new(Point3::new(4.0, 0.8, 2.5), -0.75, material_dielectric));

    world
}


fn main() {
    // World.
    let world = random_scene();
    // Camera.
    let look_from = Point3::new(13.0, 2.0, 5.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let up_vector = Vec3::new(0.0, 1.0, 0.0);
    let field_of_view: f64 = 30.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(look_from, look_at, up_vector, field_of_view, ASPECT_RATIO, aperture, dist_to_focus);

    // Render
    let render_time = Instant::now();

    let pixels = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(|y| {
            (0..IMAGE_WIDTH)
                .into_par_iter()
                .map(|x| {
                    (0..SAMPLES_PER_PIXEL)
                        .into_par_iter()
                        .map(|_| {
                            let u = (rand::random::<f64>() + x as f64) / (IMAGE_WIDTH - 1) as f64;
                            let v = (rand::random::<f64>() + y as f64) / (IMAGE_HEIGHT - 1) as f64;
                            let ray = &camera.get_ray(u, v);

                            ray_color(&ray, &world, MAX_DEPTH)
                        })
                        .sum::<Color>()
                })
                .collect::<Vec<Color>>()
        })
        .collect::<Vec<Vec<Color>>>();

    let mut buffer = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (y, row) in pixels.iter().enumerate() {
        for (x, color) in row.iter().enumerate() {
            buffer.put_pixel(x as u32, y as u32, color.write_color(SAMPLES_PER_PIXEL));
        }
    }

    let render_time = render_time.elapsed();
    println!("{:?}", render_time);

    match buffer.save("renders/output.png") {
        Err(e) => eprintln!("Error writing file: {e}"),
        Ok(()) => println!("Done."),
    };
}
