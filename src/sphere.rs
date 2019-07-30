use super::vec::Vector3;
use super::ray::Ray;

pub struct Sphere {
    position: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64) -> Sphere {
        Sphere {
            position,
            radius,
        }
    }

    pub fn intersection(&self, mut r: Ray) -> f64 {
        let mut oc = r.origin() - self.position;
        let mut a_dot = r.direction();
        let a = a_dot.dot(a_dot);
        let b = oc.dot(a_dot);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = (b * b) - (a * c);
        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / a
        }
    }
}