use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Color, Point3, Vec3};
use crate::materials::Material;
use crate::objects::aabb::AABB;
use std::sync::Arc;
use dyn_clonable::dyn_clone::DynClone;

pub mod bvh;
pub mod cube;
pub mod moving_sphere;
pub mod sphere;
pub mod rectangles;
pub mod instances;

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Material,
    pub background: Color
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        self.normal = normal;
        self.front_face = front_face;
    }

    pub fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.background
    }
}

pub trait Hittable: DynClone {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}


/// Holds hittable objects
#[derive(Default, Clone)]
pub struct HittableList {
    pub hittable_list: Vec<Arc<dyn Hittable + Sync + Send>>,
}


impl HittableList {
    pub fn new() -> Self {
        HittableList {
            hittable_list: vec![],
        }
    }

    pub fn add<T: Hittable + Send + Sync + 'static>(&mut self, hittable: T) {
        self.hittable_list.push(Arc::new(hittable));
    }
}


impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.hittable_list.iter() {
            if object.hit(&ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
            }
        }

        if hit_anything {
            *hit_record = temp_record;
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.hittable_list.is_empty() {
            return false;
        }

        let mut temp_box = AABB::default();
        let mut first_box = true;

        for hittable in self.hittable_list.iter() {
            if !hittable.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }

            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box(*output_box, temp_box)
            };

            first_box = false;
        }

        true
    }
}
