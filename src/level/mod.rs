mod input;
mod loader;
mod maze_level;
mod maze_renderer;
mod plugin;

use maze_level::MazeLevel;

pub use loader::{DimensionLength, LoadLevel};
pub use plugin::LevelPlugin;
