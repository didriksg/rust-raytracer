use crate::data_structs::vec3::{Color, Point3};
use crate::materials::textures::Texture;

#[derive(Clone, Default)]
pub struct SolidColor {
    color_value: Color
}

impl SolidColor {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        SolidColor { color_value: Color::new(red, green, blue)}
    }
}


impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}