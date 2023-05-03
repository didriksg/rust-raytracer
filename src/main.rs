use std::time::Instant;

use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use rust_raytracer::data_structs::ray::ray_color;
use rust_raytracer::data_structs::vec3::{Color, Point3, Vec3};
use rust_raytracer::objects::camera::Camera;
use rust_raytracer::objects::hittables::Hittable;
use rust_raytracer::scenes::{scene_selector, WorldEnum};

// Image. Change these params to get faster, but lower quality renders. const
const ASPECT_RATIO: f64 = 1.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 200;
const MAX_DEPTH: usize = 50;
const OUTPUT_PATH: &str = "output.png";


fn ray_trace_pixel(camera: &Camera, world: &dyn Hittable, background: &Color, x: u32, y: u32) -> Color {
    let u = (rand::random::<f64>() + x as f64) / (IMAGE_WIDTH - 1) as f64;
    let v = (rand::random::<f64>() + y as f64) / (IMAGE_HEIGHT - 1) as f64;
    let ray = &camera.get_ray(u, v);

    ray_color(ray, background, world, MAX_DEPTH)
}

fn main() {
    // World.
    let world = scene_selector(WorldEnum::FinalScene);
    let light = 0.0;
    let background = Color::new(light, light, light);

    // Camera.
    let look_from = Point3::new(478.0, 278.0, -600.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let up_vector = Vec3::new(0.0, 1.0, 0.0);
    let field_of_view: f64 = 40.0;
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let start_time = 0.0;
    let end_time = 1.0;

    let camera = Camera::new(
        look_from,
        look_at,
        up_vector,
        field_of_view,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        start_time,
        end_time,
    );

    // Progress bar.
    let progress_bar = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64);
    let progress_style = ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {percent}% [Rendering]");
    progress_bar.set_style(progress_style.unwrap());

    // Render loop.
    let render_time = Instant::now();

    let pixels = (0..IMAGE_HEIGHT).into_par_iter().rev().map(|y| {
        (0..IMAGE_WIDTH).into_par_iter().map(|x| {
            progress_bar.inc(1);
            (0..SAMPLES_PER_PIXEL).into_par_iter().map(|_| {
                ray_trace_pixel(&camera, &world, &background, x, y)
            }).sum::<Color>()
        }).collect::<Vec<Color>>()
    }).collect::<Vec<Vec<Color>>>();

    // Generate image from vector of pixels.
    let mut buffer = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (y, row) in pixels.iter().enumerate() {
        for (x, color) in row.iter().enumerate() {
            buffer.put_pixel(x as u32, y as u32, color.write_color(SAMPLES_PER_PIXEL));
        }
    }

    let render_time = render_time.elapsed();
    println!("Done.");
    println!("Render time: {:?}", render_time);

    match buffer.save(OUTPUT_PATH) {
        Err(e) => eprintln!("Error writing file: {e}"),
        Ok(()) => println!("Render saved to: {OUTPUT_PATH}"),
    };
}
