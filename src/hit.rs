use std::sync::Arc;

use crate::{material::Material, ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        Self {
            t,
            point,
            normal,
            material,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
