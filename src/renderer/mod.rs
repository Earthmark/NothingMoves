mod despawn_after;
mod hud;
mod maze_view;
mod tween;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        despawn_after::plugin,
        hud::plugin,
        maze_view::plugin,
        tween::plugin,
    ));
}
