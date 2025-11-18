use std::sync::Arc;

use crate::{
    hit::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

const EPSILON: f64 = 1e-8; // A small value for float comparisons

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn point_on_surface(&self) -> Vec3 {
        self.center + Vec3::random_unit_vector() * self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let a = ray.direction.dot(&ray.direction);
        let co = ray.origin - self.center;
        let b = 2.0 * co.dot(&ray.direction);
        let c = co.dot(&co) - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * a * c;

        // 1. Check if the discriminant is negative (missed the sphere).
        // We allow for a tiny negative value due to precision error.
        if discriminant < -EPSILON {
            return None;
        }

        // 2. Calculate the two potential roots (t values)
        let disc_sqrt = discriminant.sqrt();
        let inv_2a = 1.0 / (2.0 * a); // Calculate 1/(2a) once

        let mut t_near = (-b - disc_sqrt) * inv_2a; // Root 1 (generally nearer)
        let mut t_far = (-b + disc_sqrt) * inv_2a; // Root 2 (generally farther)

        // Ensure t_near is actually the smaller one, important if 'a' is negative (rare in ray tracing).
        if t_near > t_far {
            std::mem::swap(&mut t_near, &mut t_far);
        }

        let mut t = t_near;

        // 3. Find the valid root within the range [t_min, t_max]

        // Check the nearest root (t_near) first
        if t < t_min || t > t_max {
            // If t_near is out of range, check the farther root (t_far)
            t = t_far;
            if t < t_min || t > t_max {
                // If t_far is also out of range, the ray misses within the criteria
                return None;
            }
        }

        // 4. Record the hit
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;

        let normal;
        if ray.direction.dot(&outward_normal) > 0.0 {
            normal = -outward_normal;
        } else {
            normal = outward_normal;
        }

        Some(HitRecord::new(t, p, normal, self.material.clone()))
    }
}
