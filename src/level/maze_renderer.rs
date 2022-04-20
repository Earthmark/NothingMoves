use std::time::{Duration, Instant};
use std::{f32::consts::PI, marker::PhantomData};

use super::{loader::MazeAssets, maze_level::*};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct MazePositionTrackerBundle {
    pub position_tracker: MazePositionTracker,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component, Default)]
pub struct MazePositionTracker {
    visible_axis: [u8; 2],
}

fn maze_level_offset(maze: &MazeLevel, axis: [u8; 2]) -> Vec3 {
    let p = maze.pos_in(axis);
    Vec3::new(-(p[0] as f32), 0.0, -(p[1] as f32))
}

pub fn update_maze_offset(
    level: Res<MazeLevel>,
    mut maze_query: Query<(&MazePositionTracker, &mut Transform)>,
    mut position_changed: EventReader<PositionChanged>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    let mut update_pos = || {
        for (renderer, mut trs) in maze_query.iter_mut() {
            trs.translation = maze_level_offset(level.as_ref(), renderer.visible_axis);
        }
    };
    for _ in position_changed.iter() {
        update_pos();
    }
    for _ in axis_changed.iter() {
        update_pos();
    }
}

#[derive(Bundle, Default)]
pub struct MazeRotationTrackerBundle {
    pub position_tracker: MazeRotationTracker,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component, Default)]
pub struct MazeRotationTracker {
    visible_axis: [u8; 2],
}

// The maze hierarchy is as follows:
// - Rotate - (player assumed to be in the center)
//  - Player Offset / scale - (enforces the player is in the center)
//   - Walls

fn get_rot_from_axis(axis: &AxisChanged) -> Quat {
    let length = PI / 2.0;
    let x_axis = if axis.axis[0] < axis.previous_axis[0] {
        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, length)
    } else if axis.axis[0] > axis.previous_axis[0] {
        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, -length)
    } else {
        Quat::IDENTITY
    };
    let y_axis = if axis.axis[1] < axis.previous_axis[1] {
        Quat::from_euler(EulerRot::XYZ, length, 0.0, 0.0)
    } else if axis.axis[1] > axis.previous_axis[1] {
        Quat::from_euler(EulerRot::XYZ, -length, 0.0, 0.0)
    } else {
        Quat::IDENTITY
    };
    x_axis * y_axis
}

pub fn maze_level_renderer(
    time: Res<Time>,
    level: Res<MazeLevel>,
    assets: Res<MazeAssets>,
    mut c: Commands,
    mut axis_changed: EventReader<AxisChanged>,
) {
    for axis in axis_changed.iter() {
        let start = get_rot_from_axis(axis).inverse();
        c.spawn_bundle(MazeRotationTrackerBundle {
            position_tracker: MazeRotationTracker {
                visible_axis: level.axis(),
            },
            transform: Transform::from_rotation(start),
            ..default()
        })
        .insert(RotateForN {
            start,
            end: Quat::IDENTITY,
            start_time: time.last_update().unwrap(),
            duration: Duration::from_millis(500),
            remove_entity: false,
        })
        .with_children(|c| {
            c.spawn_bundle(MazePositionTrackerBundle {
                position_tracker: MazePositionTracker {
                    visible_axis: level.axis(),
                },
                transform: Transform::from_translation(maze_level_offset(
                    level.as_ref(),
                    level.axis(),
                )),
                ..default()
            })
            .with_children(|c| {
                // borders
                let [px, py] = level.pos_limit();
                let lx = px as f32;
                let ly = py as f32;
                c.spawn_bundle(
                    assets.wall(
                        Transform::from_xyz((lx / 2.0) - 0.5, 0.0, -0.5)
                            .with_scale(Vec3::new(1.0, 1.0, lx))
                            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                    ),
                );
                c.spawn_bundle(
                    assets.wall(
                        Transform::from_xyz((lx / 2.0) - 0.5, 0.0, ly - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, lx))
                            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                    ),
                );
                c.spawn_bundle(
                    assets.wall(
                        Transform::from_xyz(-0.5, 0.0, (ly / 2.0) - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, ly)),
                    ),
                );
                c.spawn_bundle(
                    assets.wall(
                        Transform::from_xyz(lx - 0.5, 0.0, (ly / 2.0) - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, ly)),
                    ),
                );

                // joints
                let [psx, psy] = level.pos_limit();
                for x in 0..psx + 1 {
                    for y in 0..psy + 1 {
                        c.spawn_bundle(assets.joint(Transform::from_xyz(
                            x as f32 - 0.5,
                            0.0,
                            y as f32 - 0.5,
                        )));
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
                    c.spawn_bundle(
                        assets.wall(Transform::from_translation(position).with_rotation(rotation)),
                    );
                }
            });
        });
    }
}

pub fn start_despawn_of_render(
    time: Res<Time>,
    mut c: Commands,
    render_query: Query<Entity, (With<MazeRotationTracker>, Without<Rotate>)>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    for axis in axis_changed.iter() {
        for e in render_query.iter() {
            c.entity(e).insert(RotateForN {
                start: Quat::IDENTITY,
                end: get_rot_from_axis(axis),
                start_time: time.last_update().unwrap(),
                duration: Duration::from_millis(500),
                remove_entity: true,
            });
        }
    }
}

#[derive(Component)]
pub struct RotateForN {
    start_time: Instant,
    duration: Duration,
    start: Quat,
    end: Quat,
    remove_entity: bool,
}

pub fn rotate_for_n_update(time: Res<Time>, mut rotator: Query<(&RotateForN, &mut Transform)>) {
    for (rot, mut trs) in rotator.iter_mut() {
        *trs = Transform::from_rotation(rot.start.lerp(
            rot.end,
            (time.last_update().unwrap() - rot.start_time).as_secs_f32()
                / rot.duration.as_secs_f32(),
        ));
    }
}

pub fn remove_after_time(mut c: Commands, time: Res<Time>, query: Query<(Entity, &RotateForN)>) {
    for (e, r) in query.iter() {
        if time.last_update() >= Some(r.start_time + r.duration) {
            if r.remove_entity {
                c.entity(e).despawn_recursive();
            } else {
                c.entity(e).remove::<RotateForN>();
            }
        }
    }
}

#[derive(Component)]
pub struct RemoveAfterN<T: Component> {
    remove_time: Instant,
    phantom: PhantomData<T>,
}

pub fn remove_after_n_watcher<T: Component>(
    mut c: Commands,
    time: Res<Time>,
    query: Query<(Entity, &RemoveAfterN<T>)>,
) {
    for (e, r) in query.iter() {
        if Some(r.remove_time) <= time.last_update() {
            c.entity(e).remove::<T>();
        }
    }
}

#[derive(Component)]
pub struct Rotate {
    speed: Quat,
}

pub fn maze_level_rotator(time: Res<Time>, mut rotator: Query<(&Rotate, &mut Transform)>) {
    for (rot, mut trs) in rotator.iter_mut() {
        trs.rotate(Quat::IDENTITY.lerp(rot.speed, time.delta_seconds()));
    }
}

#[derive(Component)]
pub struct DeleteAfterDuration {
    delete_time: std::time::Instant,
}

pub fn delete_after_duration_maintainer(
    time: Res<Time>,
    mut c: Commands,
    rotator: Query<(Entity, &DeleteAfterDuration)>,
) {
    for (e, d) in rotator.iter() {
        if Some(d.delete_time) <= time.last_update() {
            c.entity(e).despawn_recursive();
        }
    }
}
