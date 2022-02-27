use std::f32::consts::PI;

use super::MazeLevel;
use bevy::prelude::*;

#[derive(Component)]
pub struct MazeAssets {
    joint: Handle<Mesh>,
    wall: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl MazeAssets {
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

pub fn maze_level_asset_loader_system(
    mut commands: Commands,
    query: Query<Entity, Added<super::LevelLoader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(MazeAssets {
            joint: meshes.add(Mesh::from(shape::Box::new(0.2, 1.0, 0.2))),
            wall: meshes.add(Mesh::from(shape::Box::new(0.1, 0.6, 1.0))),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        });
    }
}

pub fn maze_level_renderer<const DIMS: usize>(
    mut commands: Commands,
    query: Query<(Entity, &MazeLevel<DIMS>, &MazeAssets), Changed<MazeLevel<DIMS>>>,
) {
    for (entity, level, assets) in query.iter() {
        commands.entity(entity).despawn_descendants();

        commands.entity(entity).with_children(move |builder| {
            builder
                .spawn()
                .insert(Transform::default())
                .insert(GlobalTransform::default())
                .with_children(move |builder| {
                    // borders
                    let lx = level.length_x() as f32;
                    let ly = level.length_y() as f32;
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
                    for x in 0..level.length_x() + 1 {
                        for y in 0..level.length_y() + 1 {
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
                            assets.wall(
                                Transform::from_translation(position).with_rotation(rotation),
                            ),
                        );
                    }
                });
        });
    }
}
