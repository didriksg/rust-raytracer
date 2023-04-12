use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Color, random_unit_vector};
use crate::materials::Scatterable;
use crate::objects::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = record.normal + random_unit_vector();
        scatter_direction = if scatter_direction.near_zero() {
            record.normal
        } else {
            scatter_direction
        };

        *scattered = Ray::new(record.point, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}