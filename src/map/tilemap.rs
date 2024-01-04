use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::time::SystemTime;

use super::{
    perlin::generate_perlin_noise_map,
    tile::{Tiles, WaterTiles},
};

#[derive(Component)]
pub struct MainTileMap;

#[derive(Component)]
pub struct JobLayerTileMap;

#[derive(Component)]
pub struct WaterTileMap;

#[derive(Component)]
pub struct PlantTileLayer;

pub(crate) fn generate_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // This should be constant.
    let map_size = TilemapSize { x: 128, y: 128 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    // Generate the perlin noise map using the system time as the map generation seed.
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;
    let perlin_map = generate_perlin_noise_map(map_size, 6, 0.5, 2.0, seed);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            let val = perlin_map[x as usize][y as usize];
            let tile = tile_height_mapping(val);

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile.get_texture_index()),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
        MainTileMap,
    ));

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    let texture_handle: Handle<Image> = asset_server.load("water.png");

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            let val = perlin_map[x as usize][y as usize];
            let tile = water_height(0.3, val);
            let tile_index = if let Some(tile_val) = tile {
                tile_val.get_texture_index()
            } else {
                0
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_index),
                    color: TileColor(Color::hsla(180.0, 0.5, 0.5, 0.3)),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.5),
            ..Default::default()
        },
        WaterTileMap,
    ));

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    let texture_handle: Handle<Image> = asset_server.load("jobs_layer.png");

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
            ..Default::default()
        },
        JobLayerTileMap,
    ));

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    let texture_handle: Handle<Image> = asset_server.load("wheat.png");

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.5),
            ..Default::default()
        },
        PlantTileLayer,
    ));
}

fn tile_height_mapping(val: f64) -> Tiles {
    if val > 0.85 {
        Tiles::Stone
    } else if val > 0.7 {
        Tiles::Rock
    } else if val > 0.65 {
        Tiles::Grass
    } else if val > 0.10 {
        Tiles::Field
    } else {
        Tiles::Dirt
    }
}

fn water_height(water_height: f64, val: f64) -> Option<WaterTiles> {
    let offset = 0.1;
    let mut tile = None;
    if val < water_height {
        tile = Some(WaterTiles::LowWater);
    }
    if val + offset < water_height {
        tile = Some(WaterTiles::MediumWater);
    }

    tile
}
