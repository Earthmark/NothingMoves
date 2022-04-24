use super::*;
use crate::AppState;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
struct LevelInit;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(loader::load_maze_assets)
            .add_system(loader::level_load_system.before(LevelInit))
            .add_event::<loader::LoadLevel>()
            .add_event::<maze_level::AxisChanged>()
            .add_event::<maze_level::PositionChanged>()
            .add_system_set(
                SystemSet::on_enter(AppState::InMaze)
                    .label(LevelInit)
                    .with_system(maze_ui_renderer::spawn_ui)
                    .with_system(loader::initial_events_on_load)
                    .with_system(loader::spawn_player),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InMaze)
                    .after(LevelInit)
                    .with_system(maze_ui_renderer::maze_axis_label_update_listener)
                    .with_system(maze_ui_renderer::maze_position_label_update_listener)
                    .with_system(maze_ui_renderer::maze_axis_label_background_updater)
                    .with_system(maze_renderer::maze_level_renderer)
                    .with_system(maze_renderer::rotate_for_n_update)
                    .with_system(maze_renderer::remove_after_time)
                    .with_system(
                        maze_renderer::update_maze_offset.after(maze_renderer::maze_level_renderer),
                    )
                    .with_system(maze_renderer::start_despawn_of_render)
                    .with_system(input::level_navigation),
            );
    }
}
