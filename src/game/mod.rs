pub mod level;
mod movement;

use bevy::prelude::*;

pub use movement::{AxisChanged, PositionChanged};

pub fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, movement::plugin));
}
