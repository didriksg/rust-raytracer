use crate::data_structs::vec3::{Color, Point3, Vec3};
use crate::materials::{Emmitable, Scatterable};
use crate::objects::hittables::{HitRecord, Hittable};

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Ray { origin, direction, time }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}


pub fn ray_color(ray: &Ray, background: &Color, world: &dyn Hittable, depth: usize) -> Color {
    let mut record = HitRecord::default();

    // Exceeded bounce limit. End.
    if depth == 0 {
        return Color::ZERO;
    }

    // No hit -> Background color.
    if !world.hit(ray, 0.0001, f64::INFINITY, &mut record) {
        return *background
    }

    let mut scattered = Ray::default();
    let mut attenuation = Color::ZERO;
    let emitted = record.material.emitted(record.u, record.v, &record.point);

    // If material is not scattering, return emitted color.
    if !record.material.scatter(ray, &record, &mut attenuation, &mut scattered) {
        return emitted
    }

    emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
}