use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Color, Point3, Vec3};
use crate::materials::lambertian::Lambertian;
use crate::materials::Material;
use crate::objects::sphere::Sphere;

pub mod sphere;
pub mod camera;


pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            point: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: false,
            material: Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
        }
    }

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
}

pub struct HittableList {
    hittable_list: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { hittable_list: Vec::new() }
    }

    pub fn add(&mut self, sphere: Sphere) {
        self.hittable_list.push(sphere);
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
}
