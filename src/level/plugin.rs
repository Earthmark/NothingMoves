use super::{input, loader, maze_renderer};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(loader::level_load_system)
            .add_event::<loader::LoadLevel>()
            .add_system(maze_renderer::maze_level_renderer::<2>)
            .add_system(maze_renderer::maze_level_renderer::<3>)
            .add_system(maze_renderer::maze_level_renderer::<4>)
            .add_system(maze_renderer::maze_level_renderer::<5>)
            .add_system(maze_renderer::maze_level_renderer::<6>)
            .add_system(input::level_navigation::<2>)
            .add_system(input::level_navigation::<3>)
            .add_system(input::level_navigation::<4>)
            .add_system(input::level_navigation::<5>)
            .add_system(input::level_navigation::<6>);
    }
}
