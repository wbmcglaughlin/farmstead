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
        .insert_resource(ui::mode::SelectionMode::Selection)
        .add_systems(
            Startup,
            (
                ui::camera::add_camera,
                map::tilemap::generate_map,
                ui::selection::create_rect_sprite,
            ),
        )
        .add_systems(
            Update,
            (
                ui::mode::switch_mode,
                entities::player::spawn_player,
                ui::camera::movement,
                entities::player::player_movement,
                ui::selection::adjust_rect_visibility_and_size,
                entities::player::move_to_target,
                entities::click::click_drag_handler,
                entities::click::check_click_selection,
            ),
        )
        .run();
}
