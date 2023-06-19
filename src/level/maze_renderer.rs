use super::maze_level::*;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::f32::consts::PI;
use std::time::Duration;

pub struct MazeRendererPlugin;

impl Plugin for MazeRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_maze_assets)
            .add_system(spawn_player.in_schedule(OnEnter(crate::AppState::InMaze)))
            .add_systems(
                (
                    maze_level_renderer,
                    rotate_for_n_update,
                    remove_after_time::<RemoveAt>,
                    remove_after_time::<ShiftForN>,
                    update_maze_offset.after(maze_level_renderer),
                    start_despawn_of_render,
                )
                    .in_set(OnUpdate(crate::AppState::InMaze)),
            );
    }
}

fn load_maze_assets(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    c.insert_resource(MazeAssets {
        joint: meshes.add(Mesh::from(shape::Box::new(0.2, 1.0, 0.2))),
        wall: meshes.add(Mesh::from(shape::Box::new(0.1, 0.6, 1.0))),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    });
}

#[derive(Resource)]
struct MazeAssets {
    joint: Handle<Mesh>,
    wall: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl MazeAssets {
    pub fn wall(&self, transform: Transform) -> PbrBundle {
        PbrBundle {
            mesh: self.wall.clone(),
            material: self.material.clone(),
            transform,
            ..Default::default()
        }
    }

    pub fn joint(&self, transform: Transform) -> PbrBundle {
        PbrBundle {
            mesh: self.joint.clone(),
            material: self.material.clone(),
            transform,
            ..Default::default()
        }
    }
}

fn spawn_player(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    c.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.3,
            ..default()
        })),
        material: materials.add(Color::rgb(0.5, 0.5, 0.8).into()),
        ..Default::default()
    });
}

#[derive(Bundle, Default)]
pub struct MazePositionTrackerBundle {
    pub position_tracker: MazePositionTracker,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub computed_visibility: ComputedVisibility,
    pub visibility: Visibility,
}

#[derive(Component, Default)]
pub struct MazePositionTracker {
    visible_axis: [u8; 2],
}

fn maze_level_offset(maze: &MazeLevel, axis: [u8; 2]) -> Vec3 {
    let p = maze.pos_in(axis);
    Vec3::new(-(p[0] as f32), 0.0, -(p[1] as f32))
}

