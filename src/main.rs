use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
mod entities;
mod jobs;
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
        .add_systems(PostStartup, jobs::job::generate_job_queue)
        .add_systems(
            Update,
            (
                ui::camera::movement,
                ui::mode::switch_mode,
                ui::selection::adjust_rect_visibility_and_size,
                entities::player::spawn_player,
                entities::player::move_to_target,
                entities::player::player_movement,
                entities::player::search_for_job,
                entities::click::click_drag_handler,
                entities::player::execute_job,
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
