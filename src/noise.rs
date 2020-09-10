use crate::math::*;

// Helper functions
fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                acc += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                    * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                    * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    acc
}
fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * dot(&c[i][j][k], &weight_v);
            }
        }
    }
    acc
}

// Perlin
#[derive(Clone)]
pub struct Perlin {
    perm_x: [i32; Perlin::POINT_COUNT],
    perm_y: [i32; Perlin::POINT_COUNT],
    perm_z: [i32; Perlin::POINT_COUNT],
    rand_vec: [Vec3; Perlin::POINT_COUNT],
}
impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rand_vec = [Vec3::default(); Perlin::POINT_COUNT];
        for e in rand_vec.iter_mut() {
            *e = Vec3::random_range(-1.0, 1.0);
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Perlin {
            perm_x,
            perm_y,
            perm_z,
            rand_vec,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[(self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255])
                        as usize]
                }
            }
        }
        perlin_interp(&c, u, v, w)
    }
    pub fn turbulence(&self, p: &Point3, depth: i32) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            acc += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        acc.abs()
    }

    fn perlin_generate_perm() -> [i32; Perlin::POINT_COUNT] {
        let mut p = [0; Perlin::POINT_COUNT];
        for i in 0..Perlin::POINT_COUNT {
            p[i] = i as i32;
        }
        Perlin::permute(p);
        p
    }
    fn permute(mut p: [i32; Perlin::POINT_COUNT]) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        p.shuffle(&mut rng);
    }
}
