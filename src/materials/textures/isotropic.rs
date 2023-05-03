use std::sync::Arc;
use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Color, random_in_unit_sphere};
use crate::materials::Scatterable;
use crate::materials::textures::solid_color::SolidColor;
use crate::materials::textures::Texture;
use crate::objects::hittables::{HitRecord};

#[derive(Clone)]
pub struct Isotropic {
    albedo: Arc<dyn Texture>
}

impl Isotropic {
    pub fn from_texture<T: Texture + Send + Sync + 'static>(texture: T) -> Self {
        Self { albedo: Arc::new(texture) }
    }

    pub fn from_color(color: Color) -> Self {
        Self::from_texture(SolidColor::new(color.x, color.y, color.z))
    }
}

impl Scatterable for Isotropic {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray::new(record.point, random_in_unit_sphere(), ray.time);
        *attenuation = self.albedo.value(record.u, record.v, &record.point);

        true
    }
}