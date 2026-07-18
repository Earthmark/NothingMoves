use crate::assets::MazeAssets;
use crate::game::level::MazeLevel;
use crate::game::{AxisChanged, PositionChanged};
use crate::renderer::tween::TransformTween;
use crate::screens::AppState;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::f32::consts::PI;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InMaze), spawn_player)
        .add_systems(
            Update,
            (
                maze_level_renderer,
                update_maze_offset.after(maze_level_renderer),
                start_despawn_of_render,
            )
                .run_if(in_state(AppState::InMaze)),
        );
}

fn spawn_player(mut c: Commands, assets: Res<MazeAssets>) {
    c.spawn((
        DespawnOnExit(AppState::InMaze),
        Mesh3d(assets.player_mesh.clone()),
        MeshMaterial3d(assets.player_material.clone()),
    ));
}

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct MazePositionTracker {
    visible_axis: [u8; 2],
}

fn maze_level_offset(maze: &MazeLevel, axis: [u8; 2]) -> Vec3 {
    let p = maze.pos_in(axis);
    Vec3::new(-(p[0] as f32), 0.0, -(p[1] as f32))
}

fn update_maze_offset(
    level: Res<MazeLevel>,
    mut c: Commands,
    mut maze_query: Query<(Entity, &MazePositionTracker, &Transform), Without<TransformTween>>,
    mut position_changed: MessageReader<PositionChanged>,
    mut axis_changed: MessageReader<AxisChanged>,
) {
    let mut update_pos = || {
        for (e, renderer, trs) in &mut maze_query {
            if let Ok(mut c) = c.get_entity(e) {
                c.insert(TransformTween::new(
                    Duration::from_millis(100),
                    *trs,
                    trs.with_translation(maze_level_offset(level.as_ref(), renderer.visible_axis)),
                ));
            }
        }
    };
    for _ in position_changed.read() {
        update_pos();
    }
    for _ in axis_changed.read() {
        update_pos();
    }
}

#[derive(Component, Default)]
#[require(Transform, Visibility)]
struct MazeRotationTracker;

// The maze hierarchy is as follows:
// - Rotate - (player assumed to be in the center)
//  - Player Offset / scale - (enforces the player is in the center)
//   - Walls

fn get_rot_from_axis(axis: &AxisChanged) -> Quat {
    let length = PI / 4.0;
    let x_axis = match axis.axis[0].cmp(&axis.previous_axis[0]) {
        Ordering::Greater => Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, -length),
        Ordering::Less => Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, length),
        Ordering::Equal => Quat::IDENTITY,
    };
    let y_axis = match axis.axis[1].cmp(&axis.previous_axis[1]) {
        Ordering::Greater => Quat::from_euler(EulerRot::XYZ, -length, 0.0, 0.0),
        Ordering::Less => Quat::from_euler(EulerRot::XYZ, length, 0.0, 0.0),
        Ordering::Equal => Quat::IDENTITY,
    };
    x_axis * y_axis
}

fn maze_level_renderer(
    level: Res<MazeLevel>,
    assets: Res<MazeAssets>,
    mut c: Commands,
    mut axis_changed: MessageReader<AxisChanged>,
) {
    for axis in axis_changed.read() {
        c.spawn((
            DespawnOnExit(AppState::InMaze),
            MazeRotationTracker,
            TransformTween::new(
                Duration::from_millis(200),
                Transform::from_rotation(get_rot_from_axis(axis).inverse())
                    .with_scale(Vec3::new(1.0, 0.0, 1.0)),
                Transform::default(),
            ),
        ))
        .with_children(|c| {
            c.spawn((
                MazePositionTracker {
                    visible_axis: level.axis(),
                },
                Transform::from_translation(maze_level_offset(level.as_ref(), level.axis())),
            ))
            .with_children(|c| {
                // borders
                let [px, py] = level.pos_limit();
                let lx = px as f32;
                let ly = py as f32;
                c.spawn((
                    assets.wall(),
                    Transform::from_xyz((lx / 2.0) - 0.5, 0.0, -0.5)
                        .with_scale(Vec3::new(1.0, 1.0, lx))
                        .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                ));
                c.spawn((
                    assets.wall(),
                    Transform::from_xyz((lx / 2.0) - 0.5, 0.0, ly - 0.5)
                        .with_scale(Vec3::new(1.0, 1.0, lx))
                        .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                ));
                c.spawn((
                    assets.wall(),
                    Transform::from_xyz(-0.5, 0.0, (ly / 2.0) - 0.5)
                        .with_scale(Vec3::new(1.0, 1.0, ly)),
                ));
                c.spawn((
                    assets.wall(),
                    Transform::from_xyz(lx - 0.5, 0.0, (ly / 2.0) - 0.5)
                        .with_scale(Vec3::new(1.0, 1.0, ly)),
                ));

                // joints
                let [psx, psy] = level.pos_limit();
                for x in 0..psx + 1 {
                    for y in 0..psy + 1 {
                        c.spawn((
                            assets.joint(),
                            Transform::from_xyz(x as f32 - 0.5, 0.0, y as f32 - 0.5),
                        ));
                    }
                }

                // walls
                for (v1, v2) in level.iter_walls() {
                    let p1 = Vec3::new(v1[0] as f32, 0.0, v1[1] as f32);
                    let p2 = Vec3::new(v2[0] as f32, 0.0, v2[1] as f32);
                    let rotation = if v1[0] != v2[0] {
                        Quat::IDENTITY
                    } else {
                        Quat::from_rotation_y(PI / 2.0)
                    };
                    let position = p1.lerp(p2, 0.5);
                    c.spawn((
                        assets.wall(),
                        Transform::from_translation(position).with_rotation(rotation),
                    ));
                }
            });
        });
    }
}

fn start_despawn_of_render(
    mut c: Commands,
    render_query: Query<Entity, With<MazeRotationTracker>>,
    mut axis_changed: MessageReader<AxisChanged>,
) {
    for axis in axis_changed.read() {
        for e in &render_query {
            if let Ok(mut c) = c.get_entity(e) {
                c.insert(
                    TransformTween::new(
                        Duration::from_millis(200),
                        Transform::default(),
                        Transform::from_rotation(get_rot_from_axis(axis)),
                    )
                    .despawn_on_complete(),
                )
                .remove::<MazeRotationTracker>();
            }
        }
    }
}
