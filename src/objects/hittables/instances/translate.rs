use std::sync::Arc;
use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::Vec3;
use crate::objects::aabb::AABB;
use crate::objects::hittables::{HitRecord, Hittable};

#[derive(Clone, Default)]
struct Translate {
    hittable: Arc<dyn Hittable>,
    offset: Vec3
}

impl Translate {
    pub fn new<T: Hittable + 'static>(hittable: T, offset: Vec3) -> Self {
        Self { hittable: Arc::new(hittable), offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        todo!()
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        todo!()
    }
}