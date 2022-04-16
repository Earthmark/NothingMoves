use super::maze_level::{self, *};
use bevy::prelude::*;

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

pub fn spawn_ui(mut c: Commands, maze: Res<MazeLevel>, assets: Res<AssetServer>) {
    let style = TextStyle {
        font: assets.load("fonts\\UnicaOne-Regular.ttf"),
        font_size: 50.0,
        ..default()
    };

    let label = |s: &str, c: Color| TextBundle {
        text: Text::with_section(
            s,
            TextStyle {
                color: c,
                ..style.clone()
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            size: Size::new(Val::Auto, Val::Px(50.0)),
            ..default()
        },
        ..default()
    };

    let dimension_col = |dimension: usize| {
        move |c: &mut ChildBuilder| {
            c.spawn_bundle(NodeBundle::default())
                .with_children(|c| {
                    c.spawn_bundle(label("-", Color::DARK_GRAY))
                        .insert(MazeAxisLabel {
                            dim: dimension as u8,
                            dir: maze_level::Direction::Negative,
                        });
                })
                .insert(MazeAxisLabel {
                    dim: dimension as u8,
                    dir: maze_level::Direction::Negative,
                });
            c.spawn_bundle(label("#", Color::WHITE))
                .insert(MazePositionLabel { dimension });

            c.spawn_bundle(NodeBundle::default())
                .with_children(|c| {
                    c.spawn_bundle(label("-", Color::DARK_GRAY))
                        .insert(MazeAxisLabel {
                            dim: dimension as u8,
                            dir: maze_level::Direction::Positive,
                        });
                })
                .insert(MazeAxisLabel {
                    dim: dimension as u8,
                    dir: maze_level::Direction::Positive,
                });
        }
    };

    c.spawn_bundle(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    })
    .with_children(|c| {
        c.spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|c| {
            // Axis Shift Controls
            c.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Z<W/S>X",
                    style.clone(),
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                style: Style {
                    margin: Rect {
                        right: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
            c.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Q<D/A>E",
                    style.clone(),
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                style: Style {
                    margin: Rect {
                        left: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
        });

        c.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|c| {
            c.spawn_bundle(label("[", Color::WHITE));
            for (i, _) in maze.dims_limit().iter().enumerate() {
                c.spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        margin: Rect {
                            left: Val::Px(3.0),
                            right: Val::Px(3.0),
                            ..default()
                        },
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(dimension_col(i));
            }
            c.spawn_bundle(label("]", Color::WHITE));
        });
    });
}

#[derive(Component)]
struct MazeUiRoot;

#[derive(Component, Clone)]
pub struct MazeAxisLabel {
    dim: u8,
    dir: maze_level::Direction,
}

pub fn maze_axis_label_background_updater(
    level: Res<MazeLevel>,
    mut query: Query<(&MazeAxisLabel, &mut UiColor)>,
    mut axis_changed: EventReader<AxisChanged>,
    mut position_changed: EventReader<PositionChanged>,
) {
    let mut update_bg = || {
        for (axis, mut ui_color) in query.iter_mut() {
            ui_color.0 = if let Some(true) = level.can_move(axis.dim, axis.dir) {
                Color::WHITE
            } else {
                Color::GRAY
            }
            .into();
        }
    };
    for _ in position_changed.iter() {
        update_bg();
    }
    for _ in axis_changed.iter() {
        update_bg();
    }
}

pub fn maze_axis_label_update_listener(
    mut query: Query<(&MazeAxisLabel, &mut Text)>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    for changed in axis_changed.iter() {
        for (label, mut text) in query.iter_mut() {
            if changed.axis[0] == label.dim {
                text.sections[0].value = match label.dir {
                    maze_level::Direction::Positive => "W".into(),
                    maze_level::Direction::Negative => "S".into(),
                };
            } else if changed.axis[1] == label.dim {
                text.sections[0].value = match label.dir {
                    maze_level::Direction::Positive => "D".into(),
                    maze_level::Direction::Negative => "A".into(),
                };
            } else {
                text.sections[0].value = "".into();
            }
        }
    }
}

#[derive(Component)]
pub struct MazePositionLabel {
    dimension: usize,
}

pub fn maze_position_label_update_listener(
    maze: Res<MazeLevel>,
    mut query: Query<(&MazePositionLabel, &mut Text)>,
    mut position_changed: EventReader<PositionChanged>,
) {
    for _ in position_changed.iter() {
        for (label, mut text) in query.iter_mut() {
            if let Some(section) = text.sections.first_mut() {
                if let Some(target) = maze.dims().get(label.dimension) {
                    section.value = format!("{}", target);
                }
            }
        }
    }
}
