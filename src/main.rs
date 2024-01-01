use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
mod entities;
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
        .add_systems(Startup, ui::camera::add_camera)
        .add_systems(Startup, map::tilemap::generate_map)
        .add_systems(Startup, entities::player::spawn_player)
        .add_systems(Update, ui::camera::movement)
        .add_systems(Update, entities::player::player_movement)
        .add_systems(Update, entities::player::move_to_target)
        .add_systems(Update, entities::click::check_click_selection)
        .run();
}
