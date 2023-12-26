use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
mod helpers;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::Rng;

// TODO: this needs to be implemented into a component to add to each tile.
#[derive(FromPrimitive)]
enum Tiles {
    Field,
    Grass,
    Farmland,
    Dirt,
    Stone,
    Rock,
}

fn random_enum_variant() -> (Option<Tiles>, u32) {
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(0..=5); // Adjust range based on your enum's values
    (Tiles::from_i32(random_int), random_int as u32)
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 128, y: 128 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let (tile_type, index) = random_enum_variant();
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    // Can just change this to get different tiles probably.
                    // The bundle should also contain a way to access what type of tile this is.
                    texture_index: TileTextureIndex(index),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("farmstead."),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Startup, startup)
        .add_systems(Update, helpers::camera::movement)
        .run();
}
