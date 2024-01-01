use bevy::prelude::*;

const PLAYER_SPEED: f32 = 30.0;

#[derive(Component)]
pub struct Player {
    pub selected: bool,
    pub target: Option<Vec2>,
}

#[derive(Component)]
pub struct Highlight;

impl Player {
    pub fn new() -> Self {
        Self {
            selected: false,
            target: None,
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("walk.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 2 };
    let player_transform = Transform::from_xyz(0.0, 0.0, 1.0);

    let hightlight = commands
        .spawn(SpriteBundle {
            texture: asset_server.load("highlight.png"),
            transform: player_transform,
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(Highlight)
        .id();

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: player_transform,
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Player::new(),
        ))
        .add_child(hightlight);
}

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn move_to_target(time: Res<Time>, mut player_entity: Query<(&mut Transform, &mut Player)>) {
    for (mut transform, mut player) in player_entity.iter_mut() {
        // Check if the player has a target position.
        if let Some(t) = player.target {
            // Find deltas.
            let dt = time.delta().as_secs_f32();
            let dx = t.x - transform.translation.x;
            let dy = t.y - transform.translation.y;

            // Get distance squared.
            let distance_squared = dx.powf(2.0) + dy.powf(2.0);

            // Angle to which player is going.
            // TODO: will need some sort of pathfinding algoritm.
            let angle = dy.atan2(dx);

            // Get player transform if they were to move at the optimal speed.
            let translation_x = PLAYER_SPEED * angle.cos() * dt;
            let translation_y = PLAYER_SPEED * angle.sin() * dt;
            let translation_distance_squared = translation_x.powf(2.0) + translation_y.powf(2.0);

            // If the distance to the target is less than the current step, set the position and
            // clear the target position.
            if distance_squared < translation_distance_squared {
                transform.translation.x = t.x;
                transform.translation.y = t.y;
                player.target = None;
            } else {
                transform.translation.x += translation_x;
                transform.translation.y += translation_y;
            }
        }
    }
}
