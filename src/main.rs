mod hit;
mod ray;
mod render;
mod sphere;
mod vec3;

use crate::{hit::Hittable, sphere::Sphere, vec3::Vec3};

fn main() {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let sphere1 = Sphere::new(Vec3::new(2.0, 0.0, -6.0), 3.0);
    let sphere2 = Sphere::new(Vec3::new(-3.0, 0.0, -7.0), 3.0);
    let sphere3 = Sphere::new(Vec3::new(0.0, 2.0, -4.0), 1.0);
    scene.push(Box::new(sphere1));
    scene.push(Box::new(sphere2));
    scene.push(Box::new(sphere3));

    render::draw(100, 50, scene);
}
