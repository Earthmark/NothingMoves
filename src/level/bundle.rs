use super::*;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct LevelPluginBundle;

impl PluginGroup for LevelPluginBundle {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(maze_ui_renderer::MazeUiRendererPlugin)
            .add(maze_renderer::MazeRendererPlugin)
            .add(input::MazeInputBundle)
            .add(loader::MazeLoaderPlugin)
    }
}
