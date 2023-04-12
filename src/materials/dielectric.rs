use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Color, Vec3};
use crate::materials::Scatterable;
use crate::objects::HitRecord;

#[derive(Copy, Clone)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric { index_of_refraction }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::ONE;
        let refraction_ratio = if record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = f64::min(-unit_direction.dot(record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let random_double = rand::random::<f64>();
        let reflectance = Dielectric::reflectance(cos_theta, refraction_ratio);
        let direction = if cannot_refract || reflectance > random_double {
            Vec3::reflect(unit_direction, record.normal)
        } else {
            Vec3::refract(unit_direction, record.normal, refraction_ratio)
        };

        *scattered = Ray::new(record.point, direction);
        true
    }
}