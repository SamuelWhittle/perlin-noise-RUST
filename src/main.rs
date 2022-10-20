use perlin_noise::PerlinNoise;

fn main () {
    let mut perlin = PerlinNoise::multi_octave_with_seed(vec![30, 30, 30], 3, 0.3f32, 0);

    //println!("normalizing vector [5, 6, 7]: {:?}", PerlinNoise::normalize_vector(vec![5, 6, 7]));
    println!("noise_value: {:?}", perlin.get_noise_value(vec![5.5, 5.5, 5.5]));
}
