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
    pub fn hit_orig(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let ray_origin_as_vector = ray.origin.as_vector();
        let ray_direction_as_vector = ray.direction.as_vector();
        let minimum_as_vector = self.minimum.as_vector();
        let maximum_as_vector = self.maximum.as_vector();

        for i in 0..3 {

            let t0 = f64::min((minimum_as_vector[i] - ray_origin_as_vector[i]) / ray_direction_as_vector[i], (maximum_as_vector[i] - ray_origin_as_vector[i]) / ray_direction_as_vector[i]);
            let t1 = f64::max((minimum_as_vector[i] - ray_origin_as_vector[i]) / ray_direction_as_vector[i], (maximum_as_vector[i] - ray_origin_as_vector[i]) / ray_direction_as_vector[i]);

            t_min = f64::max(t0, t_min);
            t_max = f64::max(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let inverse_dir_x = 1.0 / ray.direction.x;
        let inverse_dir_y = 1.0 / ray.direction.y;
        let inverse_dir_z = 1.0 / ray.direction.z;

        let t1 = (self.minimum.x - ray.origin.x) * inverse_dir_x;
        let t2 = (self.maximum.x - ray.origin.x) * inverse_dir_x;
        let t3 = (self.minimum.y - ray.origin.y) * inverse_dir_y;
        let t4 = (self.maximum.y - ray.origin.y) * inverse_dir_y;
        let t5 = (self.minimum.z - ray.origin.z) * inverse_dir_z;
        let t6 = (self.maximum.z - ray.origin.z) * inverse_dir_z;

        t_min = f64::max(f64::max(f64::min(t1, t2), f64::min(t3, t4)), f64::min(t5, t6));
        t_max = f64::min(f64::min(f64::max(t1, t2), f64::max(t3, t4)), f64::max(t5, t6));

        if t_max < 0.0 {
            return false;
        }

        if t_min > t_max {
            return false;
        }

        true

        // let ray_origin_as_vector = ray.origin.as_vector();
        // let ray_direction_as_vector = ray.direction.as_vector();
        // let minimum_as_vector = self.minimum.as_vector();
        // let maximum_as_vector = self.maximum.as_vector();
        //
        // for i in 0..3 {
        //     let inverse_direction = 1.0 / ray_direction_as_vector[i];
        //     let mut t0 = (minimum_as_vector[i] - ray_origin_as_vector[i]) * inverse_direction;
        //     let mut t1 = (maximum_as_vector[i] - ray_origin_as_vector[i]) * inverse_direction;
        //
        //     if inverse_direction < 0.0 {
        //         swap(&mut t0, &mut t1);
        //     }
        //
        //     t_min = if t0 > t_min { t0 } else { t_min };
        //     t_max = if t1 < t_max { t1 } else { t_max };
        //
        //     if t_max <= t_min {
        //         return false;
        //     }
        // }
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let smallest_point = Point3::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z));

        let biggest_point = Point3::new(
            f64::max(box0.maximum.x,box1.maximum.x),
            f64::max(box0.maximum.y,box1.maximum.y),
            f64::max(box0.maximum.z,box1.maximum.z));

        AABB::new(smallest_point, biggest_point)
    }
}