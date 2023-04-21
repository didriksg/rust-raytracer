use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Point3, Vec3};
use crate::materials::Material;
use crate::objects::{HitRecord, Hittable};
use crate::objects::aabb::AABB;
use crate::objects::sphere::root_check;

#[derive(Copy, Clone)]
pub struct MovingSphere {
    start_point: Point3,
    end_point: Point3,
    start_time: f64,
    end_time: f64,
    radius: f64,
    material: Material,
}

impl MovingSphere {
    pub fn new(start_point: Point3, end_point: Point3, start_time: f64, end_time: f64, radius: f64, material: Material) -> Self {
        MovingSphere {
            start_point,
            end_point,
            start_time,
            end_time,
            radius,
            material,
        }
    }

    fn center(&self, at_time: f64) -> Point3 {
        self.start_point + ((at_time - self.start_time) / (at_time - self.end_time)) * (self.end_point - self.start_point)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let origin_center = ray.origin - self.center(ray.time);
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

        let outward_normal = (hit_record.point - self.center(ray.time)) / self.radius;
        hit_record.set_face_normal(&ray, outward_normal);
        hit_record.material = self.material;

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);
        let box_time0 = AABB::new(
            self.center(time0) - radius_vector,
            self.center(time0) + radius_vector
        );

        let box_time1 = AABB::new(
            self.center(time1) - radius_vector,
            self.center(time1) + radius_vector
        );

        *output_box = AABB::surrounding_box(box_time0, box_time1);

        true
    }
}