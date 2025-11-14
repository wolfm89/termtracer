mod hit;
mod ray;
mod render;
mod sphere;
mod vec3;

use crate::{hit::Hittable, sphere::Sphere, vec3::Vec3};

fn main() {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let sphere1 = Sphere::new(Vec3::new(4.0, 0.0, -5.0), 2.0);
    let sphere2 = Sphere::new(Vec3::new(-4.0, -2.0, -7.0), 3.0);
    let sphere3 = Sphere::new(Vec3::new(4.0, 4.0, -9.0), 3.0);
    let sphere4 = Sphere::new(Vec3::new(0.0, 0.0, -10.0), 4.0);
    scene.push(Box::new(sphere1));
    scene.push(Box::new(sphere2));
    scene.push(Box::new(sphere3));
    scene.push(Box::new(sphere4));

    render::draw(100, 45, scene);
}
