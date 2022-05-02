use super::*;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct LevelPluginBundle;

impl PluginGroup for LevelPluginBundle {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(maze_ui_renderer::MazeUiRendererPlugin)
            .add(maze_renderer::MazeRendererPlugin)
            .add(input::MazeInputBundle)
            .add(loader::MazeLoaderPlugin);
    }
}
