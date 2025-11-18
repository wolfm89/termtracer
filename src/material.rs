use crate::{hit::HitRecord, ray::Ray, vec3::Vec3};

const EPSILON: f64 = 1e-4;

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

// Lambertian
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
        let mut scatter_dir = hit.normal + hit.normal.random_on_hemisphere();
        if scatter_dir.near_zero() {
            scatter_dir = hit.normal;
        }
        let origin = hit.point + hit.normal * EPSILON;
        let scattered = Ray::new(origin, scatter_dir);
        Some(ScatterResult::new(scattered, self.albedo))
    }
}

// Diffuse Light
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

// Metal

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = _ray_in.direction.normalize().reflect(&hit.normal);
        let mut scatter_dir = reflected;

        if self.fuzz > 0.0 {
            scatter_dir = scatter_dir + Vec3::random_unit_vector() * self.fuzz;
        }

        // Absorbed if reflection goes "into" the surface
        if scatter_dir.dot(&hit.normal) <= 0.0 {
            return None;
        }

        let origin = hit.point + hit.normal * EPSILON;
        let scattered = Ray::new(origin, scatter_dir);
        Some(ScatterResult::new(scattered, self.albedo))
    }
}
