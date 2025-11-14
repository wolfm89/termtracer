mod hit;
mod ray;
mod render;
mod sphere;
mod vec3;

use crate::{hit::Hittable, sphere::Sphere, vec3::Vec3};

fn main() {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let spheres = vec![
        Sphere::new(Vec3::new(4.0, 0.0, -5.0), 2.0),
        Sphere::new(Vec3::new(-4.0, -2.0, -7.0), 3.0),
        Sphere::new(Vec3::new(4.0, 4.0, -9.0), 3.0),
        Sphere::new(Vec3::new(0.0, 0.0, -10.0), 4.0),
        Sphere::new(Vec3::new(-5.0, 4.0, -10.0), 3.0),
    ];

    for sphere in spheres {
        scene.push(Box::new(sphere));
    }

    render::draw(150, 90, scene);
}
