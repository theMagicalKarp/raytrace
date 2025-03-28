use itertools::iproduct;
use nalgebra::Vector3;
use rand::prelude::*;

const PERLIN_POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    randvec: Box<[Vector3<f32>; PERLIN_POINT_COUNT]>,
    perm_x: Box<[usize; PERLIN_POINT_COUNT]>,
    perm_y: Box<[usize; PERLIN_POINT_COUNT]>,
    perm_z: Box<[usize; PERLIN_POINT_COUNT]>,
}

impl Default for Perlin {
    fn default() -> Self {
        Perlin::new()
    }
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut randvec = [Vector3::<f32>::default(); PERLIN_POINT_COUNT];
        for vec in randvec.iter_mut() {
            *vec = Vector3::new(
                rng.random_range(-1.0f32..1.0f32),
                rng.random_range(-1.0f32..1.0f32),
                rng.random_range(-1.0f32..1.0f32),
            )
            .normalize();
        }

        let mut perm_x = Box::new([0; PERLIN_POINT_COUNT]);
        Perlin::perlin_generate_perm(&mut perm_x);

        let mut perm_y = Box::new([0; PERLIN_POINT_COUNT]);
        Perlin::perlin_generate_perm(&mut perm_y);

        let mut perm_z = Box::new([0; PERLIN_POINT_COUNT]);
        Perlin::perlin_generate_perm(&mut perm_z);

        Perlin {
            randvec: Box::new(randvec),
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: Vector3<f32>) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = (point.x.floor()) as i32;
        let j = (point.y.floor()) as i32;
        let k = (point.z.floor()) as i32;
        let mut c = [[[Vector3::<f32>::default(); 2]; 2]; 2];

        iproduct!(0..2, 0..2, 0..2).for_each(|(di, dj, dk)| {
            let xi = ((i + di) & 255) as usize;
            let yi = ((j + dj) & 255) as usize;
            let zi = ((k + dk) & 255) as usize;
            c[di as usize][dj as usize][dk as usize] =
                self.randvec[self.perm_x[xi] ^ self.perm_y[yi] ^ self.perm_z[zi]];
        });

        Perlin::perlin_interp(&c, u, v, w)
    }

    pub fn perlin_generate_perm(p: &mut [usize; PERLIN_POINT_COUNT]) {
        p.iter_mut().enumerate().for_each(|(i, p_i)| *p_i = i);
        Perlin::permute(p, PERLIN_POINT_COUNT);
    }

    pub fn permute(p: &mut [usize; PERLIN_POINT_COUNT], n: usize) {
        let mut rng = rand::rng();
        (1..n).rev().for_each(|i| p.swap(i, rng.random_range(0..i)));
    }

    pub fn turb(&self, point: Vector3<f32>, depth: u32) -> f32 {
        let mut temp_p = point;
        let mut weight = 1.0;
        (0..depth)
            .map(|_| {
                let val = weight * self.noise(temp_p);
                weight *= 0.5;
                temp_p *= 2.0;
                val
            })
            .sum::<f32>()
            .abs()
    }

    pub fn perlin_interp(c: &[[[Vector3<f32>; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        iproduct!(0..2, 0..2, 0..2)
            .map(|(i, j, k)| {
                let fi = i as f32;
                let ji = j as f32;
                let ki = k as f32;

                let weight_v = Vector3::<f32>::new(u - fi, v - ji, w - ki);

                (fi * uu + (1.0 - fi) * (1.0 - uu))
                    * (ji * vv + (1.0 - ji) * (1.0 - vv))
                    * (ki * ww + (1.0 - ki) * (1.0 - ww))
                    * c[i][j][k].dot(&weight_v)
            })
            .sum()
    }
}
