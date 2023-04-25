use rand::distributions::uniform::SampleRange;
use rand::thread_rng;
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

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> bool {
        let mut box_a = AABB::default();
        let mut box_b = AABB::default();

        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BVHNode.");
        }

        box_a.minimum.as_vector()[axis] < box_a.minimum.as_vector()[axis]
    }

    fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {true}
    fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {true}
    fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {true}

    pub fn new(self, hittables: Vec<Box<dyn Hittable + Send + Sync>>, start: usize, end: usize, time0: f64, time1: f64) -> Self {
        let mut rng = thread_rng();

        let mut objects = hittables;
        let axis = (0..2).sample_single(&mut rng);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => Self::box_x_compare,
        };


        BVHNode {
            objects: Default::default(),
            start: 0,
            end: 0,
            left: None,
            right: None,
            bbox: Default::default(),
        }
    }

    pub fn from_list_hittable_list(hittable_list: HittableList, start: usize, end: usize, time0: f64, time1: f64) -> Self {
        BVHNode {
            objects: hittable_list.clone(),
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
        if !self.bbox.hit(ray, t_min, t_max) {
            return false
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