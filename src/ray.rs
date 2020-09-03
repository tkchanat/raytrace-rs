use crate::math::*;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}
impl Ray {
    pub fn new() -> Self {
        Ray {
            orig: Point3::new(),
            dir: Vec3::new(),
        }
    }
    pub fn from(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Point3 {
        &self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
