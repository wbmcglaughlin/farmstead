use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
mod map;
mod ui;

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
        .add_systems(Startup, map::tilemap::generate_map)
        .add_systems(Update, ui::camera::movement)
        .run();
}
