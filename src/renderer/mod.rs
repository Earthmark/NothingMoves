mod hud;
mod maze_view;
mod tween;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((hud::plugin, maze_view::plugin, tween::plugin));
}
