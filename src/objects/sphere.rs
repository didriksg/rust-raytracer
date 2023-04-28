use std::f64::consts::PI;
use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Point3, Vec3};
use crate::materials::Material;
use crate::objects::{HitRecord, Hittable};
use crate::objects::aabb::AABB;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn get_sphere_uv(point: &Point3, u: &mut f64, v: &mut f64) {
        let theta = f64::acos(-point.y);
        let phi = f64::atan2(-point.z, point.x) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

pub fn root_check(root: f64, t_min: f64, t_max: f64) -> bool {
    root < t_min || t_max < root
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let origin_center = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = origin_center.dot(ray.direction);
        let c = origin_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        // Negative discriminant indicates no hit.
        if discriminant < 0.0 {
            return false;
        }

        let discriminant_root = discriminant.sqrt();

        let mut root = (-half_b - discriminant_root) / a;
        if root_check(root, t_min, t_max) {
            root = (-half_b + discriminant_root) / a;
            if root_check(root, t_min, t_max) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(root);

        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(&ray, outward_normal);


        // Update to correct u and v coordinates.
        Sphere::get_sphere_uv(&outward_normal, &mut hit_record.u, &mut hit_record.v);

        hit_record.material = self.material.clone();

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);

        *output_box = AABB::new(
            self.center - radius_vector,
            self.center + radius_vector
        );

        true
    }
}
