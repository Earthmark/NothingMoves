use super::maze_level::*;
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
        ..Default::default()
    };

    let label = |s: &str| TextBundle {
        text: Text::with_section(
            s,
            style.clone(),
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        style: Style {
            size: Size::new(Val::Auto, Val::Px(50.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    fn column() -> NodeBundle {
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        }
    }

    let dimension_col = |dimension: usize| {
        move |c: &mut ChildBuilder| {
            c.spawn_bundle(label("-")).insert(MazeAxisLabel {
                dim: dimension as u8,
                dir: AxisDirection::Positive,
            });
            c.spawn_bundle(label("#"))
                .insert(MazePositionLabel { dimension });
            c.spawn_bundle(label("-")).insert(MazeAxisLabel {
                dim: dimension as u8,
                dir: AxisDirection::Negative,
            });
        }
    };

    c.spawn_bundle(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    })
    .with_children(|c| {
        c.spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
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
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
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
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        });

        c.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|c| {
            c.spawn_bundle(label("["));
            for (i, _) in maze.dims_limit().iter().enumerate() {
                c.spawn_bundle(column()).with_children(dimension_col(i));
            }
            c.spawn_bundle(label("]"));
        });
    });
}

#[derive(Component)]
struct MazeUiRoot;

enum AxisDirection {
    Positive,
    Negative,
}

#[derive(Component)]
pub struct MazeAxisLabel {
    dim: u8,
    dir: AxisDirection,
}

pub fn maze_axis_label_update_listener(
    mut query: Query<(&MazeAxisLabel, &mut Text)>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    for changed in axis_changed.iter() {
        for (label, mut text) in query.iter_mut() {
            if changed.axis[0] == label.dim {
                text.sections[0].value = match label.dir {
                    AxisDirection::Positive => "S".into(),
                    AxisDirection::Negative => "W".into(),
                };
            } else if changed.axis[1] == label.dim {
                text.sections[0].value = match label.dir {
                    AxisDirection::Positive => "A".into(),
                    AxisDirection::Negative => "D".into(),
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
