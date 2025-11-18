use crate::ray::Ray;
use crate::vec3::Vec3;

const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
}

impl Camera {
    pub fn new(
        vfov_deg: f64,
        aspect_ratio: f64,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        // convert degrees to radians
        let theta = vfov_deg.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Vec3::new(0.0, 0.0, 0.0);

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction =
            self.lower_left_corner + self.horizontal * u + self.vertical * (1.0 - v) - self.origin;

        Ray::new(self.origin, direction.normalize())
    }
}
