use crate::assets::CommonAssets;
use crate::game::level::{Direction, MazeLevel};
use crate::screens::AppState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InMaze), spawn_ui)
        .add_systems(
            Update,
            (
                maze_axis_label_update_listener,
                maze_position_label_update_listener,
                maze_axis_label_background_updater,
            )
                .run_if(in_state(AppState::InMaze).and_then(resource_changed::<MazeLevel>)),
        );
}

// Current dimension status text layout:
// Show inactive as greyed out.

// Primary
//                     |  |
//   W     D     +  +
// [ 0, 1, 0, 0, 0, 0 ] -> [ 6, 6, 6, 6, 6, 6 ]
//   S     A     +
//
// Secondary
//  |  |  |  |  |  | |
//   W     D     +  +
// [ 0, 1, 0, 0, 0, 0 ]
//   S     A     +
//
// Z < W/S > X | Q < D/A > E

// Movement cell states
// dimension, direction
// can_move <- Level.can_move_in(dimension, direction)
// key_text <- action_bindings.key_for(position.maybe_bound_axis(dimension))

// match (can_move, key_text) {
//   (true, Some(key)) -> White Key Square,
//   (false, Some(key)) -> Greyed out key square,
//   (true, None) -> White Circle,
//   (false, None) -> Greyed out circle,
// }

const TEXT_COLOR: Srgba = bevy::color::palettes::css::BLACK;
const INACTIVE_COLOR: Srgba = bevy::color::palettes::css::GREY;
const ACTIVE_COLOR: Color = Color::WHITE;

fn spaced_column() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceEvenly,
        margin: UiRect {
            left: Val::Px(3.0),
            right: Val::Px(3.0),
            ..default()
        },
        ..default()
    }
}

fn spawn_ui(mut c: Commands, maze: Res<MazeLevel>, common_assets: Res<CommonAssets>) {
    let label = |s: &str, c: Color| {
        (
            Text::new(s),
            common_assets.common_text_style(),
            TextColor(c),
        )
    };

    c.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        DespawnOnExit(AppState::InMaze),
    ))
    .with_children(|c| {
        c.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..default()
        })
        .with_children(|c| {
            c.spawn((
                spaced_column(),
                children![
                    label("Q", Color::WHITE),
                    label("[", Color::WHITE),
                    label("Z", Color::WHITE)
                ],
            ));
            for (dim, _) in maze.sides().iter().enumerate() {
                c.spawn((
                    spaced_column(),
                    children![
                        (
                            label("-", TEXT_COLOR.into()),
                            MazeAxisLabel {
                                dim,
                                dir: Direction::Positive,
                            },
                        ),
                        (label("#", Color::WHITE), MazePositionLabel(dim)),
                        (
                            label("-", TEXT_COLOR.into()),
                            MazeAxisLabel {
                                dim,
                                dir: Direction::Negative,
                            },
                        )
                    ],
                ));
            }
            c.spawn((
                spaced_column(),
                children![
                    label("E", Color::WHITE),
                    label("]", Color::WHITE),
                    label("X", Color::WHITE)
                ],
            ));
        });
    });
}

#[derive(Component, Clone)]
struct MazeAxisLabel {
    dim: usize,
    dir: Direction,
}

fn maze_axis_label_background_updater(
    level: Res<MazeLevel>,
    mut query: Query<(&MazeAxisLabel, &mut BackgroundColor)>,
) {
    for (axis, mut ui_color) in query.iter_mut() {
        ui_color.0 = if let Some(true) = level.can_move(axis.dim, axis.dir) {
            ACTIVE_COLOR
        } else {
            INACTIVE_COLOR.into()
        };
    }
}

fn maze_axis_label_update_listener(
    mut query: Query<(&MazeAxisLabel, &mut Text)>,
    level: Res<MazeLevel>,
) {
    let [ax, ay] = level.axis();
    for (label, mut text) in query.iter_mut() {
        text.set_if_neq(Text::new(if ax == label.dim {
            match label.dir {
                Direction::Positive => "W",
                Direction::Negative => "S",
            }
        } else if ay == label.dim {
            match label.dir {
                Direction::Positive => "D",
                Direction::Negative => "A",
            }
        } else {
            "-"
        }));
    }
}

#[derive(Component)]
struct MazePositionLabel(usize);

fn maze_position_label_update_listener(
    maze: Res<MazeLevel>,
    mut query: Query<(&MazePositionLabel, &mut Text, &mut TextColor)>,
) {
    for (label, mut text, mut color) in query.iter_mut() {
        if let Some(target) = maze.pos().get(label.0) {
            let position = target + 1;
            text.set_if_neq(Text::new(format!("{}", position)));

            color.0 = if maze.sides().get(label.0) == Some(&position) {
                bevy::color::palettes::css::LIMEGREEN.into()
            } else {
                Color::WHITE
            };
        }
    }
}
