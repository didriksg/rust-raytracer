use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Point3, Vec3};
use crate::materials::Material;
use crate::objects::aabb::AABB;
use crate::objects::hittables::{HitRecord, Hittable};

#[derive(Clone, Default)]
pub struct XzRectangle {
    material: Material,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRectangle {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self { material, x0, x1, z0, z1, k }
    }
}

impl Hittable for XzRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return false;
        }

        let x = ray.origin.x + (t * ray.direction.x);
        let z = ray.origin.z + (t * ray.direction.z);
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;
        hit_record.set_face_normal(ray, Vec3::new(0.0, 1.0, 0.0));
        hit_record.material = self.material.clone();
        hit_record.point = ray.at(t);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z0),
        );

        true
    }
}