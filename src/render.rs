use crate::{
    hit::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

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
const VERTICAL_FOV_DEGREES: f64 = 60.0;
const IMAGE_PLANE_Z: f64 = -1.0;
const T_MIN: f64 = 1e-6;
const T_MAX: f64 = f64::INFINITY;

const AMBIENT: f64 = 0.1;

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

pub fn draw(width: usize, height: usize, scene: Vec<Box<dyn Hittable>>) {
    let light_dir = LIGHT_DIR.normalize();

    let mut output = String::new();

    for y in (0..height).step_by(2) {
        for x in 0..width {
            let fg = get_pixel_color(x, y, width, height, &scene, &light_dir);
            let bg = get_pixel_color(x, y + 1, width, height, &scene, &light_dir);

            let prefix = ansi_fg_bg(fg, bg);

            output.push_str(&prefix);
            output.push('▀');
        }
        output.push_str("\x1b[0m\n");
    }
    print!("{}", output)
}

fn calculate_viewport_params(aspect: f64) -> (f64, f64) {
    // Fixed vertical FOV approach
    let vfov_rad = VERTICAL_FOV_DEGREES.to_radians();
    let viewport_height = 2.0 * (vfov_rad / 2.0).tan();
    let viewport_width = viewport_height * aspect;

    // Image plane is always at z = -1 with this approach
    (viewport_width, viewport_height)
}

fn image_to_world(x: usize, y: usize, z: f64, width: usize, height: usize) -> Vec3 {
    let aspect = width as f64 / height as f64;
    let (viewport_width, viewport_height) = calculate_viewport_params(aspect);

    let half_width = viewport_width / 2.0;
    let half_height = viewport_height / 2.0;

    // Map pixel coordinates to viewport coordinates
    let u = (x as f64 / width as f64) * viewport_width - half_width;
    let v = (y as f64 / height as f64) * viewport_height - half_height;

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

fn intensity_to_rgb(intensity: f64) -> Rgb {
    let v = (intensity * 255.).round().clamp(0., 255.) as u8;
    Rgb::new(v, v, v)
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
    light_dir: &Vec3,
) -> Rgb {
    let dir = image_to_world(x, y, IMAGE_PLANE_Z, width, height);
    let ray = Ray::new(CAMERA_ORIGIN, dir);
    let hit: Option<HitRecord> = trace_ray(&ray, scene);
    let intensity: f64 = calc_intensity(light_dir, hit);
    intensity_to_rgb(intensity)
}
