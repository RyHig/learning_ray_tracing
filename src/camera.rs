use super::ray::Ray;
use super::vec::Vector3;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3
}

impl Camera {
    pub fn new(origin: Vector3, lower_left_corner: Vector3, 
               horizontal: Vector3, vertical: Vector3) -> Camera {
                   Camera {
                       origin,
                       lower_left_corner,
                       horizontal,
                       vertical,
                   }
               }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal +
                 v * self.vertical - self.origin)
    }
}