use rand::Rng;
use rayon::prelude::*;

use crate::{
    camera::Camera,
    hit::{HitRecord, Hittable},
    material::Color,
    ray::Ray,
    vec3::Vec3,
};

const T_MIN: f64 = 1e-3;
const T_MAX: f64 = f64::INFINITY;

const SAMPLES_PER_PIXEL: usize = 50;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub fn draw(width: usize, height: usize, scene: Vec<Box<dyn Hittable>>, cam: &Camera) {
    let rows: Vec<String> = (0..height)
        .into_par_iter()
        .step_by(2)
        .map(|y| {
            let mut row = String::new();
            for x in 0..width {
                let fg = get_pixel_color(x, y, width, height, &scene, cam);
                let bg = get_pixel_color(x, y + 1, width, height, &scene, cam);
                row.push_str(&ansi_fg_bg(fg, bg));
                row.push('▀');
            }
            format!("{}\x1b[0m", row)
        })
        .collect();

    // Print rows in correct order
    for row in rows {
        println!("{}", row);
    }
}

fn trace_ray(ray: &Ray, scene: &Vec<Box<dyn Hittable>>) -> Option<HitRecord> {
    let mut closest_t = T_MAX;
    let mut closest_hit: Option<HitRecord> = None;

    // calculate hits
    for object in scene.iter() {
        let hit = object.hit(&ray, T_MIN, closest_t);
        match hit {
            Some(h) => {
                if h.t < closest_t {
                    closest_t = h.t;
                    closest_hit = Some(h);
                }
            }
            None => (),
        }
    }

    closest_hit
}

fn ray_color(ray: &Ray, scene: &Vec<Box<dyn Hittable>>, depth: usize) -> Color {
    if depth == 0 {
        return Vec3::zero();
    }

    if let Some(hit) = trace_ray(ray, &scene) {
        let emitted = hit.material.emitted();

        if let Some(scatter) = hit.material.scatter(ray, &hit) {
            return emitted
                + ray_color(&scatter.scattered, &scene, depth - 1) * scatter.attenuation;
        }
        return emitted;
    }

    // Background sky
    let t = 0.5 * (ray.direction.y + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    white * (1.0 - t) + blue * t
}

fn color_to_rgb(color: Vec3) -> Rgb {
    // Gamma-correct
    let r = color.x.clamp(0.0, 1.0).sqrt();
    let g = color.y.clamp(0.0, 1.0).sqrt();
    let b = color.z.clamp(0.0, 1.0).sqrt();

    Rgb::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn ansi_fg_bg(fg: Rgb, bg: Rgb) -> String {
    format!(
        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m",
        fg.r, fg.g, fg.b, bg.r, bg.g, bg.b
    )
}

fn get_pixel_color(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    scene: &Vec<Box<dyn Hittable>>,
    cam: &Camera,
) -> Rgb {
    let mut color = Vec3::zero();

    let mut rng = rand::rng();
    for _ in 0..SAMPLES_PER_PIXEL {
        let dx = rng.random_range(-0.5..0.5);
        let dy = rng.random_range(-0.5..0.5);

        let u = (x as f64 + dx) / (width as f64);
        let v = (y as f64 + dy) / (height as f64);

        let ray = cam.get_ray(u, v);
        color = color + ray_color(&ray, scene, 10);
    }

    color = color / (SAMPLES_PER_PIXEL as f64);
    color_to_rgb(color)
}
