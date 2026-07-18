use std::f32::consts::PI;

use crate::assets::CommonAssets;
use crate::assets::MazeUiAssets;
use crate::game::level::{Direction, MazeLevel};
use crate::game::{AxisChanged, PositionChanged};
use crate::screens::AppState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::InMaze),
        (spawn_ui, create_rotation_binder),
    )
    .add_systems(
        Update,
        (
            maze_axis_label_update_listener,
            maze_position_label_update_listener,
            maze_axis_label_background_updater,
            update_guide_arrows,
        )
            .run_if(in_state(AppState::InMaze)),
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

#[derive(Component)]
struct DimensionArrowUpdater {
    flipped: bool,
    enabled: bool,
}

fn update_guide_arrows(
    ui_assets: Res<MazeUiAssets>,
    mut axis_changed: MessageReader<AxisChanged>,
    mut query: Query<(&DimensionArrowUpdater, &mut ImageNode)>,
) {
    for _ in axis_changed.read() {
        for (dim, mut img) in query.iter_mut() {
            img.image = match (dim.enabled, dim.flipped) {
                (true, true) => &ui_assets.rotate_arrow_flip,
                (true, false) => &ui_assets.rotate_arrow,
                (false, true) => &ui_assets.rotate_arrow_flip_inactive,
                (false, false) => &ui_assets.rotate_arrow_inactive,
            }
            .clone();
        }
    }
}

fn create_rotation_binder(
    mut c: Commands,
    common_assets: Res<CommonAssets>,
    ui_assets: Res<MazeUiAssets>,
) {
    c.spawn((
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        DespawnOnExit(AppState::InMaze),
    ))
    .with_children(|c| {
        c.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|c| {
            c.spawn((
                Text2d::new("Q"),
                common_assets.common_text_style(),
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ));
            c.spawn((
                ImageNode::new(ui_assets.rotate_arrow.clone()),
                DimensionArrowUpdater {
                    flipped: false,
                    enabled: true,
                },
            ));
        });

        c.spawn(Node {
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|c| {
            c.spawn((Text2d::new("A"), common_assets.common_text_style()));
            c.spawn(ImageNode::new(ui_assets.move_arrow.clone()));
            c.spawn((
                ImageNode::new(ui_assets.move_arrow_inactive.clone()),
                Transform::from_rotation(Quat::from_rotation_z(PI)),
            ));
            c.spawn((Text2d::new("D"), common_assets.common_text_style()));
        });

        c.spawn(Node {
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|c| {
            c.spawn((
                Text2d::new("E"),
                common_assets.common_text_style(),
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ));
            c.spawn((
                Transform::from_rotation(Quat::from_rotation_z(PI)),
                ImageNode::new(ui_assets.rotate_arrow_flip_inactive.clone()),
                Node::default(),
                DimensionArrowUpdater {
                    flipped: true,
                    enabled: true,
                },
            ));
        });
    });
}

fn spawn_ui(mut c: Commands, maze: Res<MazeLevel>, common_assets: Res<CommonAssets>) {
    let label = |s: &str, c: Color| {
        (
            Text2d::new(s),
            common_assets.common_text_style(),
            TextColor(c),
        )
    };

    let dimension_col = |dimension: usize| {
        move |c: &mut ChildSpawnerCommands| {
            c.spawn((
                Node::default(),
                MazeAxisLabel {
                    dim: dimension as u8,
                    dir: Direction::Negative,
                },
            ))
            .with_children(|c| {
                c.spawn((
                    label("-", bevy::color::palettes::css::DARK_GRAY.into()),
                    MazeAxisLabel {
                        dim: dimension as u8,
                        dir: Direction::Negative,
                    },
                ));
            });
            c.spawn((label("#", Color::WHITE), MazePositionLabel { dimension }));

            c.spawn((
                Node::default(),
                MazeAxisLabel {
                    dim: dimension as u8,
                    dir: Direction::Positive,
                },
            ))
            .with_children(|c| {
                c.spawn((
                    label("-", bevy::color::palettes::css::DARK_GRAY.into()),
                    MazeAxisLabel {
                        dim: dimension as u8,
                        dir: Direction::Positive,
                    },
                ));
            });
        }
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
            c.spawn(label("[", Color::WHITE));
            for (i, _) in maze.dims_limit().iter().enumerate() {
                c.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    margin: UiRect {
                        left: Val::Px(3.0),
                        right: Val::Px(3.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(dimension_col(i));
            }
            c.spawn(label("]", Color::WHITE));
        });
    });
}

#[derive(Component, Clone)]
struct MazeAxisLabel {
    dim: u8,
    dir: Direction,
}

fn maze_axis_label_background_updater(
    level: Res<MazeLevel>,
    mut query: Query<(&MazeAxisLabel, &mut BackgroundColor)>,
    mut axis_changed: MessageReader<AxisChanged>,
    mut position_changed: MessageReader<PositionChanged>,
) {
    let mut update_bg = || {
        for (axis, mut ui_color) in query.iter_mut() {
            ui_color.0 = if let Some(true) = level.can_move(axis.dim, axis.dir) {
                Color::WHITE
            } else {
                bevy::color::palettes::css::GRAY.into()
            };
        }
    };
    for _ in position_changed.read() {
        update_bg();
    }
    for _ in axis_changed.read() {
        update_bg();
    }
}

fn maze_axis_label_update_listener(
    mut query: Query<(&MazeAxisLabel, &mut Text2d)>,
    mut axis_changed: MessageReader<AxisChanged>,
) {
    for changed in axis_changed.read() {
        for (label, mut text) in query.iter_mut() {
            *text = Text2d::new(if changed.axis[0] == label.dim {
                match label.dir {
                    Direction::Positive => "W",
                    Direction::Negative => "S",
                }
            } else if changed.axis[1] == label.dim {
                match label.dir {
                    Direction::Positive => "D",
                    Direction::Negative => "A",
                }
            } else {
                ""
            });
        }
    }
}

#[derive(Component)]
struct MazePositionLabel {
    dimension: usize,
}

fn maze_position_label_update_listener(
    maze: Res<MazeLevel>,
    mut query: Query<(&MazePositionLabel, &mut Text2d, &mut TextColor)>,
    mut position_changed: MessageReader<PositionChanged>,
) {
    for _ in position_changed.read() {
        for (label, mut text, mut color) in query.iter_mut() {
            if let Some(target) = maze.dims().get(label.dimension) {
                let position = target + 1;
                *text = Text2d::new(format!("{}", position));

                color.0 = if maze.dims_limit().get(label.dimension) == Some(&position) {
                    bevy::color::palettes::css::LIMEGREEN.into()
                } else {
                    Color::WHITE
                };
            }
        }
    }
}
