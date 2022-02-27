use std::f32::consts::PI;

use crate::maze;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct LevelLoader {
    pub rng_source: RngSource,
    pub dimensions: DimensionLength,
}

#[derive(Clone, Debug)]
pub enum RngSource {
    Seeded(u64),
}

// Remove this once construction methods for dimensions are found.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DimensionLength {
    Two([usize; 2]),
    Three([usize; 3]),
    Four([usize; 4]),
    Five([usize; 5]),
    Six([usize; 6]),
}

impl Default for LevelLoader {
    fn default() -> Self {
        Self {
            rng_source: RngSource::Seeded(123456789),
            dimensions: DimensionLength::Two([2, 2]),
        }
    }
}

#[derive(Default, Bundle)]
pub struct LevelLoaderBundle {
    pub level_loader: LevelLoader,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

enum FocusedAxis {
    X,
    Y,
}

enum Direction {
    Positive,
    Negative,
}

#[derive(Component)]
struct MazeAssets {
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

#[derive(Component)]
struct MazeLevel<const DIMS: usize> {
    position: [usize; DIMS],
    maze: maze::Maze<DIMS>,
    dim_x: usize,
    dim_y: usize,
    axis: FocusedAxis,
}

impl<const DIMS: usize> Default for MazeLevel<DIMS> {
    fn default() -> Self {
        Self {
            maze: Default::default(),
            dim_x: 0,
            dim_y: 1,
            axis: FocusedAxis::X,
            position: [0; DIMS],
        }
    }
}

impl<const DIMS: usize> MazeLevel<DIMS> {
    fn new(lengths: &[usize; DIMS], rng: &mut impl rand::Rng) -> Self {
        Self {
            maze: crate::maze::Maze::new(lengths, rng),
            ..Default::default()
        }
    }

    fn flip_axis(&mut self) {
        match self.axis {
            FocusedAxis::X => self.axis = FocusedAxis::Y,
            FocusedAxis::Y => self.axis = FocusedAxis::X,
        }
    }

    fn axis_current(&self) -> usize {
        match self.axis {
            FocusedAxis::X => self.dim_x,
            FocusedAxis::Y => self.dim_y,
        }
    }

    fn off_axis_shift(&mut self, dir: Direction) {
        let current = match self.axis {
            FocusedAxis::X => self.dim_y,
            FocusedAxis::Y => self.dim_x,
        };

        let linear_current = if current > self.axis_current() {
            current - 1
        } else {
            current
        };

        let new_off_axis = match dir {
            Direction::Positive => linear_current.checked_add(1).unwrap_or(0),
            Direction::Negative => linear_current.checked_sub(1).unwrap_or(self.dims() - 2),
        } % (self.dims() - 1);
        let dest = if new_off_axis >= self.axis_current() {
            new_off_axis + 1
        } else {
            new_off_axis
        };

        match self.axis {
            FocusedAxis::X => self.dim_y = dest,
            FocusedAxis::Y => self.dim_x = dest,
        };
    }

    // assume dim_x and dim_y are both together.
    #[inline]
    fn length_x(&self) -> usize {
        self.length_of_dim(self.dim_x).unwrap()
    }

    #[inline]
    fn length_y(&self) -> usize {
        self.length_of_dim(self.dim_y).unwrap()
    }

    fn should_make_wall(&self, position: &[usize; DIMS], direction: usize) -> bool {
        if let Some(walkable) = self.maze.can_move(position, direction) {
            !walkable
        } else {
            false
        }
    }

    fn dims(&self) -> usize {
        DIMS
    }

    fn length_of_dim(&self, dim: usize) -> Option<usize> {
        self.maze.lengths().get(dim).copied()
    }

    fn generate_walls<'a>(
        &'a self,
        dim_x: usize,
        dim_y: usize,
    ) -> Option<impl std::iter::Iterator<Item = ([usize; 2], [usize; 2])> + 'a> {
        // If we're mapping the same dimension, bail out.
        // If we're not accessing the correct dimensions, bail out.
        if dim_x == dim_y || dim_x >= DIMS || dim_y >= DIMS {
            return None;
        }

