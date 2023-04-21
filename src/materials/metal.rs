use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Color, random_in_unit_sphere, Vec3};
use crate::materials::Scatterable;
use crate::objects::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let fuzz_limited = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };

        Metal { albedo, fuzz: fuzz_limited }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(ray.direction.normalize(), record.normal);

        *scattered = Ray::new(record.point, reflected + self.fuzz * random_in_unit_sphere(), ray.time);
        *attenuation = self.albedo;

        scattered.direction.dot(record.normal) > 0.0
    }
}