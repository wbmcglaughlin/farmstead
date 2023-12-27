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
    tilemap: Query<
        (&GlobalTransform, &TilemapSize, &TilemapTileSize),
        (With<MainTileMap>, Without<Camera>),
    >,
) {
    for (global, mut transform, mut ortho, camera) in query.iter_mut() {
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

        let mut ortho_scale = ortho.scale;
        let old_ortho_scale = ortho.scale;

        if keyboard_input.pressed(KeyCode::Z) {
            ortho_scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho_scale -= 0.1;
        }

        if ortho_scale < 0.5 {
            ortho_scale = 0.5;
        }

        // Apply ortho scale.
        ortho.scale = ortho_scale;

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;

        // We want the center of the screen to contain atleast a tile. To do this we need to find
        // if the center point is contained by all the four corners of the tilemap.
        let (tilemap_global_transform, mapsize, tilesize) = tilemap.single();
        let tilemap_translation = tilemap_global_transform.translation();
        let tilemapwidth = mapsize.x as f32 * tilesize.x;
        let tilemapheight = mapsize.y as f32 * tilesize.y;

        let window = q_window.single();
        let cursor_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);

        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let ray = camera.viewport_to_world_2d(global, cursor_pos).unwrap();

        if ray.y > tilemapheight || ray.y < 0.0 || ray.x > tilemapwidth || ray.x < 0.0 {
            transform.translation -= time.delta_seconds() * direction * 500.;
            transform.translation.z = z;
            ortho.scale = old_ortho_scale;
        }
    }
}
