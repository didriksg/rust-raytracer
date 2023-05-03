use std::sync::Arc;
use crate::objects::aabb::AABB;
use crate::objects::hittables::{HitRecord, Hittable};
use std::f64::consts::PI;
use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct RotateY {
    hittable: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    has_box: bool,
    bbox: AABB
}

impl RotateY {
    pub fn new<T: Hittable + Send + Sync + 'static>(hittable: T, angle: f64) -> Self {
        let radians = angle * PI / 180.0;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = AABB::default();
        let has_box = hittable.bounding_box(0.0, 1.0, &mut bbox);

        let mut minimum_point_vector = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY).as_vector();
        let mut maximum_point_vector = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY).as_vector();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let if64 = i as f64;
                    let jf64 = j as f64;
                    let kf64 = k as f64;

                    let x = if64 * bbox.maximum.x + (1.0 - if64) * bbox.minimum.x;
                    let y = jf64 * bbox.maximum.x + (1.0 - jf64) * bbox.minimum.y;
                    let z = kf64 * bbox.maximum.x + (1.0 - kf64) * bbox.minimum.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z).as_vector();

                    for c in 0..3 {
                        minimum_point_vector[c] = minimum_point_vector[c].min(tester[c]);
                        maximum_point_vector[c] = maximum_point_vector[c].max(tester[c]);
                    }
                }
            }
        }

        let minimum_point = Point3::from_vector(minimum_point_vector);
        let maximum_point = Point3::from_vector(maximum_point_vector);

        Self {
            hittable: Arc::new(hittable),
            sin_theta,
            cos_theta,
            has_box,
            bbox: AABB::new(minimum_point, maximum_point),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut origin = ray.origin.as_vector();
        let mut direction = ray.direction.as_vector();

        origin[0] = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin[2] = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;

        direction[0] = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction[2] = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

        let rotated_ray = Ray::new(
            Point3::from_vector(origin),
            Point3::from_vector(direction),
            ray.time);

        if !self.hittable.hit(&rotated_ray, t_min, t_max, hit_record) {
            return false;
        }

        let mut point = hit_record.point.as_vector();
        let mut normal = hit_record.normal.as_vector();

        point[0] = self.cos_theta * hit_record.point.x + self.sin_theta * hit_record.point.z;
        point[2] = -self.sin_theta * hit_record.point.x + self.cos_theta * hit_record.point.z;

        normal[0] = self.cos_theta * hit_record.normal.x + self.sin_theta * hit_record.normal.z;
        normal[2] = -self.sin_theta * hit_record.normal.x + self.cos_theta * hit_record.normal.z;

        hit_record.point = Point3::from_vector(point);
        hit_record.set_face_normal(&rotated_ray, Vec3::from_vector(normal));

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;

        self.has_box
    }
}