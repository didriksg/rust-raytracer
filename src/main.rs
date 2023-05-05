use std::time::Instant;

use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use rust_raytracer::data_structs::ray::ray_color;
use rust_raytracer::data_structs::vec3::{Color, Vec3};
use rust_raytracer::objects::camera::Camera;
use rust_raytracer::objects::hittables::{Hittable, HittableList};
use rust_raytracer::scenes::{scene_selector, WorldEnum};

// Image. Change these params to get faster, but lower quality renders. const
const ASPECT_RATIO: f64 = 1.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: usize = 10_000;
const MAX_DEPTH: usize = 50;
const OUTPUT_PATH: &str = "output.png";


fn ray_trace_pixel(camera: &Camera, world: &dyn Hittable, background: &Color, x: u32, y: u32) -> Color {
    let u = (rand::random::<f64>() + x as f64) / (IMAGE_WIDTH - 1) as f64;
    let v = (rand::random::<f64>() + y as f64) / (IMAGE_HEIGHT - 1) as f64;
    let ray = &camera.get_ray(u, v);

    ray_color(ray, background, world, MAX_DEPTH)
}

fn render_loop(image_width: u32, image_height: u32, camera: &Camera, world: &HittableList, background: &Color) -> Vec<Vec<Color>> {
    (0..image_width)
        .into_par_iter()
        .rev()
        .map(|y| (0..image_height)
            .into_par_iter()
            .map(|x| ray_trace_pixel(camera, world, background, x, y))
            .collect::<Vec<Color>>())
        .collect::<Vec<Vec<Color>>>()
}

fn add_images(current_image: &Vec<Vec<Color>>, new_image: &Vec<Vec<Color>>) -> Vec<Vec<Vec3>> {
    current_image
        .into_par_iter()
        .zip(new_image)
        .map(|(current_row, new_row)| {
            current_row
                .into_par_iter()
                .zip(new_row)
                .map(|(orig_pixel, new_pixel)| (orig_pixel + new_pixel))
                .collect::<Vec<Color>>()
        }).collect::<Vec<Vec<Color>>>()
}

fn main() {
    // World.
    let (background, camera, world) = scene_selector(
        WorldEnum::FinalScene,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
    );

    // Progress bar.
    let progress_bar = ProgressBar::new(SAMPLES_PER_PIXEL as u64);
    let progress_style = ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {percent}% [Rendering frame {pos}/{len}]");
    progress_bar.set_style(progress_style.unwrap());

    // Render loop.
    let render_time = Instant::now();

    let mut pixels = vec![vec![Color::ZERO; IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];
    for i in 0..SAMPLES_PER_PIXEL {
        let new_pixels = render_loop(IMAGE_HEIGHT, IMAGE_WIDTH, &camera, &world, &background);
        pixels = add_images(&pixels, &new_pixels);

        // Generate image from vector of pixels.
        let mut buffer = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
        for (y, row) in pixels.iter().enumerate() {
            for (x, color) in row.iter().enumerate() {
                buffer.put_pixel(x as u32, y as u32, color.write_color(i + 1));
            }
        }
        buffer.save(OUTPUT_PATH).expect("TODO: panic message");
        // {
        //     Err(e) => eprintln!("Error writing file: {e}"),
        //     Ok(()) => println!("Render saved to: {OUTPUT_PATH}"),
        // };  
        progress_bar.inc(1);
    }



    let render_time = render_time.elapsed();
    println!("Done.");
    println!("Render time: {:?}", render_time);

}
