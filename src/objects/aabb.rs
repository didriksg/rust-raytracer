use std::mem::swap;
use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::Point3;


/// Axis-aligned bounding box
#[derive(Default, Copy, Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        AABB { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let ray_origin_as_vector = ray.origin.as_vector();
        let minimum_as_vector = self.minimum.as_vector();
        let maximum_as_vector = self.maximum.as_vector();

        for i in 0..3 {
            let inverse_direction = 1.0 / ray_origin_as_vector[i];
            let mut t0 = minimum_as_vector[i] - ray_origin_as_vector[i] * inverse_direction;
            let mut t1 = maximum_as_vector[i] - ray_origin_as_vector[i] * inverse_direction;

            if inverse_direction < 0.0 {
                swap(&mut t0, &mut t1);
            }

            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let smallest_point = Point3::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z));

        let biggest_point = Point3::new(
            f64::min(box0.maximum.x,box1.maximum.x),
            f64::min(box0.maximum.y,box1.maximum.y),
            f64::min(box0.maximum.z,box1.maximum.z));

        AABB::new(smallest_point, biggest_point)
    }
}