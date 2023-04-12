use crate::data_structs::vec3::{Color, Point3, Vec3};
use crate::materials::Scatterable;
use crate::objects::{HitRecord, Hittable};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, direction: dir }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn default() -> Ray {
        Ray::new(Vec3::ZERO, Vec3::ZERO)
    }
}


pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    let mut record = HitRecord::default();

    if depth <= 0 {
        return Color::ZERO;
    }

    if world.hit(ray, 0.0001, f64::INFINITY, &mut record) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::ZERO;

        return if record.material.scatter(ray, &record, &mut attenuation, &mut scattered) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::ZERO
        };
    }

    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
}