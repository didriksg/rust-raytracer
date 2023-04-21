use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Point3, Vec3};
use crate::materials::Material;
use crate::objects::aabb::AABB;
use crate::objects::bvh::BVHNode;
use crate::objects::moving_sphere::MovingSphere;
use crate::objects::sphere::Sphere;

pub mod sphere;
pub mod camera;
pub mod moving_sphere;
pub mod aabb;
pub mod bvh;


#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
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
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}

pub enum HittableObjet {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    HittableList(HittableList),
    BVHNode(BVHNode),
}

impl Hittable for HittableObjet {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        match *self {
            HittableObjet::Sphere(ref inner) => inner.hit(ray, t_min, t_max, hit_record),
            HittableObjet::MovingSphere(ref inner) => inner.hit(ray, t_min, t_max, hit_record),
            HittableObjet::HittableList(ref inner) => inner.hit(ray, t_min, t_max, hit_record),
            HittableObjet::BVHNode(ref inner) => inner.hit(ray, t_min, t_max, hit_record),
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        match *self {
            HittableObjet::Sphere(ref inner) => inner.bounding_box(time0, time1, output_box),
            HittableObjet::MovingSphere(ref inner) => inner.bounding_box(time0, time1, output_box),
            HittableObjet::HittableList(ref inner) => inner.bounding_box(time0, time1, output_box),
            HittableObjet::BVHNode(ref inner) => inner.bounding_box(time0, time1, output_box),
        }
    }
}



/// Holds hittable objects.
#[derive(Default)]
pub struct HittableList {
    hittable_list: Vec<Box<dyn Hittable + Sync + Send>>,
}


impl HittableList {
    pub fn new() -> Self {
        HittableList {
            hittable_list: vec![],
        }
    }

    pub fn add<T: Hittable + Send + Sync + 'static>(&mut self, hittable: T) {
        self.hittable_list.push(Box::new(hittable));
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

        return hit_anything;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.hittable_list.is_empty() {
            return false;
        }

        let mut temp_box = AABB::default();
        let mut first_box = true;

        for hittable in self.hittable_list.iter() {
            if !hittable.bounding_box(time0, time1, &mut temp_box) {
                return false
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
