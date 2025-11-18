use crate::{hit::HitRecord, ray::Ray, vec3::Vec3};

pub type Color = Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color,
}

impl ScatterResult {
    pub fn new(scattered: Ray, attenuation: Color) -> Self {
        Self {
            scattered,
            attenuation,
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterResult>;

    // Add this new method
    fn emitted(&self) -> Color {
        Vec3::zero() // Default: no emission
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_dir = hit.normal + Vec3::random_unit_vector();
        if scatter_dir.length() < 1e-5 {
            scatter_dir = hit.normal;
        }
        let origin = hit.point + hit.normal * 1e-4;
        let scattered = Ray::new(origin, scatter_dir);
        Some(ScatterResult::new(scattered, self.albedo))
    }
}

pub struct DiffuseLight {
    pub emit: Color,
}

impl DiffuseLight {
    pub fn new(emit: Color) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit: &HitRecord) -> Option<ScatterResult> {
        None // Lights don't scatter rays
    }

    fn emitted(&self) -> Color {
        self.emit
    }
}
