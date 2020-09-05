use std::cmp::PartialOrd;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

// Constants
pub const INIFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}
pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    return rand::random();
}
pub fn random_range_double(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    return min + (max - min) * random_double();
}
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input > max {
        return max;
    } else if input < min {
        return min;
    }
    input
}

// Vector3
#[derive(Debug, Copy, Clone)]
pub struct Vec3(f64, f64, f64);
pub use Vec3 as Color;
pub use Vec3 as Point3;

impl Vec3 {
    // Constants
    pub const BLACK: Color = Color(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color(1.0, 1.0, 1.0);

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Vec3(r, g, b)
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3(e0, e1, e2)
    }
    pub fn random() -> Self {
        Vec3(random_double(), random_double(), random_double())
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3(
            random_range_double(min, max),
            random_range_double(min, max),
            random_range_double(min, max),
        )
    }
    pub fn random_in_unit_sphere() -> Self {
        let mut p = Vec3::random_range(-1.0, 1.0);
        while p.length_squared() < 1.0 {
            p = Vec3::random_range(-1.0, 1.0);
        }
        p
    }
    pub fn random_unit_vector() -> Self {
        let a = random_range_double(0.0, 2.0 * PI);
        let z = random_range_double(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3(r * a.cos(), r * a.sin(), z)
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        // In the same hemisphere as the normal
        if dot(&in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
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
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}
pub fn normalize(v: &Vec3) -> Vec3 {
    v.clone() / v.length()
}
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3(
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * dot(v, n) * 2.0
}
pub fn refract(v: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&(-*v), n);
    let r_out_perp = (*v + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * -((1.0 - r_out_perp.length_squared()).abs()).sqrt();
    let refracted = r_out_perp + r_out_parallel;
    refracted
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
