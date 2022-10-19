use wasm_bindgen::prelude::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
    grid_dims: Vec<usize>,
    num_dims: u32,
    num_octaves: u32,
    octave_scale: f32,
    seed: f32,
    unit_corners: Vec<u32>
}

#[wasm_bindgen]
impl PerlinNoise {
    pub fn define_multi_octave(grid_dims: Vec<usize>, num_octaves: u32, octave_scale: f32, seed: f32) -> PerlinNoise {
        let unit_corners = PerlinNoise::calc_unit_corners(grid_dims.len());

        PerlinNoise {
            grid_dims: grid_dims.clone(),
            num_dims: grid_dims.len() as u32,
            num_octaves,
            octave_scale,
            seed,
            unit_corners,
        }
    }

    pub fn define_single_octave(grid_dims: Vec<usize>, seed: f32) -> PerlinNoise {
        let unit_corners = PerlinNoise::calc_unit_corners(grid_dims.len());

        PerlinNoise {
            grid_dims: grid_dims.clone(),
            num_dims: grid_dims.len() as u32,
            num_octaves: 1,
            octave_scale: 1.0,
            seed,
            unit_corners,
        }
    }

    pub fn get_noise_value(&self, coords: Vec<f32>) -> f32 {

        0.0
    }

    pub fn calc_corner_coords(&self, coords: Vec<f32>) -> Vec<u32> {
        let corner_count = 2u32.pow(self.num_dims);

        let mut corner_coords: Vec<u32> = Vec::new();

        for corner in 0..corner_count {
            for dim in 0..self.num_dims {
                corner_coords.push(coords[dim as usize].floor() as u32 + self.unit_corners[(corner * 3 + dim) as usize]);
            }
        }

        corner_coords
    }

    pub fn calc_unit_corners(num_dims: usize) -> Vec<u32>{
        let corner_count = 2u32.pow(num_dims as u32);

        let mut unit_corners = Vec::new();

        for corner in 0..corner_count {
            for bit in (0..num_dims).rev() {
                unit_corners.push(corner >> bit & 1);
            }
        }

        unit_corners
    }

    // BASIC GETS
    pub fn get_grid_dims(&self) -> Vec<usize> {
        self.grid_dims.clone()
    }

    pub fn get_num_octaves(&self) -> u32 {
        self.num_octaves
    }

    pub fn get_octave_scale(&self) -> f32 {
        self.octave_scale
    }

    pub fn get_unit_corners(&self) -> Vec<u32> {
        self.unit_corners.clone()
    }

    pub fn get_num_dims(&self) -> u32 {
        self.num_dims
    }

    // BASIC SETS
    pub fn set_num_octaves(&mut self, num_octaves: u32) {
        self.num_octaves = num_octaves
    }

    pub fn set_octave_scale(&mut self, octave_scale: f32) {
        self.octave_scale = octave_scale
    }
}
