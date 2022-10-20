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
    grid_dims: Vec<u64>,
    num_dims: u64,
    num_octaves: u64,
    octave_scale: f32,
    seed: u64,
    hasher: XxHash64,
    unit_corners: Vec<u64>
}

#[wasm_bindgen]
impl PerlinNoise {
    // CONSTRUCTORS
    pub fn multi_octave_with_seed(grid_dims: Vec<u64>, num_octaves: u64, octave_scale: f32, seed: u64) -> PerlinNoise {
        let unit_corners = PerlinNoise::calc_unit_corners(grid_dims.len());

        PerlinNoise {
            grid_dims: grid_dims.clone(),
            num_dims: grid_dims.len() as u64,
            num_octaves,
            octave_scale,
            seed,
            hasher: XxHash64::with_seed(seed as u64),
            unit_corners,
        }
    }

    pub fn single_octave_with_seed(grid_dims: Vec<u64>, seed: u64) -> PerlinNoise {
        let unit_corners = PerlinNoise::calc_unit_corners(grid_dims.len());

        PerlinNoise {
            grid_dims: grid_dims.clone(),
            num_dims: grid_dims.len() as u64,
            num_octaves: 1,
            octave_scale: 1.0,
            seed,
            hasher: XxHash64::with_seed(seed as u64),
            unit_corners,
        }
    }

    pub fn multi_octave(grid_dims: Vec<u64>, num_octaves: u64, octave_scale: f32) -> PerlinNoise {
        let unit_corners = PerlinNoise::calc_unit_corners(grid_dims.len());

        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();

        PerlinNoise {
            grid_dims: grid_dims.clone(),
            num_dims: grid_dims.len() as u64,
            num_octaves,
            octave_scale,
            seed,
            hasher: XxHash64::with_seed(seed as u64),
            unit_corners,
        }
    }

    pub fn single_octave(grid_dims: Vec<u64>) -> PerlinNoise {
        let unit_corners = PerlinNoise::calc_unit_corners(grid_dims.len());

        let mut rng = rand::thread_rng();
        let seed: u64 = rng.gen();

        PerlinNoise {
            grid_dims: grid_dims.clone(),
            num_dims: grid_dims.len() as u64,
            num_octaves: 1,
            octave_scale: 1.0,
            seed,
            hasher: XxHash64::with_seed(seed as u64),
            unit_corners,
        }
    }

    // UTILITY FUNCTIONS
    pub fn get_noise_value(&mut self, coords: Vec<f32>) -> f32 {
        println!("get_noise_value");

        let mut corner_vectors: Vec<u32> = Vec::new();

        for coord in self.calc_corner_coords(coords).iter() {
            self.hasher.write_u64(*coord);
            corner_vectors.push(self.hasher.finish() as u32);
        }

        let corner_count = 2u64.pow(self.num_dims as u32);

        println!("{:?}", corner_vectors);
        for corner in 0..corner_count {
            let corner_vec = corner_vectors[(corner * self.num_dims) as usize..=(corner * self.num_dims + self.num_dims - 1) as usize].to_vec();
            
            let normalized_corner_vec = PerlinNoise::normalize_vector(corner_vec.clone());

            println!("corner: {}, vector: {:?}, normalized_vector: {:?}", corner, corner_vec, normalized_corner_vec);
        }

        0.0
    }

    pub fn normalize_vector(vec: Vec<u32>) -> Vec<f32> {
        println!("normalize_vector");
        let mut magnitude: f32 = 0.0;

        for vec_comp in vec.iter() {
            magnitude += *vec_comp as f32 * *vec_comp as f32;
        }

        magnitude = magnitude.sqrt();
        println!("magnitude: {:?}", magnitude);

        let mut normal_vec: Vec<f32> = Vec::new();

        for vec_comp in vec.iter() {
            normal_vec.push(*vec_comp as f32 / magnitude);
        }

        normal_vec
    }

    pub fn calc_corner_coords(&self, coords: Vec<f32>) -> Vec<u64> {
        let corner_count = 2u64.pow(self.num_dims as u32);

        let mut corner_coords: Vec<u64> = Vec::new();

        for corner in 0..corner_count {
            for dim in 0..self.num_dims {
                corner_coords.push(coords[dim as usize].floor() as u64 + self.unit_corners[(corner * 3 + dim) as usize]);
            }
        }

        corner_coords
    }

    pub fn calc_unit_corners(num_dims: usize) -> Vec<u64> {
        let corner_count = 2u64.pow(num_dims as u32);

        let mut unit_corners = Vec::new();

        for corner in 0..corner_count {
            for bit in (0..num_dims).rev() {
                unit_corners.push(corner >> bit & 1);
            }
        }

        unit_corners
    }

    // BASIC GETS
    pub fn get_grid_dims(&self) -> Vec<u64> {
        self.grid_dims.clone()
    }

    pub fn get_num_octaves(&self) -> u64 {
        self.num_octaves
    }

    pub fn get_octave_scale(&self) -> f32 {
        self.octave_scale
    }

    pub fn get_unit_corners(&self) -> Vec<u64> {
        self.unit_corners.clone()
    }

    pub fn get_num_dims(&self) -> u64 {
        self.num_dims
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
