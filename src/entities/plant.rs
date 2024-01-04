use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component)]
pub struct Plant {
    pub tile_pos: TilePos,
    pub stage_progress: Timer,
}

#[derive(Component)]
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
) {
    let texture_handle = asset_server.load("wheat.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let growth_stage = GrowthStage { first: 0, last: 4 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(growth_stage.first),
            transform: Transform::from_scale(Vec3::splat(1.0))
                .with_translation(Vec3::new(0.0, 0.0, 3.0)),
            ..default()
        },
        Plant {
            tile_pos: TilePos { x: 0, y: 0 },
            stage_progress: Timer::from_seconds(5.0, TimerMode::Repeating),
        },
        growth_stage,
    ));
}
