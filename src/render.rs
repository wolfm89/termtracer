use crate::{
    hit::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

const SHADES: &str = ".'`^\",:;Il!i?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

const CAMERA_ORIGIN: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const LIGHT_DIR: Vec3 = Vec3 {
    x: -10.0,
    y: 5.0,
    z: -1.0,
};
const IMAGE_PLANE_Z: f64 = -1.0;
const T_MIN: f64 = 1e-6;
const T_MAX: f64 = f64::INFINITY;

const AMBIENT: f64 = 0.1;

pub fn draw(width: usize, height: usize, scene: Vec<Box<dyn Hittable>>) {
    let shades: Vec<char> = SHADES.chars().collect();
    let max_idx = shades.len() - 1;

    let light_dir = LIGHT_DIR.normalize();

    let mut output = String::new();

    for y in 0..height {
        for x in 0..width {
            // create ray
            let direction = image_to_world(x, y, IMAGE_PLANE_Z, width, height);
            let ray = Ray::new(CAMERA_ORIGIN, direction);

            // calculate hits
            let hit: Option<HitRecord> = trace_ray(&ray, &scene);

            // calculate intensity
            let intensity: f64 = calc_intensity(&light_dir, hit);

            // get symbol for intensity
            let idx = ((intensity * (max_idx as f64)) as usize).clamp(0, max_idx);
            let symbol = shades[idx];

            // write symbol twice to adjust for characters being around 2x taller than wide
            output.push(symbol);
        }
        output.push('\n');
    }
    print!("{}", output)
}

fn image_to_world(x: usize, y: usize, z: f64, width: usize, height: usize) -> Vec3 {
    let aspect = width as f64 / height as f64;

    // map to [-aspect, aspect]
    let u = ((x as f64 / width as f64) * 2.0 - 1.0) * aspect;
    // map to range [-1, 1]
    let v = (y as f64 / height as f64) * 2.0 - 1.0;

    Vec3::new(u, -v, z)
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

fn calc_intensity(light_dir: &Vec3, hit: Option<HitRecord>) -> f64 {
    let intensity: f64;
    match hit {
        Some(hit) => {
            let diff = hit.normal.dot(&light_dir).max(0.0);
            intensity = (AMBIENT + (1.0 - AMBIENT) * diff).clamp(0.0, 1.0);
        }
        None => {
            // intensity = (direction.y + 1.0) * 0.05; // 0..1
            intensity = 0.0
        }
    }
    intensity
}
