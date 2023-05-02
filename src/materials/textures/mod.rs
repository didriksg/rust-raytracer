pub mod solid_color;
pub mod checker_texture;
pub mod perlin;
pub mod image_texture;

use dyn_clonable::dyn_clone::DynClone;
use crate::data_structs::vec3::{Color, Point3};

pub trait Texture: DynClone + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}