        let length_x = self.maze.lengths()[dim_x];
        let length_y = self.maze.lengths()[dim_y];
        let position = self.position;

        Some(
            (0..length_x)
                .flat_map(move |x| (0..length_y).map(move |y| (x, y)))
                .flat_map(move |(cursor_x, cursor_y)| {
                    let mut cursor = position;
                    cursor[dim_x] = cursor_x;
                    cursor[dim_y] = cursor_y;
                    [
                        if self.should_make_wall(&cursor, dim_x) {
                            Some(([cursor_x, cursor_y], [cursor_x + 1, cursor_y]))
                        } else {
                            None
                        },
                        if self.should_make_wall(&cursor, dim_y) {
                            Some(([cursor_x, cursor_y], [cursor_x, cursor_y + 1]))
                        } else {
                            None
                        },
                    ]
                })
                .filter_map(|pair| pair),
        )
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_level_system)
            .add_system(level_generation_system::<2>)
            .add_system(level_generation_system::<3>)
            .add_system(level_generation_system::<4>)
            .add_system(level_generation_system::<5>)
            .add_system(level_generation_system::<6>)
            .add_system(level_input_system::<2>)
            .add_system(level_input_system::<3>)
            .add_system(level_input_system::<4>)
            .add_system(level_input_system::<5>)
            .add_system(level_input_system::<6>);
    }
}

fn level_input_system<const DIMS: usize>(
    mut query: Query<&mut MazeLevel<DIMS>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        for mut level in query.iter_mut() {
            level.off_axis_shift(Direction::Negative);
        }
    }
    if keys.just_pressed(KeyCode::E) {
        for mut level in query.iter_mut() {
            level.off_axis_shift(Direction::Positive);
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        for mut level in query.iter_mut() {
            level.flip_axis();
        }
    }
}

fn spawn_level_system(
    mut commands: Commands,
    query: Query<(Entity, &LevelLoader), Added<LevelLoader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, level_loader) in query.iter() {
        let mut entity = commands.entity(entity);

        let mut rng = match level_loader.rng_source {
            RngSource::Seeded(seed) => StdRng::seed_from_u64(seed),
        };
        match level_loader.dimensions {
            DimensionLength::Two(lengths) => entity.insert(MazeLevel::new(&lengths, &mut rng)),
            DimensionLength::Three(lengths) => entity.insert(MazeLevel::new(&lengths, &mut rng)),
            DimensionLength::Four(lengths) => entity.insert(MazeLevel::new(&lengths, &mut rng)),
            DimensionLength::Five(lengths) => entity.insert(MazeLevel::new(&lengths, &mut rng)),
            DimensionLength::Six(lengths) => entity.insert(MazeLevel::new(&lengths, &mut rng)),
        };

        entity.insert(MazeAssets {
            joint: meshes.add(Mesh::from(shape::Box::new(0.2, 1.0, 0.2))),
            wall: meshes.add(Mesh::from(shape::Box::new(0.1, 0.6, 1.0))),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        });
    }
}

fn level_generation_system<const DIMS: usize>(
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
                    if let Some(iter) = level.generate_walls(level.dim_x, level.dim_y) {
                        for (v1, v2) in iter {
                            let p1 = Vec3::new(v1[0] as f32, 0.0, v1[1] as f32);
                            let p2 = Vec3::new(v2[0] as f32, 0.0, v2[1] as f32);
                            let rotation = if v1[0] != v2[0] {
                                Quat::IDENTITY
                            } else {
                                Quat::from_rotation_y(PI / 2.0)
                            };
                            let position = p1.lerp(p2, 0.5);
                            builder.spawn_bundle(assets.wall(
                                Transform::from_translation(position).with_rotation(rotation),
                            ));
                        }
                    }
                });
        });
    }
}
