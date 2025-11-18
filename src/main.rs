mod camera;
mod hit;
mod material;
mod ray;
mod render;
mod sphere;
mod vec3;

use std::sync::Arc;

use crate::{
    camera::Camera,
    hit::Hittable,
    material::{DiffuseLight, Lambertian, Metal},
    sphere::Sphere,
    vec3::Vec3,
};

fn main() {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let red_lambertian = Arc::new(Lambertian::new(Vec3::new(0.9, 0.2, 0.2)));
    let blue_lambertian = Arc::new(Lambertian::new(Vec3::new(0.2, 0.2, 0.8)));
    let blue_metal = Arc::new(Metal::new(Vec3::new(0.7, 0.7, 0.9), 0.1));
    let bright_white_light = Arc::new(DiffuseLight::new(Vec3::new(20.0, 20.0, 20.0)));

    let spheres = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.5), 0.5, blue_lambertian.clone()),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100., red_lambertian.clone()),
        Sphere::new(Vec3::new(1.5, -0.05, -3.0), 0.5, blue_metal.clone()),
    ];

    for sphere in spheres {
        scene.push(Box::new(sphere));
    }

    scene.push(Box::new(Sphere::new(
        Vec3::new(-5.0, 3.0, -7.0), // Light position
        0.5,                        // Small radius
        bright_white_light,
    )));

    let aspect = 16.0 / 9.0;
    let width = 500;
    let height = (width as f64 / aspect) as usize;

    let cam = Camera::new(60.0, aspect, 75, 10);

    render::draw(width, height, scene, &cam);
}
