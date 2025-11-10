use crate::{ray::Ray, vec3::Vec3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, normal: Vec3) -> Self {
        Self { t, point, normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
