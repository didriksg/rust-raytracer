use crate::data_structs::ray::Ray;
use crate::data_structs::vec3::Point3;
use crate::materials::Material;
use crate::objects::aabb::AABB;
use crate::objects::hittables::{HitRecord, Hittable, HittableList};
use crate::objects::hittables::rectangles::xy_rectangle::XyRectangle;
use crate::objects::hittables::rectangles::xz_rectangle::XzRectangle;
use crate::objects::hittables::rectangles::yz_rectangle::YzRectangle;


#[derive(Clone, Default)]
pub struct Cube {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList
}

impl Cube {
    pub fn new(box_min: Point3, box_max: Point3, material: Material) -> Self {
        let mut sides = HittableList::new();

        sides.add(XyRectangle::new(box_min.x, box_max.x, box_min.y, box_max.y, box_max.z, material.clone()));
        sides.add(XyRectangle::new(box_min.x, box_max.x, box_min.y, box_max.y, box_min.z, material.clone()));

        sides.add(XzRectangle::new(box_min.x, box_max.x, box_min.z, box_max.z, box_max.y, material.clone()));
        sides.add(XzRectangle::new(box_min.x, box_max.x, box_min.z, box_max.z, box_min.y, material.clone()));

        sides.add(YzRectangle::new(box_min.y, box_max.y, box_min.z, box_max.z, box_max.x, material.clone()));
        sides.add(YzRectangle::new(box_min.y, box_max.y, box_min.z, box_max.z, box_min.x, material.clone()));

        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, hit_record)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);

        true
    }
}