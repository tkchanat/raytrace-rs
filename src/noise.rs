use crate::math::*;
// Perlin
#[derive(Clone)]
pub struct Perlin {
    perm_x: [i32; Perlin::POINT_COUNT],
    perm_y: [i32; Perlin::POINT_COUNT],
    perm_z: [i32; Perlin::POINT_COUNT],
    rand_float: [f64; Perlin::POINT_COUNT],
}
impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rand_float = [0.0; Perlin::POINT_COUNT];
        for e in rand_float.iter_mut() {
            *e = random_double();
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();
        Perlin {
            perm_x,
            perm_y,
            perm_z,
            rand_float,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = ((4.0 * p.x()) as usize) & 255;
        let j = ((4.0 * p.y()) as usize) & 255;
        let k = ((4.0 * p.z()) as usize) & 255;
        self.rand_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
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
