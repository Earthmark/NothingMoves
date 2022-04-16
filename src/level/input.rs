use bevy::prelude::*;

use super::maze_level::*;
use super::maze_level::{Axis, Direction};

pub fn level_navigation(
    level: Option<ResMut<MazeLevel>>,
    keys: Res<Input<KeyCode>>,
    mut position_event: EventWriter<PositionChanged>,
    mut axis_event: EventWriter<AxisChanged>,
) {
    if let Some(mut level) = level {
        if keys.just_pressed(KeyCode::Q) {
            level.shift_axis(Axis::X, Direction::Negative);
            axis_event.send(AxisChanged { axis: level.axis() });
        }
        if keys.just_pressed(KeyCode::E) {
            level.shift_axis(Axis::X, Direction::Positive);
            axis_event.send(AxisChanged { axis: level.axis() });
        }
        if keys.just_pressed(KeyCode::Z) {
            level.shift_axis(Axis::Y, Direction::Negative);
            axis_event.send(AxisChanged { axis: level.axis() });
        }
        if keys.just_pressed(KeyCode::X) {
            level.shift_axis(Axis::Y, Direction::Positive);
            axis_event.send(AxisChanged { axis: level.axis() });
        }
        if keys.just_pressed(KeyCode::W) {
            level.move_pos(Axis::X, Direction::Positive);
            position_event.send(PositionChanged {
                position: level.pos(),
            });
        }
        if keys.just_pressed(KeyCode::S) {
            level.move_pos(Axis::X, Direction::Negative);
            position_event.send(PositionChanged {
                position: level.pos(),
            });
        }
        if keys.just_pressed(KeyCode::D) {
            level.move_pos(Axis::Y, Direction::Positive);
            position_event.send(PositionChanged {
                position: level.pos(),
            });
        }
        if keys.just_pressed(KeyCode::A) {
            level.move_pos(Axis::Y, Direction::Negative);
            position_event.send(PositionChanged {
                position: level.pos(),
            });
        }
    }
}
