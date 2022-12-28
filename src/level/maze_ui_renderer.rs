use std::f32::consts::PI;

use super::maze_level::{self, *};
use bevy::prelude::*;

use crate::CommonAssets;

pub struct MazeUiRendererPlugin;

impl Plugin for MazeUiRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(crate::AppState::InMaze)
                .with_system(spawn_ui)
                .with_system(create_rotation_binder),
        )
        .add_system_set(
            SystemSet::on_update(crate::AppState::InMaze)
                .with_system(maze_axis_label_update_listener)
                .with_system(maze_position_label_update_listener)
                .with_system(maze_axis_label_background_updater)
                .with_system(update_guide_arrows),
        )
        .add_startup_system(MazeUiResources::load_resource);
    }
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

#[derive(Resource)]
struct MazeUiResources {
    rotate_arrow: Handle<Image>,
    rotate_arrow_inactive: Handle<Image>,
    rotate_arrow_flip: Handle<Image>,
    rotate_arrow_flip_inactive: Handle<Image>,
    move_arrow: Handle<Image>,
    move_arrow_inactive: Handle<Image>,
}

#[derive(Component)]
struct DimensionArrowUpdater {
    flipped: bool,
    enabled: bool,
}

fn update_guide_arrows(
    ui_assets: Res<MazeUiResources>,
    mut axis_changed: EventReader<super::AxisChanged>,
    mut query: Query<(&DimensionArrowUpdater, &mut UiImage)>,
) {
    for _ in axis_changed.iter() {
        for (dim, mut img) in query.iter_mut() {
            *img = match (dim.enabled, dim.flipped) {
                (true, true) => &ui_assets.rotate_arrow_flip,
                (true, false) => &ui_assets.rotate_arrow,
                (false, true) => &ui_assets.rotate_arrow_flip_inactive,
                (false, false) => &ui_assets.rotate_arrow_inactive,
            }
            .clone()
            .into();
        }
    }
}

impl MazeUiResources {
    pub fn load_resource(mut c: Commands, assets: Res<AssetServer>) {
        c.insert_resource(Self::load(assets))
    }

    //pub fn drop_resource(mut c: Commands) {
    //    c.remove_resource::<Self>()
    //}

    fn load(asset_server: Res<AssetServer>) -> Self {
        Self {
            rotate_arrow: asset_server.load("textures\\circle_arrow.png"),
            rotate_arrow_inactive: asset_server.load("textures\\circle_dash_arrow.png"),
            rotate_arrow_flip: asset_server.load("textures\\circle_arrow_flip.png"),
            rotate_arrow_flip_inactive: asset_server.load("textures\\circle_dash_arrow_flip.png"),
            move_arrow: asset_server.load("textures\\arrow.png"),
            move_arrow_inactive: asset_server.load("textures\\dash_arrow.png"),
        }
    }
}

fn create_rotation_binder(
    mut c: Commands,
    common_assets: Res<CommonAssets>,
    ui_assets: Res<MazeUiResources>,
) {
    c.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(20.0), Val::Percent(30.0)),
            position: UiRect {
                left: Val::Px(0.0),
                right: Val::Undefined,
                top: Val::Undefined,
                bottom: Val::Px(0.0),
            },
            ..default()
        },
        ..default()
    })
    .with_children(|c| {
        c.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            c.spawn(TextBundle {
                text: Text::from_section("Q", common_assets.common_text_style()),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::bottom(Val::Percent(5.0)),
                    ..default()
                },
                ..default()
            });
            c.spawn((
                ImageBundle {
                    image: ui_assets.rotate_arrow.clone().into(),
                    ..default()
                },
                DimensionArrowUpdater {
                    flipped: false,
                    enabled: true,
                },
            ));
        });

        c.spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            c.spawn(TextBundle {
                text: Text::from_section("A", common_assets.common_text_style()),
                ..default()
            });
            c.spawn(ImageBundle {
                image: ui_assets.move_arrow.clone().into(),
                ..default()
            });
            c.spawn(ImageBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(PI)),
                image: ui_assets.move_arrow_inactive.clone().into(),
                ..default()
            });
            c.spawn(TextBundle {
                text: Text::from_section("D", common_assets.common_text_style()),
                ..default()
            });
        });

        c.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            c.spawn(TextBundle {
                text: Text::from_section("E", common_assets.common_text_style()),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::top(Val::Percent(5.0)),
                    ..default()
                },
                ..default()
            });
            c.spawn((
                ImageBundle {
                    transform: Transform::from_rotation(Quat::from_rotation_z(PI)),
                    image: ui_assets.rotate_arrow_flip_inactive.clone().into(),
                    ..default()
                },
                DimensionArrowUpdater {
                    flipped: true,
                    enabled: true,
                },
            ));
        });
    });
}

