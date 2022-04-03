use bevy::prelude::*;

use super::maze_level::*;
use super::maze_level::{Axis, Direction};

pub fn level_navigation<const DIMS: usize>(
    mut query: Query<(Entity, &mut MazeLevel<DIMS>)>,
    keys: Res<Input<KeyCode>>,
    mut position_event: EventWriter<PositionChanged<DIMS>>,
    mut axis_event: EventWriter<AxisChanged>,
) {
    if keys.just_pressed(KeyCode::Q) {
        for (entity, mut level) in query.iter_mut() {
            level.shift_axis(Axis::X, Direction::Negative);
            axis_event.send(AxisChanged {
                level: entity,
                axis: level.axis,
            });
        }
    }
    if keys.just_pressed(KeyCode::E) {
        for (entity, mut level) in query.iter_mut() {
            level.shift_axis(Axis::X, Direction::Positive);
            axis_event.send(AxisChanged {
                level: entity,
                axis: level.axis,
            });
        }
    }
    if keys.just_pressed(KeyCode::W) {
        for (entity, mut level) in query.iter_mut() {
            level.move_pos(Axis::X, Direction::Positive);
            position_event.send(PositionChanged::<DIMS> {
                level: entity,
                position: level.position,
            });
        }
    }
    if keys.just_pressed(KeyCode::S) {
        for (entity, mut level) in query.iter_mut() {
            level.move_pos(Axis::X, Direction::Negative);
            position_event.send(PositionChanged::<DIMS> {
                level: entity,
                position: level.position,
            });
        }
    }
    if keys.just_pressed(KeyCode::D) {
        for (entity, mut level) in query.iter_mut() {
            level.move_pos(Axis::Y, Direction::Positive);
            position_event.send(PositionChanged::<DIMS> {
                level: entity,
                position: level.position,
            });
        }
    }
    if keys.just_pressed(KeyCode::A) {
        for (entity, mut level) in query.iter_mut() {
            level.move_pos(Axis::Y, Direction::Negative);
            position_event.send(PositionChanged::<DIMS> {
                level: entity,
                position: level.position,
            });
        }
    }
}
