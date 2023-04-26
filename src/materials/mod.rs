use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::Color;
use crate::objects::HitRecord;

use self::dielectric::Dielectric;
use self::lambertian::Lambertian;
use self::metal::Metal;

pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod texture;

#[derive(Clone)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian(Lambertian::new_color(Color::new(0.5, 0.5, 0.5)))
    }
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray, record, attenuation, scattered),
            Material::Metal(ref inner) => inner.scatter(ray, record, attenuation, scattered),
            Material::Dielectric(ref inner) => inner.scatter(ray, record, attenuation, scattered),
        }
    }
}