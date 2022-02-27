use bevy::prelude::*;

use super::*;

pub fn level_navigation<const DIMS: usize>(
    mut query: Query<&mut super::MazeLevel<DIMS>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        for mut level in query.iter_mut() {
            level.off_axis_shift(maze_level::Direction::Negative);
        }
    }
    if keys.just_pressed(KeyCode::E) {
        for mut level in query.iter_mut() {
            level.off_axis_shift(maze_level::Direction::Positive);
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        for mut level in query.iter_mut() {
            level.flip_axis();
        }
    }
}