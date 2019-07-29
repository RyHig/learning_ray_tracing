mod vec;
mod ray;

use vec::Vector3;
use ray::Ray;
use std::f64;

fn hit_sphere(center: Vector3, radius: f64, mut r: Ray) -> f64 {
    let mut oc = r.origin() - center;
    let mut a_dot = r.direction();
    let a = a_dot.dot(a_dot);
    let b = 2.0 * oc.dot(a_dot);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 *a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - f64::sqrt(discriminant) ) / (2.0*a)
    }

}

fn color(mut r: Ray) -> Vector3 {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let mut point = r.point_at_parameter(t) - Vector3::new(0.0, 0.0, -1.0);
        let shade = point.make_unit_vector();
        (0.5 * (shade + 1.0))
    } else {
        let mut unit_direction = r.direction();
        unit_direction = unit_direction.make_unit_vector();
        let t: f64 = 0.5 *(unit_direction.y() + 1.0);
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
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f64 = (f64::from(i))/(f64::from(nx));
            let v: f64 = (f64::from(j))/(f64::from(ny));
            let r = Ray::new(origin,
                lower_left_corner + u*horizontal + v*vertical);
            let mut col = 255.99*color(r);

        println!("{} {} {}", col.r() as i64, col.g() as i64, col.b() as i64);
        }
    }
}
