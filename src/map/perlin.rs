use bevy_ecs_tilemap::map::TilemapSize;
use noise::{NoiseFn, Perlin, Seedable};

pub fn generate_perlin_noise_map(
    map_size: TilemapSize,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
    seed: u32,
) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(1);
    perlin.set_seed(seed);

    // TODO: this should be used...
    let mut scale = 1.0;

    let height = map_size.x as usize;
    let width = map_size.y as usize;

    let mut noise_map = vec![vec![0.0; height]; width];

    let mut max_noise_height = f64::NEG_INFINITY;
    let mut min_noise_height = f64::INFINITY;

    for o in 0..octaves {
        for y in 0..height {
            for x in 0..width {
                let sample_x = x as f64 / scale;
                let sample_y = y as f64 / scale;

                let noise_value = perlin.get([sample_x, sample_y]) * 2.0 - 1.0;
                noise_map[x][y] += noise_value / persistence.powi(o as i32);

                // TODO: why is this in this location, this sets the value each octave, why not just on the last.
                max_noise_height = noise_map[x][y].max(max_noise_height);
                min_noise_height = noise_map[x][y].min(min_noise_height);
            }
        }
        scale *= lacunarity;
    }

    for y in 0..height {
        for x in 0..width {
            noise_map[x][y] =
                (noise_map[x][y] - min_noise_height) / (max_noise_height - min_noise_height);
        }
    }

    noise_map
}
