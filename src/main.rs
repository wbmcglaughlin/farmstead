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
        .add_state::<ui::mode::SelectionMode>()
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
            ),
        )
        .add_systems(
            Update,
            entities::click::check_entities_selection
                .run_if(in_state(ui::mode::SelectionMode::Selection)),
        )
        .add_systems(
            Update,
            entities::click::check_tiles_selection
                .run_if(in_state(ui::mode::SelectionMode::Tiling)),
        )
        .run();
}
