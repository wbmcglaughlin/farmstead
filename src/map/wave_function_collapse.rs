use bevy_ecs_tilemap::{map::TilemapSize, tiles::TilePos};

use super::tile::Tiles;

pub fn populate_tilemap(map_size: TilemapSize) -> Vec<Option<Tiles>> {
    let mut tilearray: Vec<Option<Tiles>> = vec![None; (map_size.x * map_size.y) as usize];

    let current_tile = Tiles::Grass;
    let current_pos = TilePos { x: 0, y: 0 };

    tilearray
}

pub fn get_surrounding_index(map_size: TilemapSize, index: usize) -> Vec<usize> {
    todo!();
}

pub fn get_surrounding_empty_index(map_size: TilemapSize, index: usize) -> Vec<usize> {
    todo!();
}
