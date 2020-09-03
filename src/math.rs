use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

// Vector3
#[derive(Debug, Copy, Clone)]
pub struct Vec3(f64, f64, f64);
pub use Vec3 as Color;
pub use Vec3 as Point3;

impl Vec3 {
    pub fn new() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3(e0, e1, e2)
    }
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}
pub fn normalize(v: &Vec3) -> Vec3 {
    v.clone() / v.length()
}
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3(u.1 * v.2 - u.2 * v.1, u.2 * v.0 - u.0 * v.2, u.0 * v.1 - u.1 * v.0)
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl Index<i32> for Vec3 {
    type Output = f64;
    fn index(&self, index: i32) -> &f64 {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bound!"),
        }
    }
}
impl IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut f64 {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bound!"),
        }
    }
}