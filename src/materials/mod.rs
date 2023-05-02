use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Color, Point3};
use crate::materials::diffuse_light::DiffuseLight;
use crate::objects::hittables::HitRecord;

use self::dielectric::Dielectric;
use self::lambertian::Lambertian;
use self::metal::Metal;

pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod textures;
pub mod diffuse_light;

#[derive(Clone)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian(Lambertian::from_color(Color::new(0.5, 0.5, 0.5)))
    }
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub trait Emmitable {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray, record, attenuation, scattered),
            Material::Metal(ref inner) => inner.scatter(ray, record, attenuation, scattered),
            Material::Dielectric(ref inner) => inner.scatter(ray, record, attenuation, scattered),

            _ => false,
        }
    }
}

impl Emmitable for Material {
    fn emitted(&self, u: f64, v: f64, p: &Color) -> Color {
        match *self {
            Material::DiffuseLight(ref inner) => inner.emitted(u, v, p),

            _ => Color::new(0.0, 0.0, 0.0)
        }
    }
}