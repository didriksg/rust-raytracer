use crate::data_structs::ray::Ray;
use crate::objects::{HitRecord, Hittable, HittableList};
use crate::objects::aabb::AABB;

/// Bounding volume hierarchy
pub struct BVHNode {
    objects: HittableList,
    start: usize,
    end: usize,
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hittable_list: &HittableList, time0: f64, time1: f64) -> Self {
        BVHNode {
            objects: *hittable_list,
            start: 0,
            end: hittable_list.hittable_list.len(),
            left: None,
            right: None,
            bbox: Default::default(),
        }

    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        todo!()
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        todo!()
    }
}