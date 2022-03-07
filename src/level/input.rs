use bevy::prelude::{Input, KeyCode, Query, Res};

use super::maze_level::*;
use super::maze_level::{Axis, Direction};

pub fn level_navigation<const DIMS: usize>(
    mut query: Query<&mut MazeLevel<DIMS>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        for mut level in query.iter_mut() {
            level.shift_axis(Axis::X, Direction::Negative);
        }
    }
    if keys.just_pressed(KeyCode::E) {
        for mut level in query.iter_mut() {
            level.shift_axis(Axis::X, Direction::Positive);
        }
    }
    if keys.just_pressed(KeyCode::W) {
        for mut level in query.iter_mut() {
            level.move_pos(Axis::X, Direction::Positive);
        }
    }
    if keys.just_pressed(KeyCode::S) {
        for mut level in query.iter_mut() {
            level.move_pos(Axis::X, Direction::Negative);
        }
    }
    if keys.just_pressed(KeyCode::D) {
        for mut level in query.iter_mut() {
            level.move_pos(Axis::Y, Direction::Positive);
        }
    }
    if keys.just_pressed(KeyCode::A) {
        for mut level in query.iter_mut() {
            level.move_pos(Axis::Y, Direction::Negative);
        }
    }
}
