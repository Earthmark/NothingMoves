use bevy::prelude::*;

use super::maze_level::*;
use super::maze_level::{Axis, Direction};

pub struct MazeInputBundle;

impl Plugin for MazeInputBundle {
    fn build(&self, app: &mut App) {
        app.add_event::<AxisChanged>()
            .add_event::<PositionChanged>()
            .add_systems(OnEnter(crate::AppState::InMaze), initial_events_on_load)
            .add_systems(
                Update,
                level_navigation.run_if(in_state(crate::AppState::InMaze)),
            );
    }
}

#[derive(Clone, Debug, Event)]
pub struct AxisChanged {
    pub axis: [u8; 2],
    pub previous_axis: [u8; 2],
}

#[derive(Clone, Debug, Event)]
pub struct PositionChanged {
    pub position: [u8; 2],
    pub previous_position: [u8; 2],
}

fn initial_events_on_load(
    maze: Res<MazeLevel>,
    mut position_changed: EventWriter<PositionChanged>,
    mut axis_changed: EventWriter<AxisChanged>,
) {
    position_changed.write(PositionChanged {
        position: maze.pos(),
        previous_position: maze.pos(),
    });
    axis_changed.write(AxisChanged {
        axis: maze.axis(),
        previous_axis: maze.axis(),
    });
}

fn level_navigation(
    mut level: ResMut<MazeLevel>,
    keys: Res<ButtonInput<KeyCode>>,
    mut position_event: EventWriter<PositionChanged>,
    mut axis_event: EventWriter<AxisChanged>,
) {
    let mut shift_axis = |key: KeyCode, axis: Axis, dir: Direction| {
        if keys.just_pressed(key) {
            let previous_axis = level.axis();
            level.shift_axis(axis, dir);
            let axis = level.axis();
            if previous_axis != axis {
                axis_event.write(AxisChanged {
                    axis,
                    previous_axis,
                });
            }
        }
    };
    shift_axis(KeyCode::KeyQ, Axis::X, Direction::Negative);
    shift_axis(KeyCode::KeyE, Axis::X, Direction::Positive);
    shift_axis(KeyCode::KeyZ, Axis::Y, Direction::Negative);
    shift_axis(KeyCode::KeyX, Axis::Y, Direction::Positive);
    let mut shift_position = |key: KeyCode, axis: Axis, dir: Direction| {
        if keys.just_pressed(key) {
            let previous_position = level.pos();
            level.move_pos(axis, dir);
            let position = level.pos();
            if previous_position != position {
                position_event.write(PositionChanged {
                    position,
                    previous_position,
                });
            }
        }
    };
    shift_position(KeyCode::KeyW, Axis::X, Direction::Positive);
    shift_position(KeyCode::KeyS, Axis::X, Direction::Negative);
    shift_position(KeyCode::KeyD, Axis::Y, Direction::Positive);
    shift_position(KeyCode::KeyA, Axis::Y, Direction::Negative);
}
