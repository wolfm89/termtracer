mod vec3;

use vec3::Vec3;

fn main() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    println!("{:?}", v);

    let len = v.length();
    println!("length = {}", len);

    let norm_v = v.normalize();
    println!("normalized = {:?}", norm_v);

    let w = Vec3::new(3.0, 1.5, 1.0);
    println!("v dot w = {:?}", v.dot(&w))
}
