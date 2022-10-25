use wasm_bindgen::prelude::*;

use std::hash::Hasher;
use twox_hash::XxHash64;
use rand::Rng;

//web-sys WASM in Web Worker
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlElement, HtmlInputElement, MessageEvent, Worker};

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

    // NOISEY BOYS
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

// UTILITY FUNCTIONS
#[wasm_bindgen]
pub fn range_map(num: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64) -> f64 {
    (num - old_min) / (old_max - old_min) * (new_max - new_min) + new_min
}

// Run entry point for the main thread
#[wasm_bindgen]
pub fn startup() {
    // Here, we create our worker. In a larger app, multiple callbacks should be
    // able to interact with the code in the worker. Therefore, we wrap it in
    // `Rc<RefCell>` following the interior mutability pattern. Here, it would
    // not be needed but we include the wrapping anyway as example.
    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within WASM".into());

    // Pass the worker to the function which sets up the `oninput` callback.
    setup_input_oninput_callback(worker_handle.clone());
}

fn setup_input_oninput_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    // If our `onmessage` callback should stay valid after exiting from the
    // `oninput` closure scope, we need to either forget it (so it is not
    // destroyed) or store it somewhere. To avoid leaking memory every time we
    // want to receive a response from the worker, we move a handle into the
    // `oninput` closure to which we will always attach the last `onmessage`
    // callback. The initial value will not be used and we silence the warning.
    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    let callback = Closure::new(move || {
        console::log_1(&"oninput callback triggered".into());
        let document = web_sys::window().unwrap().document().unwrap();

        let input_field = document
            .get_element_by_id("inputNumber")
            .expect("#inputNumber should exist");
        let input_field = input_field
            .dyn_ref::<HtmlInputElement>()
            .expect("#inputNumber should be a HtmlInputElement");

        // If the value in the field can be parsed to a `i32`, send it to the
        // worker. Otherwise clear the result field.
        match input_field.value().parse::<i32>() {
            Ok(number) => {
                // Access worker behind shared handle, following the interior
                // mutability pattern.
                let worker_handle = &*worker.borrow();
                let _ = worker_handle.post_message(&number.into());
                persistent_callback_handle = get_on_msg_callback();

                // Since the worker returns the message asynchronously, we
                // attach a callback to be triggered when the worker returns.
                worker_handle
                    .set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
            }
            Err(_) => {
                document
                    .get_element_by_id("resultField")
                    .expect("#resultField should exist")
                    .dyn_ref::<HtmlElement>()
                    .expect("#resultField should be a HtmlInputElement")
                    .set_inner_text("");
            }
        }
    });

    // Attach the closure as `oninput` callback to the input field.
    document
        .get_element_by_id("inputNumber")
        .expect("#inputNumber should exist")
        .dyn_ref::<HtmlInputElement>()
        .expect("#inputNumber should be a HtmlInputElement")
        .set_oninput(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory.
    callback.forget();
}

/// Create a closure to act on the message returned by the worker
fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    let callback = Closure::new(move |event: MessageEvent| {
        console::log_2(&"Received response: ".into(), &event.data().into());

        let result = match event.data().as_bool().unwrap() {
            true => "even",
            false => "odd",
        };

        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id("resultField")
            .expect("#resultField should exist")
            .dyn_ref::<HtmlElement>()
            .expect("#resultField should be a HtmlInputElement")
            .set_inner_text(result);
    });

    callback
}

