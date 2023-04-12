use std::f64::consts::PI;

use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, up_vector: Vec3, fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let origin = look_from;

        let theta = fov * PI / 180.0;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = up_vector.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    pub fn random_in_unit_disk() -> Point3 {
        loop {
            let mut point = Vec3::random_with_limits(-1.0, 1.0);
            point.z = 0.0;

            if point.length_squared() >= 1.0 {
                continue;
            }

            return point;
        }
    }


    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Camera::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(self.origin + offset,
                 self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}