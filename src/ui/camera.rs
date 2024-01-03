// Import necessary components from the Bevy engine and tilemap module.
use bevy::prelude::*;
use bevy_ecs_tilemap::map::{TilemapSize, TilemapTileSize};

// Reference to the MainTileMap module.
use crate::map::tilemap::MainTileMap;

// Function to add a 2D camera to the game.
pub fn add_camera(mut commands: Commands) {
    // Spawn a new camera using the default 2D camera settings.
    commands.spawn(Camera2dBundle::default());
}

// Function to handle camera movement.
pub fn movement(
    // Resources and components needed for the function.
    time: Res<Time>, // Access to the game's time information.
    keyboard_input: Res<Input<KeyCode>>, // Access to keyboard input.
    // Query to get camera transform and projection data.
    mut query: Query<(
        &GlobalTransform,
        &mut Transform,
        &mut OrthographicProjection,
        &Camera,
    )>,
    q_window: Query<&Window>, // Query to access window information.
    // Query to get tilemap size and tile size, excluding the camera.
    tilemap: Query<(&TilemapSize, &TilemapTileSize), (With<MainTileMap>, Without<Camera>)>,
) {
    // Iterate over all entities that match the query (usually just the camera).
    for (global, mut transform, mut ortho, camera) in query.iter_mut() {
        // Determine movement direction based on keyboard inputs.
        let direction = direction_from_keys(&keyboard_input);

        // Zoom in/out functionality based on Z and X keys.
        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }
        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }
        // Ensure the zoom scale doesn't go below a certain threshold.
        if ortho.scale < 0.2 {
            ortho.scale = 0.2;
        }

        // Store the current Z coordinate of the camera (to maintain layer ordering).
        let z = transform.translation.z;
        // Calculate the new position based on direction and time.
        let mut translation = time.delta_seconds() * direction * 500.;

        // Get the size of the tilemap to calculate boundaries.
        let (mapsize, tilesize) = tilemap.single();
        let tilemapwidth = mapsize.x as f32 * tilesize.x;
        let tilemapheight = mapsize.y as f32 * tilesize.y;

        // Get the center position of the window.
        let window = q_window.single();
        let center_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);

        // Calculate a ray from the camera to the world based on the center position.
        let ray = camera.viewport_to_world_2d(global, center_pos).unwrap();

        // Adjust the translation to ensure the camera stays within the tilemap boundaries.
        lock_translation(ray, tilemapwidth, tilemapheight, &mut translation);

        // Apply the calculated translation to the camera.
        transform.translation += translation;
        // Restore the Z coordinate to maintain layer ordering.
        transform.translation.z = z;
    }
}

// Function to calculate the direction of movement based on keyboard inputs.
fn direction_from_keys(keyboard_input: &Res<'_, Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    // Adjust the direction based on WASD key presses.
    if keyboard_input.pressed(KeyCode::A) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }
    direction
}

// Function to constrain the camera's movement within the bounds of the tilemap.
fn lock_translation(ray: Vec2, tilemapwidth: f32, tilemapheight: f32, translation: &mut Vec3) {
    // Check and adjust the translation based on the tilemap boundaries.
    if ray.x < -tilemapwidth / 2.0 {
        translation.x = 0_f32.max(translation.x);
    }
    if ray.x > tilemapwidth / 2.0 {
        translation.x = 0_f32.min(translation.x);
    }
    if ray.y < -tilemapheight / 2.0 {
        translation.y = 0_f32.max(translation.y);
    }
    if ray.y > tilemapheight / 2.0 {
        translation.y = 0_f32.min(translation.y);
    }
}
