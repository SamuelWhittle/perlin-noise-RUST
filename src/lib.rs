use wasm_bindgen::prelude::*;

use std::hash::Hasher;
use twox_hash::XxHash64;
use rand::Rng;

/*#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}*/

#[wasm_bindgen]
pub struct PerlinNoise {
    num_octaves: u32,
    octave_scale: f64,
    seed: u64,
}

#[wasm_bindgen]
impl PerlinNoise {
    // CONSTRUCTORS
    pub fn multi_octave_with_seed(num_octaves: u32, octave_scale: f64, seed: u64) -> PerlinNoise {
        PerlinNoise {
            num_octaves,
            octave_scale,
            seed,
        }
    }

    pub fn single_octave_with_seed(seed: u64) -> PerlinNoise {
        PerlinNoise {
            num_octaves: 1,
            octave_scale: 1.0,
            seed,
        }
    }

    pub fn multi_octave(num_octaves: u32, octave_scale: f64) -> PerlinNoise {
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();

        PerlinNoise {
            num_octaves,
            octave_scale,
            seed,
        }
    }

    pub fn single_octave() -> PerlinNoise {
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();

        PerlinNoise {
            num_octaves: 1,
            octave_scale: 1.0,
            seed,
        }
    }

    // UTILITY FUNCTIONS
    
    pub fn get_noise_array(&mut self, positions: &[f64], dimensions: usize) -> Vec<f64> {
        (0..positions.len()).step_by(dimensions).map(|index| {
            self.get_fractal_noise_value(&positions[index..index+dimensions])
        }).collect()
    }

    pub fn get_fractal_noise_value(&mut self, coords: &[f64]) -> f64 {
        let max_potential_length = ((coords.len() as u64 * 127_u64.pow(2)) as f64).sqrt();

        (0..self.num_octaves).fold(0_f64,|acc, octave| {
            let octave_coords: Vec<f64> = coords.iter().map(|coord| {
                (coord / self.octave_scale.powi(octave as i32)) + std::f64::consts::PI * 100.0 * octave as f64
            }).collect();

            let corner_count = 1 << coords.len() as u64;

            let dot_products: Vec<f64> = (0..corner_count).map(|corner| {
                let corner_coords = PerlinNoise::calc_corner_coords(&octave_coords, corner);
                
                let corner_vec: Vec<i64> = (0..corner_coords.len()).map(|index| {
                    let mut hasher: XxHash64 = XxHash64::with_seed(self.seed);

                    hasher.write_i32((index..corner_coords.len()).rev().chain(0..index + 1)
                        .map(|corner_index| corner_coords[corner_index]).enumerate()
                        .fold(0, |accumulator, (reduce_index, corner_coord)| {
                            accumulator^(corner_coord << reduce_index)
                        }) as i32);
                    hasher.finish() as i64 %256_i64
                }).collect();

                let offset_vec: Vec<f64> = PerlinNoise::get_offset_vector(&corner_coords, &octave_coords);

                PerlinNoise::get_dot_product(corner_vec, offset_vec)
            }).collect();

            let octave_noise = (0..coords.len()).fold(dot_products, |acc, dim| {
                (0..acc.len() / 2).map(|i| {
                    PerlinNoise::smerp(acc[i * 2], acc[(i * 2)+1], octave_coords[dim as usize] % 1.0)
                }).collect()
            })[0];
            //let octave_noise = (0..coords.len()).fold(dot_products, |acc, dim| self.interp_dim(acc, octave_coords[dim as usize] % 1.0))[0];

            acc + (octave_noise * self.octave_scale.powi(octave as i32) / max_potential_length)
        })
    }

    pub fn range_map(num: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
        (num - old_min) / (old_max - old_min) * (new_max - new_min) + new_min
    }

    // BASIC GETS
    pub fn get_num_octaves(&self) -> u32 {
        self.num_octaves
    }

    pub fn get_octave_scale(&self) -> f64 {
        self.octave_scale
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    // BASIC SETS
    pub fn set_num_octaves(&mut self, num_octaves: u32) {
        self.num_octaves = num_octaves
    }

    pub fn set_octave_scale(&mut self, octave_scale: f64) {
        self.octave_scale = octave_scale
    }

    pub fn set_seed(&mut self, new_seed: u64) {
        self.seed = new_seed
    }
}

impl PerlinNoise {
    /*fn lerp(start: f64, stop: f64, weight: f64) -> f64 {
        (stop - start) * weight + start
        //return (a1 - a0) * w + a0
    }*/

    fn smerp(start: f64, stop: f64, weight: f64) -> f64 {
        (stop - start) * ((weight * (weight * 6.0 - 15.0) + 10.0) * weight.powi(3)) + start
        //(stop - start) * (3.0 - weight * 2.0) * weight * weight + start
    }

    fn get_dot_product(vector_a: Vec<i64>, vector_b: Vec<f64>) -> f64 {
        vector_a.iter().zip(vector_b.iter()).map(|(&vec_a_comp, &vec_b_comp)| vec_a_comp as f64 * vec_b_comp).sum()
    }

    // Get Vector from corner_coords to coords
    fn get_offset_vector(corner_coords: &[u64], coords: &[f64]) -> Vec<f64> {
        corner_coords.iter()
            .zip(coords.iter())
            .map(|(&corner_coord, &coord)| coord - corner_coord as f64)
            .collect()
    }

    /*fn normalize_vector(vec: &[f64]) -> Vec<f64> {
        let mag: f64 = vec.iter().map(|val| val.powi(2)).sum::<f64>().sqrt();

        vec.iter().map(|val| val / mag).collect()
    }*/

    fn calc_corner_coords(coords: &[f64], corner: u64) -> Vec<u64> {
        //let unit_corner = PerlinNoise::get_unit_corner(corner, coords.len());
        let unit_corner: Vec<u64> = (0..coords.len()).map(|bit| corner >> bit & 1).collect();

        coords.iter().enumerate().map(|(index, &coord)| coord as u64 + unit_corner[index]).collect()
    }
}
