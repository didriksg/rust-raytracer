use image::io::Reader as ImageReader;
use crate::data_structs::vec3::{Color, Point3};
use crate::materials::textures::Texture;


#[derive(Clone, Default)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

const BYTES_PER_PIXEL: u32 = 3;

impl ImageTexture {
    pub fn new(filename: &str) -> Self {

        let img = ImageReader::open(filename)
            .expect("Image texture file not found.")
            .decode()
            .unwrap();

        let image_width = img.width();
        let image_height = img.height();
        let data = img.into_bytes();

        Self {
            data,
            width: image_width,
            height: image_height,
            bytes_per_scanline: BYTES_PER_PIXEL * image_width,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        let clamped_u = u.clamp(0.0, 1.0);
        let clamped_v = 1.0 - v.clamp(0.0, 1.0);

        let i = (clamped_u * self.width as f64) as u32;
        let j = (clamped_v * self.height as f64) as u32;

        let i = if i >= self.width { self.width as u32 - 1} else { i };
        let j = if j >= self.height { self.height as u32 - 1} else { j };

        let pixel_index =  (j * self.bytes_per_scanline + i * 3) as usize;

        Color::new(self.data[pixel_index] as f64 / 255.0,
                   self.data[pixel_index + 1] as f64 / 255.0,
                   self.data[pixel_index + 2] as f64 / 255.0)
    }
}