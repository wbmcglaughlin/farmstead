use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::map::tilemap::tile_pos_to_transfrom;

#[derive(Debug, Clone, Copy)]
pub enum PlantType {
    Wheat,
    Carrot,
}

impl PlantType {
    pub fn png_file(&self) -> String {
        let filename = match self {
            PlantType::Wheat => "wheat.png",
            PlantType::Carrot => "carrot.png",
        };

        String::from("sprites/plants/") + filename
    }
}

#[derive(Component)]
pub struct Plant {
    pub ptype: PlantType,
    pub tile_pos: TilePos,
    pub stage_progress: Timer,
    pub planted: bool,
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
        if !plant.planted {
            continue;
        }
        plant.stage_progress.tick(time.delta());
        if plant.stage_progress.just_finished() {
            sprite.index = if sprite.index == indices.last {
                continue;
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn plant_bundle(
    texture_atlas: Handle<TextureAtlas>,
    growth_stage: usize,
    tile_pos: TilePos,
    tilemap_translation: Vec3,
) -> SpriteSheetBundle {
    let mut offset = tile_pos_to_transfrom(tile_pos, tilemap_translation);
    offset.z = 3.0;

    SpriteSheetBundle {
        texture_atlas,
        sprite: TextureAtlasSprite::new(growth_stage),
        transform: Transform::from_scale(Vec3::splat(1.0)).with_translation(offset),
        ..default()
    }
}
