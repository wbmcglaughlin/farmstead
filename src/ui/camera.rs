use bevy::prelude::*;
use bevy_ecs_tilemap::map::{TilemapSize, TilemapTileSize};

use crate::map::tilemap::MainTileMap;

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &GlobalTransform,
        &mut Transform,
        &mut OrthographicProjection,
        &Camera,
    )>,
    q_window: Query<&Window>,
    tilemap: Query<(&TilemapSize, &TilemapTileSize), (With<MainTileMap>, Without<Camera>)>,
) {
    for (global, mut transform, mut ortho, camera) in query.iter_mut() {
        let direction = direction_from_keys(&keyboard_input);

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.2 {
            ortho.scale = 0.2;
        }

        let z = transform.translation.z;
        let mut translation = time.delta_seconds() * direction * 500.;

        // We want the center of the screen to contain atleast a tile. To do this we need to find
        // if the center point is contained by all the four corners of the tilemap.
        let (mapsize, tilesize) = tilemap.single();
        let tilemapwidth = mapsize.x as f32 * tilesize.x;
        let tilemapheight = mapsize.y as f32 * tilesize.y;

        let window = q_window.single();
        let center_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);

        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let ray = camera.viewport_to_world_2d(global, center_pos).unwrap();

        // Clamp the direction of travel to one that would be restoring. Its better to do it this
        // way as there is no "snapping".
        lock_translation(ray, tilemapwidth, tilemapheight, &mut translation);

        transform.translation += translation;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

fn direction_from_keys(keyboard_input: &Res<'_, Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;

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

fn lock_translation(ray: Vec2, tilemapwidth: f32, tilemapheight: f32, translation: &mut Vec3) {
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
