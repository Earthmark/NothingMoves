use super::level::*;
use super::level::{Axis, Direction};
use crate::screens::maze::GameScreen;
use crate::screens::AppState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_message::<AxisChanged>()
        .add_systems(OnEnter(AppState::InMaze), initial_events_on_load)
        .add_systems(
            Update,
            (level_navigation,).run_if(in_state(GameScreen::Level)),
        );
}

#[derive(Clone, Debug, Message)]
pub struct AxisChanged {
    pub axis: [usize; 2],
    pub previous_axis: [usize; 2],
}

fn initial_events_on_load(maze: Res<MazeLevel>, mut axis_changed: MessageWriter<AxisChanged>) {
    axis_changed.write(AxisChanged {
        axis: maze.axis(),
        previous_axis: maze.axis(),
    });
}

fn level_navigation(
    mut level: ResMut<MazeLevel>,
    keys: Res<ButtonInput<KeyCode>>,
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
            level.move_pos(axis, dir);
        }
    };
    shift_position(KeyCode::KeyW, Axis::X, Direction::Positive);
    shift_position(KeyCode::KeyS, Axis::X, Direction::Negative);
    shift_position(KeyCode::KeyD, Axis::Y, Direction::Positive);
    shift_position(KeyCode::KeyA, Axis::Y, Direction::Negative);
}
