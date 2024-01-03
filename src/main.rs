// Importing necessary modules and types from Bevy and the bevy_ecs_tilemap crate.
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// Including local modules that define specific functionality for entities, jobs, map, and UI in the game.
mod entities;
mod jobs;
mod map;
mod ui;

// Main function, which is the entry point of the Rust program.
fn main() {
    // Create a new Bevy application.
    App::new()
        // Add various plugins to the application, which extend its functionality.
        .add_plugins(
            DefaultPlugins
                // Customize the WindowPlugin settings for the primary window, like setting the window title.
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("farmstead."),
                        // Use default settings for other window parameters.
                        ..Default::default()
                    }),
                    // Use default settings for other plugin parameters.
                    ..default()
                })
                // Set the ImagePlugin to use nearest neighbor scaling, affecting image rendering.
                .set(ImagePlugin::default_nearest()),
        )
        // Add the TilemapPlugin to enable tilemap functionality in the game.
        .add_plugins(TilemapPlugin)
        // Add a game state to manage UI mode, specifically for selection modes.
        .add_state::<ui::mode::SelectionMode>()
        // Register multiple systems to execute at the game's startup.
        .add_systems(
            Startup,
            (
                ui::camera::add_camera,
                map::tilemap::generate_map,
                ui::selection::create_rect_sprite,
            ),
        )
        // Register a system to execute after startup, here to generate a job queue.
        .add_systems(PostStartup, jobs::job::generate_job_queue)
        // Register multiple systems to execute during the game's update cycle.
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
            ),
        )
        // Register a conditional system for entity selection, to be run only in a specific UI mode.
        .add_systems(
            Update,
            entities::click::check_entities_selection
                .run_if(in_state(ui::mode::SelectionMode::Selection)),
        )
        // Register a conditional system for tile selection, to be run only in another specific UI mode.
        .add_systems(
            Update,
            entities::click::check_tiles_selection
                .run_if(in_state(ui::mode::SelectionMode::Tiling)),
        )
        // Start the Bevy application, initiating the game loop.
        .run();
}
