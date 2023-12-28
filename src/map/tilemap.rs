use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{perlin::generate_perlin_noise_map, tile::Tiles};

#[derive(Component)]
pub struct MainTileMap;

pub(crate) fn generate_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 128, y: 128 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    let perlin_map = generate_perlin_noise_map(map_size, 6, 0.5, 2.0, 42);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            // TODO: this should be moved.
            let val = perlin_map[x as usize][y as usize];
            let tile = if val > 0.7 {
                Tiles::Rock
            } else if val > 0.65 {
                Tiles::Grass
            } else {
                Tiles::Field
            };

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

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

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
}
