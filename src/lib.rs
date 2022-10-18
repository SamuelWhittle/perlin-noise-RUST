use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(message: &str) {
    alert(message);
}

#[wasm_bindgen]
pub struct PerlinNoise {
    grid_dims: Vec<f64>,
    num_octaves: f64,
    octave_scale: f64,
    unit_corners: Vec<u8>
}

#[wasm_bindgen]
impl PerlinNoise {
    pub fn define_multi_octave(dims_js_arr: Array, num_octaves: f64, octave_scale: f64) -> PerlinNoise {
        let grid_dims = PerlinNoise::get_vec_from_js_array(dims_js_arr);
        let unit_corners = PerlinNoise::calc_unit_corners(grid_dims.len());

        PerlinNoise {
            grid_dims,
            num_octaves,
            octave_scale,
            unit_corners,
        }
    }

    pub fn define_single_octave(dims_js_arr: Array) -> PerlinNoise {
        let grid_dims = PerlinNoise::get_vec_from_js_array(dims_js_arr);
        let unit_corners = PerlinNoise::calc_unit_corners(grid_dims.len());

        PerlinNoise {
            grid_dims,
            num_octaves: 1.0,
            octave_scale: 1.0,
            unit_corners,
        }
    }

    pub fn get_noise_value(&self, coords_js_arr: Array) -> u32 {
        let mut coords = PerlinNoise::get_vec_from_js_array(coords_js_arr);

        0
    }

    pub fn calc_unit_corners(num: usize) -> Vec<u8>{
        let corner_count = 2u8.pow(num as u32);

        let mut unit_corners = Vec::new();

        for corner in 0..corner_count {
            for bit in (0..num).rev() {
                unit_corners.push(corner >> bit & 1);
            }
        }

        unit_corners
    }

    pub fn get_vec_from_js_array(arr: Array) -> Vec<f64> {
        let mut vec: Vec<f64> = Vec::new();

        for i in 0..arr.length() {
            match arr.get(i).as_f64() {
                None => {},
                Some(coord) => {
                    vec.push(coord);
                }
            }
        }

        vec
    }

    pub fn get_unit_corners(&self) -> Vec<u8> {
        self.unit_corners.clone()
    }

    pub fn get_grid_dims(&self) -> Vec<f64> {
        self.grid_dims.clone()
    }

    pub fn get_num_octaves(&self) -> f64 {
        self.num_octaves
    }

    pub fn get_octave_scale(&self) -> f64 {
        self.octave_scale
    }

    pub fn set_num_octaves(&mut self, num_octaves: f64) {
        self.num_octaves = num_octaves
    }

    pub fn set_octave_scale(&mut self, octave_scale: f64) {
        self.octave_scale = octave_scale
    }
}
