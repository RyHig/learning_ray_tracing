use crate::vec::Vector3;
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {origin, direction}
    }
    pub fn origin(&mut self) -> Vector3 {
        self.origin
    }
    pub fn direction(&mut self) -> Vector3 {
        self.direction
    }
    pub fn point_at_parameter(&mut self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
}
