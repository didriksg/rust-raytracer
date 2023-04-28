use dyn_clonable::dyn_clone::DynClone;
use crate::data_structs::vec3::{Color, Point3};

pub trait Texture: DynClone + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Clone)]
pub struct SolidColor {
    color_value: Color
}

impl SolidColor {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        SolidColor { color_value: Color::new(red, green, blue)}
    }
}


impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Point3 {
        self.color_value
    }
}