use super::level::*;
use super::level::{Axis, Direction};
use crate::screens::AppState;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_message::<AxisChanged>()
        .add_message::<PositionChanged>()
        .add_systems(OnEnter(AppState::InMaze), initial_events_on_load)
        .add_systems(
            Update,
            (
                level_navigation,
                back_to_menu.run_if(input_just_pressed(KeyCode::Escape)),
            )
                .run_if(in_state(AppState::InMaze)),
        );
}

fn back_to_menu(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::MainMenu);
}

#[derive(Clone, Debug, Message)]
pub struct AxisChanged {
    pub axis: [u8; 2],
    pub previous_axis: [u8; 2],
}

#[derive(Clone, Debug, Message)]
pub struct PositionChanged {
    pub _position: [u8; 2],
    pub _previous_position: [u8; 2],
}

fn initial_events_on_load(
    maze: Res<MazeLevel>,
    mut position_changed: MessageWriter<PositionChanged>,
    mut axis_changed: MessageWriter<AxisChanged>,
) {
    position_changed.write(PositionChanged {
        _position: maze.pos(),
        _previous_position: maze.pos(),
    });
    axis_changed.write(AxisChanged {
        axis: maze.axis(),
        previous_axis: maze.axis(),
    });
}

fn level_navigation(
    mut level: ResMut<MazeLevel>,
    keys: Res<ButtonInput<KeyCode>>,
    mut position_event: MessageWriter<PositionChanged>,
    mut axis_event: MessageWriter<AxisChanged>,
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
                    _position: position,
                    _previous_position: previous_position,
                });
            }
        }
    };
    shift_position(KeyCode::KeyW, Axis::X, Direction::Positive);
    shift_position(KeyCode::KeyS, Axis::X, Direction::Negative);
    shift_position(KeyCode::KeyD, Axis::Y, Direction::Positive);
    shift_position(KeyCode::KeyA, Axis::Y, Direction::Negative);
}
