use bevy::utils::hashbrown::HashSet;
use bevy_ecs_tilemap::{map::TilemapSize, tiles::TilePos};

use super::tile::Tiles;
use rand::Rng;

pub fn get_allowed_tiles(tile: Option<Tiles>) -> Vec<Tiles> {
    match tile {
        Some(tile) => match tile {
            Tiles::Field => vec![Tiles::Grass, Tiles::Field],
            Tiles::Grass => vec![Tiles::Grass, Tiles::Farmland, Tiles::Dirt],
            Tiles::Farmland => vec![Tiles::Grass, Tiles::Farmland],
            Tiles::Dirt => vec![Tiles::Grass, Tiles::Dirt],
            Tiles::Stone => vec![Tiles::Stone, Tiles::Dirt],
            Tiles::Rock => vec![Tiles::Rock, Tiles::Stone],
        },
        None => vec![Tiles::Dirt],
    }
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
            let tiles = get_allowed_tiles(tile_array[*pos]);
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

pub fn get_surrounding_index(map_size: TilemapSize, index: usize) -> Vec<usize> {
    let tilepos = i_xy(map_size, index);
    let mut surrounding = vec![];

    if tilepos.x > 0 {
        surrounding.push(xy_i(map_size, (tilepos.x - 1) as usize, tilepos.y as usize))
    }
    if tilepos.x < map_size.x - 1 {
        surrounding.push(xy_i(map_size, (tilepos.x + 1) as usize, tilepos.y as usize))
    }
    if tilepos.y > 0 {
        surrounding.push(xy_i(map_size, tilepos.x as usize, (tilepos.y - 1) as usize))
    }
    if tilepos.y < map_size.y - 1 {
        surrounding.push(xy_i(map_size, tilepos.x as usize, (tilepos.y + 1) as usize))
    }

    surrounding
}

pub fn get_surrounding_empty_index(
    tile_array: &Vec<Option<Tiles>>,
    map_size: TilemapSize,
    index: usize,
) -> Vec<usize> {
    let mut surrounding = get_surrounding_index(map_size, index);
    surrounding.retain(|i| tile_array[*i].is_none());

    surrounding
}

pub fn xy_i(map_size: TilemapSize, x: usize, y: usize) -> usize {
    (map_size.x as usize) * y + x
}

pub fn i_xy(map_size: TilemapSize, i: usize) -> TilePos {
    TilePos {
        x: i as u32 % map_size.x,
        y: i as u32 / map_size.x,
    }
}
