use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::Point3;
use crate::materials::Material;
use crate::objects::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

fn root_check(root: f64, t_min: f64, t_max: f64) -> bool {
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
        hit_record.material = self.material;

        true
    }
}
