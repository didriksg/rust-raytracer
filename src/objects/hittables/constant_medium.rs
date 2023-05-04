use std::sync::Arc;
use rand::random;
use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::Color;
use crate::materials::Material;
use crate::materials::textures::isotropic::Isotropic;
use crate::materials::textures::solid_color::SolidColor;
use crate::materials::textures::Texture;
use crate::objects::aabb::AABB;
use crate::objects::hittables::{HitRecord, Hittable};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Send + Sync >,
    phase_function: Isotropic,
    negative_inverse_density: f64,
}

impl ConstantMedium {
    pub fn from_texture<
        H: Hittable + Send + Sync + 'static,
        T: Texture + Send + Sync + 'static>
    (hittable: H, density: f64, texture: T) -> Self {
        Self {
            boundary: Arc::new(hittable),
            phase_function: Isotropic::from_texture(texture),
            negative_inverse_density: -1.0 / density,
        }
    }

    pub fn from_color<T: Hittable + Send + Sync + 'static>(hittable: T, density: f64, color: Color) -> Self {
        Self::from_texture(
            hittable,
            density,
            SolidColor::new(color.x, color.y, color.z)
        )
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        // let enable_debug = false;
        // let debugging = enable_debug && random::<f64>() < 0.00001;

        let mut hit_record_1 = HitRecord::default();
        let mut hit_record_2 = HitRecord::default();

        if !self.boundary.hit(
            ray,
            -f64::INFINITY,
            f64::INFINITY,
            &mut hit_record_1
        ) {
            return false
        }

        if !self.boundary.hit(
            ray,
            hit_record_1.t + 0.0001,
            f64::INFINITY,
            &mut hit_record_2
        ) {
            return false
        }

        // if debugging { eprintln!("t_min: {}, t_max: {}", hit_record_1.t, hit_record_2.t); }

        hit_record_1.t = if hit_record_1.t < t_min { t_min } else { hit_record_1.t };
        hit_record_2.t = if hit_record_2.t > t_max { t_max } else { hit_record_2.t };

        if hit_record_1.t >= hit_record_2.t {
            return false;
        }

        hit_record_1.t = hit_record_1.t.max(0.0);

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit_record_2.t - hit_record_1.t) * ray_length;
        let hit_distance = self.negative_inverse_density * random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        hit_record.t = hit_record_1.t + hit_distance / ray_length;
        hit_record.point = ray.at(hit_record.t);

        // if debugging {
        //     eprintln!("Hit distance: {:?}\nhit_record.t: {:?} \nhit_record.point: {:?}", hit_distance, hit_record.t, hit_record.point);
        // }

        hit_record.material = Material::Isotropic(self.phase_function.clone());

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}