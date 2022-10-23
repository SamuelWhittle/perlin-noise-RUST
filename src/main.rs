use perlin_noise::PerlinNoise;

fn main() {
    // SINGLE_DIM -> (POS, NOISE)
    let mut noise = PerlinNoise::multi_octave_with_seed(3, 1.0/3.0, 4);

    for i in 0..100000 {
        let pos: f32 = i as f32 / 1000.0;
        println!("({:.4},{:.4})", pos, noise.get_fractal_noise_value(vec![0.5, 0.5, pos]));
    }

    // SINGLE_DIM -> (POS, NOISE + SEED) FOR N SEEDS
    /*for seed in 0..4 {
        let mut noise = PerlinNoise::multi_octave_with_seed(4, 0.5, seed);

        for i in 0..100 {
            let pos: f32 = i as f32 / 10.0;
            println!("({:.4},{:.4})", pos, noise.get_fractal_noise_value(vec![pos]) + seed as f32 * 1.5);
        }
    }*/

    /*let mut noise = PerlinNoise::multi_octave_with_seed(4, 0.5, 0);

    for y in 0..4 {
        for x in 0..100 {
            let x_pos: f32 = x as f32 / 10.0;
            let y_pos: f32 = y as f32 / 10.0;
            println!("({:.4},{:.4})", x_pos, noise.get_fractal_noise_value(vec![x_pos, y_pos]) + y as f32 * 1.5);
        }
    }*/
}
