use std::sync::Arc;
use crate::data_structs::vec3::Color;
use crate::materials::textures::solid_color::SolidColor;
use crate::materials::textures::Texture;

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture + Send + Sync>,
    even: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new_from_texture<T: Texture + 'static>(odd: T, even: T) -> Self {
        Self { odd: Arc::new(odd), even: Arc::new(even) }
    }

    pub fn new_from_color(odd_color: Color, even_color: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(
                odd_color.x,
                odd_color.y,
                odd_color.z
            )),
            even: Arc::new(SolidColor::new(
                even_color.x,
                even_color.y,
                even_color.z
            )),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Color) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}