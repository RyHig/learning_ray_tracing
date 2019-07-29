use std::f64;
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vector3(f64, f64, f64);

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3(x, y, z)
    }
    pub fn dot(&mut self, other: Vector3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn cross(&mut self, other : Vector3) -> Vector3 {
        Vector3(self.1 * other.2 - self.2 * other.1,
                -(self.0 * other.2 - self.2 * other.0),
                self.0 * other.1 - self.1 * other.0)
    }
    pub fn x(&mut self) -> f64 {
        self.0
    }
    pub fn y(&mut self) -> f64 {
        self.1
    }
    pub fn z(&mut self) -> f64 {
        self.2
    }
    pub fn r(&mut self) -> f64 {
        self.0
    }
    pub fn g(&mut self) -> f64 {
        self.1
    }
    pub fn b(&mut self) -> f64 {
        self.2
    }
    pub fn length(&mut self) -> f64 {
        f64::sqrt(Vector3::squared_length(self))
    }
    pub fn squared_length(&mut self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn make_unit_vector(&mut self) -> Vector3 {
        let div = 1.0 / self.length();
        Vector3(self.0 * div,
                self.1 * div,
                self.2 * div)
    }
}
impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3(self.0 + _rhs.0,
                self.1 + _rhs.1,
                self.2 + _rhs.2)
    }
}
impl ops::Add<f64> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: f64) -> Vector3 {
        Vector3(self.0 + _rhs,
                self.1 + _rhs,
                self.2 + _rhs)
    }
}
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    
    fn sub(self, _rhs: Vector3) -> Vector3 {
        Vector3(self.0 - _rhs.0,
                self.1 - _rhs.1,
                self.2 - _rhs.2)
    }
}
impl ops::Sub<f64> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: f64) -> Vector3 {
        Vector3(self.0 - _rhs,
                self.1 - _rhs,
                self.2 - _rhs)
    }
}
impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3 {
        Vector3(self.0 * _rhs.0,
                self.1 * _rhs.1,
                self.2 * _rhs.2)
    }
}
impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f64) -> Vector3 {
        Vector3(self.0 * _rhs,
                self.1 * _rhs,
                self.2 * _rhs)
    }
}
impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3 {
        Vector3(self * _rhs.0,
                self * _rhs.1,
                self * _rhs.2)
    }
}
impl ops::Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: Vector3) -> Vector3 {
        Vector3(self.0 / _rhs.0,
                self.1 / _rhs.1,
                self.2 / _rhs.2)
    }
}
impl ops::Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, _rhs: Vector3) -> Vector3 {
        Vector3(self / _rhs.0,
                self / _rhs.1,
                self / _rhs.2)
    }
}
impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f64) -> Vector3 {
        Vector3(self.0 / _rhs,
                self.1 / _rhs,
                self.2 / _rhs)
    }
}