fn update_maze_offset(
    level: Res<MazeLevel>,
    time: Res<Time>,
    mut c: Commands,
    mut maze_query: Query<(Entity, &MazePositionTracker, &Transform), Without<RemoveAt>>,
    mut position_changed: EventReader<super::PositionChanged>,
    mut axis_changed: EventReader<super::AxisChanged>,
) {
    let mut update_pos = || {
        for (e, renderer, trs) in &mut maze_query {
            if let Some(mut c) = c.get_entity(e) {
                c.insert(ShiftForN {
                    duration: Duration::from_millis(100),
                    start: *trs,
                    end: trs
                        .with_translation(maze_level_offset(level.as_ref(), renderer.visible_axis)),
                    ..ShiftForN::new(time.elapsed())
                });
            }
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
struct MazeRotationTrackerBundle {
    pub position_tracker: MazeRotationTracker,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub computed_visibility: ComputedVisibility,
    pub visibility: Visibility,
}

#[derive(Component, Default)]
struct MazeRotationTracker;

// The maze hierarchy is as follows:
// - Rotate - (player assumed to be in the center)
//  - Player Offset / scale - (enforces the player is in the center)
//   - Walls

fn get_rot_from_axis(axis: &super::AxisChanged) -> Quat {
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
    time: Res<Time>,
    level: Res<MazeLevel>,
    assets: Res<MazeAssets>,
    mut c: Commands,
    mut axis_changed: EventReader<super::AxisChanged>,
) {
    for axis in axis_changed.iter() {
        let start = get_rot_from_axis(axis).inverse();
        c.spawn((
            MazeRotationTrackerBundle {
                position_tracker: MazeRotationTracker,
                transform: Transform::from_rotation(start),
                ..default()
            },
            ShiftForN {
                start: Transform::from_rotation(start).with_scale(Vec3::new(1.0, 0.0, 1.0)),
                duration: Duration::from_millis(200),
                ..ShiftForN::new(time.elapsed())
            },
        ))
        .with_children(|c| {
            c.spawn(MazePositionTrackerBundle {
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
                c.spawn(
                    assets.wall(
                        Transform::from_xyz((lx / 2.0) - 0.5, 0.0, -0.5)
                            .with_scale(Vec3::new(1.0, 1.0, lx))
                            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                    ),
                );
                c.spawn(
                    assets.wall(
                        Transform::from_xyz((lx / 2.0) - 0.5, 0.0, ly - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, lx))
                            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                    ),
                );
                c.spawn(
                    assets.wall(
                        Transform::from_xyz(-0.5, 0.0, (ly / 2.0) - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, ly)),
                    ),
                );
                c.spawn(
                    assets.wall(
                        Transform::from_xyz(lx - 0.5, 0.0, (ly / 2.0) - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, ly)),
                    ),
                );

                // joints
                let [psx, psy] = level.pos_limit();
                for x in 0..psx + 1 {
                    for y in 0..psy + 1 {
                        c.spawn(assets.joint(Transform::from_xyz(
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
                    c.spawn(
                        assets.wall(Transform::from_translation(position).with_rotation(rotation)),
                    );
                }
            });
        });
    }
}

fn start_despawn_of_render(
    time: Res<Time>,
    mut c: Commands,
    render_query: Query<Entity, (With<MazeRotationTracker>, Without<RemoveAt>)>,
    mut axis_changed: EventReader<super::AxisChanged>,
) {
    for axis in axis_changed.iter() {
        for e in &render_query {
            if let Some(mut c) = c.get_entity(e) {
                c.insert((
                    ShiftForN {
                        end: Transform::from_rotation(get_rot_from_axis(axis)),
                        duration: Duration::from_millis(200),
                        ..ShiftForN::new(time.elapsed())
                    },
                    RemoveAt {
                        end_time: time.elapsed() + Duration::from_millis(200),
                    },
                ))
                .remove::<MazeRotationTracker>();
            }
        }
    }
}

trait TimedComponent {
    fn end_time(&self) -> Duration;
    fn on_elapsed(&self, c: &mut Commands, e: Entity);
}

fn remove_after_time<Comp: TimedComponent + Component>(
    mut c: Commands,
    time: Res<Time>,
    query: Query<(Entity, &Comp)>,
) {
    for (e, r) in &query {
        if time.elapsed() > r.end_time() {
            r.on_elapsed(&mut c, e);
        }
    }
}

#[derive(Component)]
struct ShiftForN {
    start_time: Duration,
    duration: Duration,
    start: Transform,
    end: Transform,
}

impl ShiftForN {
    fn new(start_time: Duration) -> Self {
        Self {
            start_time,
            duration: Default::default(),
            start: Default::default(),
            end: Default::default(),
        }
    }

    fn lerp(&self, val: f32) -> Transform {
        Transform {
            translation: self.start.translation.lerp(self.end.translation, val),
            rotation: self.start.rotation.slerp(self.end.rotation, val),
            scale: self.start.scale.lerp(self.end.scale, val),
        }
    }
}

impl TimedComponent for ShiftForN {
    fn end_time(&self) -> Duration {
        self.start_time + self.duration
    }

    fn on_elapsed(&self, c: &mut Commands, e: Entity) {
        if let Some(mut c) = c.get_entity(e) {
            c.remove::<ShiftForN>();
        }
    }
}

fn rotate_for_n_update(time: Res<Time>, mut rotator: Query<(&ShiftForN, &mut Transform)>) {
    for (shift, mut trs) in &mut rotator {
        let lerp_val =
            (time.elapsed() - shift.start_time).as_secs_f32() / shift.duration.as_secs_f32();
        let lerp_val = lerp_val.clamp(0.0, 1.0);
        *trs = shift.lerp(lerp_val);
    }
}

#[derive(Component)]
struct RemoveAt {
    end_time: Duration,
}

impl TimedComponent for RemoveAt {
    fn end_time(&self) -> Duration {
        self.end_time
    }

    fn on_elapsed(&self, c: &mut Commands, e: Entity) {
        if let Some(c) = c.get_entity(e) {
            c.despawn_recursive();
        }
    }
}
