use bevy_ecs_tilemap::map::TilemapSize;
use noise::{NoiseFn, Perlin, Seedable};

pub struct PerlinNoiseSeed {
    pub octaves: usize,
    pub persistence: f64,
    pub lacunarity: f64,
    pub scale: f64,
    pub seed: u32,
}

pub fn generate_perlin_noise_map(
    map_size: TilemapSize,
    perlin_seed: PerlinNoiseSeed,
) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(1).set_seed(perlin_seed.seed);

    let height = map_size.x as usize;
    let width = map_size.y as usize;
    let mut scale = perlin_seed.scale;

    let mut noise_map = vec![vec![0.0; height]; width];

    let mut max_noise_height = f64::NEG_INFINITY;
    let mut min_noise_height = f64::INFINITY;

    for o in 0..perlin_seed.octaves {
        for x in 0..width {
            for y in 0..height {
                let sample_x = x as f64 / scale;
                let sample_y = y as f64 / scale;

                let noise_value = perlin.get([sample_x, sample_y]) * 2.0 - 1.0;
                noise_map[x][y] += noise_value / perlin_seed.persistence.powi(o as i32);

                // TODO: why is this in this location, this sets the value each octave, why not just on the last.
                max_noise_height = noise_map[x][y].max(max_noise_height);
                min_noise_height = noise_map[x][y].min(min_noise_height);
            }
        }
        scale *= perlin_seed.lacunarity;
    }

    for row in noise_map.iter_mut() {
        for val in row.iter_mut() {
            *val = (*val - min_noise_height) / (max_noise_height - min_noise_height);
        }
    }

    noise_map
}
