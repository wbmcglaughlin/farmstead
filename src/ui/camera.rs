use bevy::prelude::*;

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
    tilemap: Query<&GlobalTransform, (With<MainTileMap>, Without<Camera>)>,
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

        let screen_coords = camera
            .world_to_ndc(global, tilemap.single().translation())
            .unwrap();
        dbg!(screen_coords);
        if screen_coords.x > 1.0 || screen_coords.x < -1.0 {
            transform.translation -= time.delta_seconds() * direction * 500.;
            transform.translation.z = z;
            ortho.scale = old_ortho_scale;
        }
    }
}
