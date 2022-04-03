use super::*;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(loader::level_load_system)
            .add_event::<loader::LoadLevel>()
            .add_event::<maze_level::AxisChanged>()
            .add_plugin(SingleDimMazePlugin::<2>)
            .add_plugin(SingleDimMazePlugin::<3>)
            .add_plugin(SingleDimMazePlugin::<4>)
            .add_plugin(SingleDimMazePlugin::<5>)
            .add_plugin(SingleDimMazePlugin::<6>);
    }
}

pub struct SingleDimMazePlugin<const DIMS: usize>;

impl<const DIMS: usize> Plugin for SingleDimMazePlugin<DIMS> {
    fn build(&self, app: &mut App) {
        app.add_system(maze_renderer::maze_level_renderer::<DIMS>)
            .add_event::<maze_level::PositionChanged<DIMS>>()
            .add_system(maze_renderer::spawn_ui::<DIMS>)
            .add_system(maze_ui_renderer::spawn_ui::<DIMS>)
            .add_system(maze_ui_renderer::maze_axis_label_update_listener::<DIMS>)
            .add_system(maze_ui_renderer::maze_position_label_update_listener::<DIMS>)
            .add_system(loader::initial_events_on_load::<DIMS>)
            .add_system(maze_renderer::update_maze_offset::<DIMS>)
            .add_system(input::level_navigation::<DIMS>);
    }
}
