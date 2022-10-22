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
    num_octaves: u64,
    octave_scale: f32,
    seed: u64,
}

#[wasm_bindgen]
impl PerlinNoise {
    // CONSTRUCTORS
    pub fn multi_octave_with_seed(num_octaves: u64, octave_scale: f32, seed: u64) -> PerlinNoise {
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

    pub fn multi_octave(num_octaves: u64, octave_scale: f32) -> PerlinNoise {
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
    pub fn get_fractal_noise_value(&mut self, coords: Vec<f32>) -> f32 {
        let mut noise_value: f32 = 0.0;

        for octave in 0..self.num_octaves {
            let octave_coords: Vec<f32> = coords.clone().iter().map(|coord| {
                coord / self.octave_scale.powi(octave as i32)
            }).collect();

            let mut dot_products: Vec<f32> = Vec::new();

            let corner_count = 2u64.pow(coords.len() as u32);

            for corner in 0..corner_count {
                let unit_corner = PerlinNoise::get_unit_corner(corner, coords.len());

                let corner_coords = PerlinNoise::calc_corner_coords(octave_coords.clone(), unit_corner);

                let mut hasher: XxHash64 = XxHash64::with_seed(self.seed);
                let mut corner_vec: Vec<f32> = corner_coords.iter().map(|coord|  {
                    hasher.write_i32((coord.round() + octave as f32)as i32);
                    (hasher.finish() as i64 % 256_i64) as f32
                }).collect();

                corner_vec = PerlinNoise::normalize_vector(corner_vec);

                let offset_vec: Vec<f32> = PerlinNoise::get_offset_vector(corner_coords, octave_coords.clone());

                dot_products.push(PerlinNoise::get_dot_product(corner_vec, offset_vec));
            }

            for dim in (0..coords.len()).rev() {
                dot_products = PerlinNoise::interp_dim(dot_products, octave_coords[dim as usize] % 1.0);
            }

            noise_value += dot_products[0] * self.octave_scale.powi(octave as i32);
            //println!("({:.3}, {:?})", coords[0] + 2.0 * (octave + 1) as f32, dot_products[0] * self.octave_scale.powi(octave as i32));
            //println!("octave_coords: {:?}, octave: {}, noise_value: {}", octave_coords, octave, noise_value);
        }

        noise_value
    }

    pub fn get_noise_value(&mut self, coords: Vec<f32>) -> f32 {
        let mut noise_value: f32 = 0.0;

        let mut dot_products: Vec<f32> = Vec::new();

        let corner_count = 2u64.pow(coords.len() as u32);

        for corner in 0..corner_count {
            let unit_corner = PerlinNoise::get_unit_corner(corner, coords.len());

            let corner_coords = PerlinNoise::calc_corner_coords(coords.clone(), unit_corner);

            let mut hasher: XxHash64 = XxHash64::with_seed(self.seed);
            let mut corner_vec: Vec<f32> = corner_coords.iter().map(|coord|  {
                hasher.write_i32(coord.round() as i32);
                (hasher.finish() as i64 % 256_i64) as f32
            }).collect();

            corner_vec = PerlinNoise::normalize_vector(corner_vec);

            let offset_vec: Vec<f32> = PerlinNoise::get_offset_vector(corner_coords, coords.clone());

            dot_products.push(PerlinNoise::get_dot_product(corner_vec, offset_vec));
        }

        for dim in (0..coords.len()).rev() {
            dot_products = PerlinNoise::interp_dim(dot_products, coords[dim as usize] % 1.0);
        }

        noise_value += dot_products[0];

        noise_value
    }

    pub fn interp_dim(values: Vec<f32>, weight: f32) -> Vec<f32> {
        //println!("smerp_dim vals: {:?}", values);
        let mut new: Vec<f32> = Vec::new();

        for i in 0..values.len() {
            if i % 2 == 0 {
                new.push(PerlinNoise::smerp(values[i], values[i+1], weight));
            }
        }

        new
    }

    pub fn lerp(start: f32, stop: f32, weight: f32) -> f32 {
        (stop - start) * weight + start
        //return (a1 - a0) * w + a0
    }

    pub fn smerp(start: f32, stop: f32, weight: f32) -> f32 {
        //println!("start: {}, stop: {}, weight: {}, value: {}", start, stop, weight, (stop - start) * ((weight * (weight * 6.0 - 15.0) + 10.0) * weight.powi(3)) + start);
        (stop - start) * ((weight * (weight * 6.0 - 15.0) + 10.0) * weight.powi(3)) + start
        //return (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
        //(stop - start) * (3.0 - weight * 2.0) * weight * weight + start
        //return (a1 - a0) * (3.0 - w * 2.0) * w * w + a0
    }

    pub fn get_dot_product(vector_a: Vec<f32>, vector_b: Vec<f32>) -> f32 {
        let mut dot_product: f32 = 0.0;
        
        for (i, comp) in vector_a.iter().enumerate() {
            dot_product += comp * vector_b[i];
        }

        dot_product
    }

    // Get Vector from point_a to point_b
    pub fn get_offset_vector(point_a: Vec<f32>, point_b: Vec<f32>) -> Vec<f32> {
        let mut new_vec: Vec<f32> = Vec::new();

        for (i, coord) in point_a.iter().enumerate() {
            //println!("point_a coord: {}, point_b coord: {}", coord, point_b[i]);
            new_vec.push((point_b[i] - coord)/* / (point_a.len() as f32).sqrt()*/);
        }

        new_vec
    }

    pub fn normalize_vector(vec: Vec<f32>) -> Vec<f32> {
        let mut magnitude: f32 = 0.0;

        for vec_comp in vec.iter() {
            magnitude += *vec_comp as f32 * *vec_comp as f32;
        }

        magnitude = magnitude.sqrt();

        let mut normal_vec: Vec<f32> = Vec::new();

        for vec_comp in vec.iter() {
            normal_vec.push(*vec_comp as f32 / magnitude);
        }

        normal_vec
    }

    pub fn calc_corner_coords(coords: Vec<f32>, unit_corner: Vec<f32>) -> Vec<f32> {
        let mut corner_coords: Vec<f32> = Vec::new();

        for dim in 0..coords.len() {
            corner_coords.push(coords[dim as usize].floor() + unit_corner[dim as usize]);
        }

        corner_coords
    }

    pub fn get_unit_corner(num: u64, num_dims: usize) -> Vec<f32> {
        let mut bin_vec: Vec<f32> = Vec::new();

        for bit in (0..num_dims).rev() {
            bin_vec.push(((num >> bit & 1) as f32).round());
        }

        bin_vec
    }

    // BASIC GETS
    pub fn get_num_octaves(&self) -> u64 {
        self.num_octaves
    }

    pub fn get_octave_scale(&self) -> f32 {
        self.octave_scale
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    // BASIC SETS
    pub fn set_num_octaves(&mut self, num_octaves: u64) {
        self.num_octaves = num_octaves
    }

    pub fn set_octave_scale(&mut self, octave_scale: f32) {
        self.octave_scale = octave_scale
    }

    pub fn set_seed(&mut self, new_seed: u64) {
        self.seed = new_seed
    }
}
