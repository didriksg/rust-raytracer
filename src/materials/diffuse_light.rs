use std::sync::Arc;
use crate::data_structs::vec3::{Color, Point3};
use crate::materials::{Emmitable};
use crate::materials::textures::solid_color::SolidColor;

use crate::materials::textures::Texture;

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn from_texture<T: Texture + 'static>(texture: T) -> Self {
        Self {
            emit: Arc::new(texture)
        }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self::from_texture(SolidColor::new(albedo.x, albedo.y, albedo.z))
    }
}

impl Emmitable for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Point3 {
        self.emit.value(u, v, p)
    }
}
