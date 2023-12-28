use bevy_ecs_tilemap::{map::TilemapSize, tiles::TilePos};

use super::tile::Tiles;

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

pub fn get_surroudning_filled_index(
    tile_array: &Vec<Option<Tiles>>,
    map_size: TilemapSize,
    index: usize,
) -> Vec<usize> {
    let mut surrounding = get_surrounding_index(map_size, index);
    surrounding.retain(|i| tile_array[*i].is_some());

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
