use std::cmp::Ordering;
use std::sync::Arc;

use rand::distributions::uniform::SampleRange;
use rand::thread_rng;

use crate::data_structs::ray::Ray;
use crate::objects::{HitRecord, Hittable, HittableList};
use crate::objects::aabb::AABB;

/// Bounding volume hierarchy
#[derive(Default, Clone)]
pub struct BVHNode {
    left: Option<Arc<dyn Hittable + Sync + Send>>,
    right: Option<Arc<dyn Hittable + Sync + Send>>,
    bbox: AABB,
}


impl BVHNode {
    fn box_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>, axis: usize) -> Ordering {
        let mut box_a = AABB::default();
        let mut box_b = AABB::default();

        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BVHNode.");
        }

       if box_a.minimum.as_vector()[axis] < box_b.minimum.as_vector()[axis] {
            Ordering::Greater
        } else if box_a.minimum.as_vector()[axis] == box_a.minimum.as_vector()[axis] {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }

    fn box_x_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        Self::box_compare(a, b, 2)
    }

    pub fn new(hittables: Vec<Arc<dyn Hittable + Send + Sync>>, start: usize, end: usize, time0: f64, time1: f64) -> Self {
        let mut rng = thread_rng();

        let mut objects = hittables.clone();
        let axis = (0..3).sample_single(&mut rng);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let left;
        let right;

        let object_span = end - start;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            left = Arc::new(BVHNode::new(objects.clone(), start, mid, time0, time1));
            right = Arc::new(BVHNode::new(objects.clone(), mid, end, time0, time1));
        }

        let mut box_left = AABB::default();
        let mut box_right = AABB::default();

        if left.bounding_box(time0, time1, &mut box_left)
            || right.bounding_box(time0, time1, &mut box_right) {
            panic!("No bounding box in bvh node constructor.")
        }

        Self {
            left: Some(left),
            right: Some(right),
            bbox: AABB::surrounding_box(box_left, box_right),
        }
    }

    pub fn from_list_hittable_list(hittable_list: HittableList, time0: f64, time1: f64) -> Self {
        BVHNode::new(hittable_list.hittable_list.clone(), 0, hittable_list.hittable_list.len(), time0, time1)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left
            .as_ref()
            .unwrap()
            .hit(ray, t_min, t_max, hit_record);

        let hit_right = self.right
            .as_ref()
            .unwrap()
            .hit(ray, t_min, t_max, hit_record);

        hit_left || hit_right
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;

        true
    }
}