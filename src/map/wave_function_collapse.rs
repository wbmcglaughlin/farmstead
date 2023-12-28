use crate::map::indexing::get_surrounding_empty_index;

use super::{
    indexing::{get_surroudning_filled_index, xy_i},
    tile::Tiles,
};
use bevy::utils::hashbrown::HashSet;
use bevy_ecs_tilemap::{map::TilemapSize, tiles::TilePos};
use rand::Rng;

pub fn intersection(nums: Vec<Vec<Tiles>>) -> Vec<Tiles> {
    let mut intersect_result: Vec<Tiles> = nums[0].clone();

    for temp_vec in nums {
        let unique_a: HashSet<Tiles> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .copied()
            .collect::<Vec<_>>();
    }
    intersect_result
}

pub fn get_allowed_tiles(
    tile_array: &Vec<Option<Tiles>>,
    map_size: TilemapSize,
    index: usize,
) -> Vec<Tiles> {
    let filled_tiles = get_surroudning_filled_index(tile_array, map_size, index);
    let mut vecs = Vec::new();

    if filled_tiles.is_empty() {
        return vec![Tiles::Dirt];
    }
    for tile in filled_tiles {
        vecs.push(match tile_array[tile] {
            Some(tile) => match tile {
                Tiles::Field => vec![Tiles::Grass, Tiles::Field],
                Tiles::Grass => vec![Tiles::Grass, Tiles::Farmland, Tiles::Dirt],
                Tiles::Farmland => vec![Tiles::Grass, Tiles::Farmland, Tiles::Dirt],
                Tiles::Dirt => vec![Tiles::Grass, Tiles::Dirt, Tiles::Stone],
                Tiles::Stone => vec![Tiles::Stone, Tiles::Dirt],
                Tiles::Rock => vec![Tiles::Rock, Tiles::Stone],
            },
            None => vec![Tiles::Dirt],
        });
    }

    intersection(vecs)
}

pub fn populate_tilemap(map_size: TilemapSize) -> Vec<Option<Tiles>> {
    let mut tile_array: Vec<Option<Tiles>> = vec![None; (map_size.x * map_size.y) as usize];

    let current_pos = TilePos { x: 0, y: 0 };
    let mut queue = vec![xy_i(
        map_size,
        current_pos.x as usize,
        current_pos.y as usize,
    )]
    .into_iter()
    .collect::<HashSet<usize>>();

    while !queue.is_empty() {
        assert!(queue.len() < (map_size.x * map_size.y) as usize);

        let mut temp_queue: HashSet<usize> = HashSet::new();
        for pos in queue.iter() {
            let tiles = get_allowed_tiles(&tile_array, map_size, *pos);
            let tile = tiles
                .get(rand::thread_rng().gen_range(0..tiles.len()))
                .unwrap();
            tile_array[*pos] = Some(*tile);
        }
        for pos in queue.iter() {
            let empty: Vec<usize> = get_surrounding_empty_index(&tile_array, map_size, *pos);
            temp_queue.extend(empty);
        }

        queue = temp_queue;
    }

    tile_array
}