fn spawn_ui(mut c: Commands, maze: Res<MazeLevel>, common_assets: Res<CommonAssets>) {
    let label = |s: &str, c: Color| TextBundle {
        text: Text::from_section(
            s,
            TextStyle {
                color: c,
                ..common_assets.common_text_style()
            },
        )
        .with_alignment(TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        }),
        style: Style {
            size: Size::new(Val::Auto, Val::Px(50.0)),
            ..default()
        },
        ..default()
    };

    let dimension_col = |dimension: usize| {
        move |c: &mut ChildBuilder| {
            c.spawn((
                NodeBundle::default(),
                MazeAxisLabel {
                    dim: dimension as u8,
                    dir: maze_level::Direction::Negative,
                },
            ))
            .with_children(|c| {
                c.spawn((
                    label("-", Color::DARK_GRAY),
                    MazeAxisLabel {
                        dim: dimension as u8,
                        dir: maze_level::Direction::Negative,
                    },
                ));
            });
            c.spawn((label("#", Color::WHITE), MazePositionLabel { dimension }));

            c.spawn((
                NodeBundle::default(),
                MazeAxisLabel {
                    dim: dimension as u8,
                    dir: maze_level::Direction::Positive,
                },
            ))
            .with_children(|c| {
                c.spawn((
                    label("-", Color::DARK_GRAY),
                    MazeAxisLabel {
                        dim: dimension as u8,
                        dir: maze_level::Direction::Positive,
                    },
                ));
            });
        }
    };

    c.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .with_children(|c| {
        c.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            c.spawn(label("[", Color::WHITE));
            for (i, _) in maze.dims_limit().iter().enumerate() {
                c.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        margin: UiRect {
                            left: Val::Px(3.0),
                            right: Val::Px(3.0),
                            ..default()
                        },
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
    dir: maze_level::Direction,
}

fn maze_axis_label_background_updater(
    level: Res<MazeLevel>,
    mut query: Query<(&MazeAxisLabel, &mut BackgroundColor)>,
    mut axis_changed: EventReader<super::AxisChanged>,
    mut position_changed: EventReader<super::PositionChanged>,
) {
    let mut update_bg = || {
        for (axis, mut ui_color) in query.iter_mut() {
            ui_color.0 = if let Some(true) = level.can_move(axis.dim, axis.dir) {
                Color::WHITE
            } else {
                Color::GRAY
            };
        }
    };
    for _ in position_changed.iter() {
        update_bg();
    }
    for _ in axis_changed.iter() {
        update_bg();
    }
}

fn maze_axis_label_update_listener(
    mut query: Query<(&MazeAxisLabel, &mut Text)>,
    mut axis_changed: EventReader<super::AxisChanged>,
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
struct MazePositionLabel {
    dimension: usize,
}

fn maze_position_label_update_listener(
    maze: Res<MazeLevel>,
    mut query: Query<(&MazePositionLabel, &mut Text)>,
    mut position_changed: EventReader<super::PositionChanged>,
) {
    for _ in position_changed.iter() {
        for (label, mut text) in query.iter_mut() {
            if let Some(section) = text.sections.first_mut() {
                if let Some(target) = maze.dims().get(label.dimension) {
                    let position = target + 1;
                    section.value = format!("{}", position);
                    section.style.color =
                        if maze.dims_limit().get(label.dimension) == Some(&position) {
                            Color::LIME_GREEN
                        } else {
                            Color::WHITE
                        };
                }
            }
        }
    }
}
