use std::sync::Arc;
use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Color, random_unit_vector};
use crate::materials::Scatterable;
use crate::materials::textures::solid_color::SolidColor;
use crate::materials::textures::Texture;
use crate::objects::hittables::HitRecord;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn from_color(albedo: Color) -> Self {
        Self::new_texture(SolidColor::new(albedo.x, albedo.y, albedo.z))
    }

    pub fn new_texture<T: Texture + 'static>(texture: T) -> Self {
        Lambertian { albedo: Arc::new(texture) }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = record.normal + random_unit_vector();
        scatter_direction = if scatter_direction.near_zero() {
            record.normal
        } else {
            scatter_direction
        };

        *scattered = Ray::new(record.point, scatter_direction, ray.time);
        *attenuation = self.albedo.value(record.u, record.v, &record.point);

        true
    }
}