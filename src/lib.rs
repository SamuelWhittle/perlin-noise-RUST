use wasm_bindgen::prelude::*;

use std::hash::Hasher;
use twox_hash::XxHash64;
use rand::Rng;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct PerlinNoise {
    num_octaves: u32,
    octave_scale: f32,
    seed: u64,
    interp_func: fn(f32, f32, f32) -> f32
}

#[wasm_bindgen]
impl PerlinNoise {
    // CONSTRUCTORS
    pub fn multi_octave_with_seed(num_octaves: u32, octave_scale: f32, seed: u64) -> PerlinNoise {
        PerlinNoise {
            num_octaves,
            octave_scale,
            seed,
            interp_func: PerlinNoise::smerp
        }
    }

    pub fn single_octave_with_seed(seed: u64) -> PerlinNoise {
        PerlinNoise {
            num_octaves: 1,
            octave_scale: 1.0,
            seed,
            interp_func: PerlinNoise::smerp
        }
    }

    pub fn multi_octave(num_octaves: u32, octave_scale: f32) -> PerlinNoise {
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();

        PerlinNoise {
            num_octaves,
            octave_scale,
            seed,
            interp_func: PerlinNoise::smerp
        }
    }

    pub fn single_octave() -> PerlinNoise {
        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();

        PerlinNoise {
            num_octaves: 1,
            octave_scale: 1.0,
            seed,
            interp_func: PerlinNoise::smerp
        }
    }

    // UTILITY FUNCTIONS
    
    pub fn get_noise_img_data(&mut self, positions: &[f32], dimensions: usize) -> Vec<f32> {
        (0..positions.len()).step_by(dimensions).map(|index| {
            self.get_fractal_noise_value(positions[index..index+dimensions].to_vec())
        }).collect()
    }

    pub fn get_fractal_noise_value(&mut self, coords: Vec<f32>) -> f32 {
        (0..self.num_octaves).fold(0_f32,|acc, octave| {
            let octave_coords: Vec<f32> = coords.clone().iter().map(|coord| {
                coord / self.octave_scale.powi(octave as i32)
            }).collect();

            let corner_count = 1 << coords.len() as u64;

            let dot_products = (0..corner_count).map(|corner| {
                let corner_coords = PerlinNoise::calc_corner_coords(octave_coords.clone(), corner);

                let mut corner_vec: Vec<f32> = (0..corner_coords.len()).map(|index| {
                    let mut hasher: XxHash64 = XxHash64::with_seed(self.seed);

                    hasher.write_i32((index..corner_coords.len()).rev().chain(0..index + 1)
                        .map(|corner_index| corner_coords[corner_index]).enumerate()
                        .fold(0, |accumulator, (reduce_index, corner_coord)| {
                            accumulator^(corner_coord << reduce_index)
                        }) as i32);
                    (hasher.finish() as i64) as f32
                }).collect();

                corner_vec = PerlinNoise::normalize_vector(corner_vec);

                let offset_vec: Vec<f32> = PerlinNoise::get_offset_vector(corner_coords, octave_coords.clone());

                PerlinNoise::get_dot_product(corner_vec, offset_vec)
            }).collect();

            let octave_noise = (0..coords.len()).fold(dot_products, |acc, dim| self.interp_dim(acc, octave_coords[dim as usize] % 1.0))[0];

            acc + octave_noise * 2.0 / (coords.len() as f32).sqrt() * self.octave_scale.powi(octave as i32)
        })
    }

    pub fn interp_dim(&self, values: Vec<f32>, weight: f32) -> Vec<f32> {
        (0..values.len() / 2).map(|i| {
            (self.interp_func)(values[i * 2], values[(i * 2)+1], weight)
        }).collect()
    }

    pub fn lerp(start: f32, stop: f32, weight: f32) -> f32 {
        (stop - start) * weight + start
        //return (a1 - a0) * w + a0
    }

    pub fn smerp(start: f32, stop: f32, weight: f32) -> f32 {
        (stop - start) * ((weight * (weight * 6.0 - 15.0) + 10.0) * weight.powi(3)) + start
        //(stop - start) * (3.0 - weight * 2.0) * weight * weight + start
    }

    pub fn get_dot_product(vector_a: Vec<f32>, vector_b: Vec<f32>) -> f32 {
        vector_a.iter().zip(vector_b.iter()).map(|(&vec_a_comp, &vec_b_comp)| vec_a_comp * vec_b_comp).sum()
    }

    // Get Vector from corner_coords to coords
    pub fn get_offset_vector(corner_coords: Vec<u64>, coords: Vec<f32>) -> Vec<f32> {
        corner_coords.iter()
            .zip(coords.iter())
            .map(|(&corner_coord, &coord)| coord - corner_coord as f32)
            .collect()
    }

    pub fn normalize_vector(vec: Vec<f32>) -> Vec<f32> {
        let mag: f32 = vec.iter().map(|val| val.powi(2)).sum::<f32>().sqrt();

        vec.iter().map(|val| val / mag).collect()
    }

    pub fn calc_corner_coords(coords: Vec<f32>, corner: u64) -> Vec<u64> {
        //let unit_corner = PerlinNoise::get_unit_corner(corner, coords.len());
        let unit_corner: Vec<u64> = (0..coords.len()).map(|bit| corner >> bit & 1).collect();

        coords.iter().enumerate().map(|(index, &coord)| coord as u64 + unit_corner[index]).collect()
    }

    pub fn range_map(num: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
        (num - old_min) / (old_max - old_min) * (new_max - new_min) + new_min
    }

    // BASIC GETS
    pub fn get_num_octaves(&self) -> u32 {
        self.num_octaves
    }

    pub fn get_octave_scale(&self) -> f32 {
        self.octave_scale
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    // BASIC SETS
    pub fn set_num_octaves(&mut self, num_octaves: u32) {
        self.num_octaves = num_octaves
    }

    pub fn set_octave_scale(&mut self, octave_scale: f32) {
        self.octave_scale = octave_scale
    }

    pub fn set_seed(&mut self, new_seed: u64) {
        self.seed = new_seed
    }

    pub fn set_interp_func(&mut self, func_name: &str) {
        match func_name {
            "smerp" => {
                self.interp_func = PerlinNoise::smerp;
            },
            "lerp" => {
                self.interp_func = PerlinNoise::lerp;
            },
            _ => {
                console_log!("{} is not a valid interp_func name", func_name);
            }
        }
    }
}
