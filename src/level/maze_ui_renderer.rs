use super::maze_level::{Axis, *};
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

pub fn spawn_ui<const DIMS: usize>(
    mut c: Commands,
    query: Query<&MazeLevel<DIMS>, Added<MazeLevel<DIMS>>>,
    assets: Res<AssetServer>,
) {
    for _maze in query.iter() {
        info!("Added maze");
        let style = TextStyle {
            font: assets.load("fonts\\UnicaOne-Regular.ttf"),
            font_size: 50.0,
            ..Default::default()
        };

        let label = |s: &'static str| TextBundle {
            text: Text::with_section(s, style.clone(), Default::default()),
            style: Style {
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        };

        fn row() -> NodeBundle {
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_content: AlignContent::Stretch,
                    position: Rect::all(Val::Auto),
                    margin: Rect::all(Val::Auto),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }
        }

        fn column() -> NodeBundle {
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    align_content: AlignContent::Stretch,
                    position: Rect::all(Val::Auto),
                    margin: Rect::all(Val::Auto),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            }
        }

        c.spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|c| {
            c.spawn_bundle(column()).with_children(|c| {
                // upper
                c.spawn_bundle(row()).with_children(|c| {
                    c.spawn_bundle(label("-"));
                    c.spawn_bundle(label("w"));
                    c.spawn_bundle(label("a"));
                    c.spawn_bundle(label("s"));
                    c.spawn_bundle(label("-"));
                });

                // middle
                c.spawn_bundle(row()).with_children(|c| {
                    c.spawn_bundle(label("["));
                    c.spawn_bundle(label("1,"));
                    c.spawn_bundle(label("2,"));
                    c.spawn_bundle(label("3,"));
                    c.spawn_bundle(label("]"));
                });

                // lower
                c.spawn_bundle(row()).with_children(|c| {
                    c.spawn_bundle(label("-"));
                    c.spawn_bundle(label("z"));
                    c.spawn_bundle(label("x"));
                    c.spawn_bundle(label("c"));
                    c.spawn_bundle(label("-"));
                });
            });

            // Axis Shift Controls
            c.spawn_bundle(row()).with_children(|commands| {
                commands.spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Z".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: "<".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: "W/S".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: ">".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: "X".into(),
                                style: style.clone(),
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                });

                commands.spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Q".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: "<".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: "D/A".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: ">".into(),
                                style: style.clone(),
                            },
                            TextSection {
                                value: "E".into(),
                                style: style.clone(),
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        });
    }
}

#[derive(Component)]
struct MazeUiRoot;

// A component used to mark something to be visible only if the specific dimension is selected.
#[derive(Component)]
pub struct MazeSelectedAxisVisibilityHider {
    // The maze level this is bound to.
    level: Entity,
    // The dimension that this listener is watching for.
    dim: u8,
    // If true, the object will be visible while the axis is selected. If false, the effect is negated.
    visible_if_selected: bool,
}

pub fn maze_selected_axis_visibility_hider_updater(
    mut query: Query<(&MazeSelectedAxisVisibilityHider, &mut Visibility)>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    for changed in axis_changed.iter() {
        for (label, mut vis) in query.iter_mut() {
            if label.level == changed.level {
                vis.is_visible = changed.axis.contains(&label.dim) == label.visible_if_selected;
            }
        }
    }
}

#[derive(Component)]
pub struct MazeAxisLabel<const DIMS: usize> {
    level: Entity,
    dim: u8,
    axis: Axis,
}

pub fn maze_axis_label_update_listener<const DIMS: usize>(
    mut query: Query<(&MazeAxisLabel<DIMS>, &mut Visibility)>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    for changed in axis_changed.iter() {
        for (label, mut vis) in query.iter_mut() {
            if label.level == changed.level {
                vis.is_visible = *label.axis.get(&changed.axis) == label.dim;
            }
        }
    }
}

// Current position status text
// Position: [X, Y,  Z, W,  Q]
//     Goal: [3, 4, 12, 3, 23]

#[derive(Component)]
pub struct MazePositionLabel<const DIMS: usize> {
    level: Entity,
    text_section_to_dim: [usize; DIMS],
}

pub fn maze_position_label_update_listener<const DIMS: usize>(
    mut query: Query<(&MazePositionLabel<DIMS>, &mut Text)>,
    mut position_changed: EventReader<PositionChanged<DIMS>>,
) {
    for changed in position_changed.iter() {
        for (label, mut text) in query.iter_mut() {
            if label.level == changed.level {
                for (section_index, dimension) in label.text_section_to_dim.iter().enumerate() {
                    if let Some(section) = text.sections.get_mut(section_index) {
                        section.value = format!("{}", changed.position[*dimension]);
                    }
                }
            }
        }
    }
}
