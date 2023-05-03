use std::sync::Arc;
use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::Vec3;
use crate::objects::aabb::AABB;
use crate::objects::hittables::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Translate {
    hittable: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3
}

impl Translate {
    pub fn new<T: Hittable + Send + Sync + 'static>(hittable: T, offset: Vec3) -> Self {
        Self { hittable: Arc::new(hittable), offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if !self.hittable.hit(&moved_ray, t_min, t_max, hit_record) {
            return false;
        }

        hit_record.point += self.offset;
        hit_record.set_face_normal(&moved_ray, hit_record.normal);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.hittable.bounding_box(time0, time1, output_box) {
           return false;
        }

        *output_box = AABB::new(
            output_box.minimum + self.offset,
            output_box.maximum + self.offset,
        );

        true
    }
}