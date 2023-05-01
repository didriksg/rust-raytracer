use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::data_structs::vec3::{Color, Point3, Vec3};
use crate::materials::textures::Texture;

#[derive(Clone, Default)]
pub struct Perlin {
    random_vector: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count: usize = 256;

        Self {
            random_vector: (0..point_count).map(|_| Vec3::random_with_limits(-1.0, 1.0)).collect::<Vec<Vec3>>(),
            perm_x: Self::generate_permutation(point_count as i32),
            perm_y: Self::generate_permutation(point_count as i32),
            perm_z: Self::generate_permutation(point_count as i32),
        }
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        let floored_x = f64::floor(point.x);
        let floored_y = f64::floor(point.y);
        let floored_z = f64::floor(point.z);

        let u = point.x - floored_x;
        let v = point.y - floored_y;
        let w = point.z - floored_z;

        let i = floored_x as i32;
        let j = floored_y as i32;
        let k = floored_z as i32;

        let mut color_array = [[[Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    color_array[di][dj][dk] = self.random_vector[(
                        self.perm_x[((i + di as i32) & 255) as usize] ^
                            self.perm_y[((j + dj as i32) & 255) as usize] ^
                            self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize]
                }
            }
        }

        Self::perlin_interpolation(color_array, u, v, w)
    }

    fn perlin_interpolation(color_array: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        (0..2).map(|i| {
            (0..2).map(|j| {
                (0..2).map(|k| {
                    let if64 = i as f64;
                    let jf64 = j as f64;
                    let kf64 = k as f64;

                    let weight_vector = Vec3::new(u - if64, v - jf64, w - kf64);

                    (if64 * uu + (1.0 - if64) * (1.0 - uu))
                        * (jf64 * vv + (1.0 - jf64) * (1.0 - vv))
                        * (kf64 * ww + (1.0 - kf64) * (1.0 - ww))
                        * weight_vector.dot(color_array[i][j][k])
                }).sum::<f64>()
            }).sum::<f64>()
        }).sum::<f64>()
    }

    pub fn turbulence(&self, point: &Point3, depth: usize) -> f64 {
        let mut temp_point = *point;
        let mut weight = 1.0;

        let accumulated = (0..depth).map(|_| {
            let turb = weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point = temp_point * 2.0;

            turb
        }).sum();

        f64::abs(accumulated)
    }

    fn generate_permutation(point_count: i32) -> Vec<i32> {
        let mut permutation: Vec<i32> = (0..point_count).collect();
        permutation.shuffle(&mut thread_rng());

        permutation
    }
}

#[derive(Clone, Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self { noise: Perlin::new(), scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 *
            (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turbulence(p, 7)))
    }
}