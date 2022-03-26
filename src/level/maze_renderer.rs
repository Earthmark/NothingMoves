use std::f32::consts::PI;

use super::maze_level::*;
use bevy::prelude::*;

pub fn spawn_ui<const DIMS: usize>(
    mut commands: Commands,
    query: Query<(Entity, &MazeLevel<DIMS>), Added<MazeLevel<DIMS>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, _maze) in query.iter() {
        commands.spawn_bundle(MazeRendererBundle {
            renderer: MazeRenderer {
                level: entity,
                last_dims: [0, 0],
                joint: meshes.add(Mesh::from(shape::Box::new(0.2, 1.0, 0.2))),
                wall: meshes.add(Mesh::from(shape::Box::new(0.1, 0.6, 1.0))),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            },
            transform: Default::default(),
            global_transform: Default::default(),
        });
    }
}

#[derive(Bundle)]
pub struct MazeRendererBundle {
    pub renderer: MazeRenderer,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component)]
pub struct MazeRenderer {
    level: Entity,
    last_dims: [u8; 2],
    joint: Handle<Mesh>,
    wall: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl MazeRenderer {
    fn wall(&self, transform: Transform) -> PbrBundle {
        PbrBundle {
            mesh: self.wall.clone(),
            material: self.material.clone(),
            transform,
            ..Default::default()
        }
    }

    fn joint(&self, transform: Transform) -> PbrBundle {
        PbrBundle {
            mesh: self.joint.clone(),
            material: self.material.clone(),
            transform,
            ..Default::default()
        }
    }
}

pub fn update_maze_offset<const DIMS: usize>(
    mut maze_query: Query<(&MazeRenderer, &mut Transform)>,
    level_query: Query<&MazeLevel<DIMS>, Changed<MazeLevel<DIMS>>>,
) {
    for (maze, mut trs) in maze_query.iter_mut() {
        if let Ok(level) = level_query.get(maze.level) {
            let p = level.pos();
            trs.translation = Vec3::new(-(p[0] as f32), 0.0, -(p[1] as f32))
        }
    }
}

pub fn maze_level_renderer<const DIMS: usize>(
    mut commands: Commands,
    mut render_query: Query<(Entity, &mut MazeRenderer)>,
    level_query: Query<&MazeLevel<DIMS>, Changed<MazeLevel<DIMS>>>,
) {
    for (entity, mut assets) in render_query.iter_mut() {
        if let Ok(level) = level_query.get(assets.level) {
            if assets.last_dims == level.pos() {
                continue;
            }
            assets.last_dims = level.pos();

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
                for (v1, v2) in iter_walls(level) {
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
