mod vec;
mod ray;
mod sphere;
mod camera;

use vec::Vector3;
use ray::Ray;
use sphere::Sphere;
use camera::Camera;
fn color(mut r: Ray) -> Vector3 {
    let origin = Vector3::new(0.0, 0.0, -1.0);
    let new_sphere = Sphere::new(origin, 0.5);
    let t = new_sphere.intersection(r);
    // println!("{}", t);
    if t > 0.0 {
        let mut point = r.point_at_parameter(t) - origin;
        let shade = point.make_unit_vector();

        (0.5 * (shade + 1.0))
    } else {
        let mut unit_direction = r.direction();
        unit_direction = unit_direction.make_unit_vector();
        let t: f64 = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);
    let c = Camera::new(origin, lower_left_corner, horizontal, vertical);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f64 = (f64::from(i))/(f64::from(nx));
            let v: f64 = (f64::from(j))/(f64::from(ny));
            let r = c.get_ray(u, v);
            let mut col = 255.0*color(r);

        println!("{} {} {}", col.r() as u8, col.g() as u8, col.b() as u8);
        }
    }
}
