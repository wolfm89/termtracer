use crate::{ray::Ray, vec3::Vec3};

const SHADES: &str = " .:-=+*#%@";

pub fn draw_camera_gradient(width: usize, height: usize) {
    let mut output = String::new();
    for y in 0..height {
        for x in 0..width {
            // map to range [-1, 1]
            let u = (x as f64 / width as f64) * 2.0 - 1.0;
            let v = (y as f64 / height as f64) * 2.0 - 1.0;
            let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(u, -v, -1.0));
            let brightness = (ray.direction.y + 1.0) / 2.0;
            let idx = (brightness * (SHADES.len() - 1) as f64) as usize;
            let symbol = SHADES.chars().nth(idx).unwrap();
            output.push(symbol);
        }
        output.push('\n');
    }
    println!("{}", output)
}
