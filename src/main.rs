mod hit;
mod ray;
mod render;
mod sphere;
mod vec3;

use crate::{hit::Hittable, sphere::Sphere, vec3::Vec3};

fn main() {
    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();

    let spheres = vec![
        Sphere::new(Vec3::new(3.0, 0.0, -5.0), 1.0),
        Sphere::new(Vec3::new(-3.0, -2.0, -7.0), 1.5),
        Sphere::new(Vec3::new(3.0, 3.0, -9.0), 1.5),
        Sphere::new(Vec3::new(0.0, 0.0, -10.0), 2.0),
        Sphere::new(Vec3::new(-4.0, 3.0, -10.0), 1.5),
    ];

    for sphere in spheres {
        scene.push(Box::new(sphere));
    }

    let aspect = 16. / 9.;
    let width = 180;
    let height = (width as f64 / aspect) as usize;

    render::draw(width, height, scene);
}
