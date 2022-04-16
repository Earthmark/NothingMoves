use std::f32::consts::PI;

use super::{loader::MazeAssets, maze_level::*};
use bevy::prelude::*;

pub fn spawn_maze_root(mut c: Commands) {
    c.spawn_bundle(MazeRendererBundle {
        renderer: MazeRenderer { last_axis: [0, 0] },
        transform: Default::default(),
        global_transform: Default::default(),
    });
}

#[derive(Bundle)]
pub struct MazeRendererBundle {
    pub renderer: MazeRenderer,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component)]
pub struct MazeRenderer {
    last_axis: [u8; 2],
}

pub fn update_maze_offset(
    level: Res<MazeLevel>,
    mut maze_query: Query<(&MazeRenderer, &mut Transform)>,
    mut position_changed: EventReader<PositionChanged>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    let mut update_pos = || {
        for (_, mut trs) in maze_query.iter_mut() {
            let p = level.pos();
            trs.translation = Vec3::new(-(p[0] as f32), 0.0, -(p[1] as f32))
        }
    };
    for _ in position_changed.iter() {
        update_pos();
    }
    for _ in axis_changed.iter() {
        update_pos();
    }
}

pub fn maze_level_renderer(
    level: Res<MazeLevel>,
    assets: Res<MazeAssets>,
    mut commands: Commands,
    mut render_query: Query<(Entity, &mut MazeRenderer)>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    for _ in axis_changed.iter() {
        for (entity, mut renderer) in render_query.iter_mut() {
            if renderer.last_axis == level.axis() {
                continue;
            }
            renderer.last_axis = level.axis();

            let mut entity = commands.entity(entity);
            entity.despawn_descendants();
            entity.with_children(|builder| {
                // borders
                let [px, py] = level.pos_limit();
                let lx = px as f32;
                let ly = py as f32;
                builder.spawn_bundle(
                    assets.wall(
                        Transform::from_xyz((lx / 2.0) - 0.5, 0.0, -0.5)
                            .with_scale(Vec3::new(1.0, 1.0, lx))
                            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                    ),
                );
                builder.spawn_bundle(
                    assets.wall(
                        Transform::from_xyz((lx / 2.0) - 0.5, 0.0, ly - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, lx))
                            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                    ),
                );
                builder.spawn_bundle(
                    assets.wall(
                        Transform::from_xyz(-0.5, 0.0, (ly / 2.0) - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, ly)),
                    ),
                );
                builder.spawn_bundle(
                    assets.wall(
                        Transform::from_xyz(lx - 0.5, 0.0, (ly / 2.0) - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, ly)),
                    ),
                );

                // joints
                let [psx, psy] = level.pos_limit();
                for x in 0..psx + 1 {
                    for y in 0..psy + 1 {
                        builder.spawn_bundle(assets.joint(Transform::from_xyz(
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
                    builder.spawn_bundle(
                        assets.wall(Transform::from_translation(position).with_rotation(rotation)),
                    );
                }
            });
        }
    }
}
