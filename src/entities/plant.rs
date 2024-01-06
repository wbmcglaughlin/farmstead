use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::map::tilemap::{tile_pos_to_transfrom, MainTileMap};

#[derive(Debug, Clone, Copy)]
pub enum PlantType {
    Wheat,
}

impl PlantType {
    pub fn png_file(&self) -> String {
        let filename = match self {
            PlantType::Wheat => "wheat.png",
        };

        String::from(filename)
    }
}

#[derive(Component)]
pub struct Plant {
    pub ptype: PlantType,
    pub tile_pos: TilePos,
    pub stage_progress: Timer,
}

#[derive(Component, Clone, Copy)]
pub struct GrowthStage {
    pub first: usize,
    pub last: usize,
}

pub fn animate_plant(
    time: Res<Time>,
    mut query: Query<(&mut Plant, &GrowthStage, &mut TextureAtlasSprite)>,
) {
    for (mut plant, indices, mut sprite) in &mut query {
        plant.stage_progress.tick(time.delta());
        if plant.stage_progress.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn plant(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    tilemap_query: Query<&Transform, With<MainTileMap>>,
) {
    let texture_handle = asset_server.load("wheat.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let tile_pos = TilePos { x: 0, y: 0 };
    let growth_stage = GrowthStage { first: 0, last: 4 };
    let tilemap_transform = tilemap_query.single();

    commands.spawn((
        plant_bundle(
            texture_atlas_handle,
            growth_stage,
            tile_pos,
            tilemap_transform.translation,
        ),
        Plant {
            ptype: PlantType::Wheat,
            tile_pos,
            stage_progress: Timer::from_seconds(20.0, TimerMode::Repeating),
        },
        growth_stage,
    ));
}

pub fn plant_bundle(
    texture_atlas: Handle<TextureAtlas>,
    growth_stage: GrowthStage,
    tile_pos: TilePos,
    tilemap_translation: Vec3,
) -> SpriteSheetBundle {
    let mut offset = tile_pos_to_transfrom(tile_pos, tilemap_translation);
    offset.z = 3.0;

    SpriteSheetBundle {
        texture_atlas,
        sprite: TextureAtlasSprite::new(growth_stage.first),
        transform: Transform::from_scale(Vec3::splat(1.0)).with_translation(offset),
        ..default()
    }
}
