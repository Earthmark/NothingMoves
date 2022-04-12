use super::*;
use crate::AppState;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(loader::level_load_system)
            .add_event::<loader::LoadLevel>()
            .add_event::<maze_level::AxisChanged>()
            .add_event::<maze_level::PositionChanged>()
            .add_system_set(
                SystemSet::on_enter(AppState::InMaze)
                    .with_system(maze_renderer::spawn_maze_root)
                    .with_system(maze_ui_renderer::spawn_ui)
                    .with_system(loader::initial_events_on_load),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InMaze)
                    .with_system(maze_ui_renderer::maze_axis_label_update_listener)
                    .with_system(maze_ui_renderer::maze_position_label_update_listener)
                    .with_system(maze_renderer::maze_level_renderer)
                    .with_system(maze_renderer::update_maze_offset)
                    .with_system(input::level_navigation),
            );
    }
}
