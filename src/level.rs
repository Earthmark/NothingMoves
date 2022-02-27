use std::{f32::consts::PI, time::Instant};

use crate::maze;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use rand::prelude::*;

struct LevelAdaptor<const DIMS: usize> {
    position: [usize; DIMS],
    maze: maze::Maze<DIMS>,
}

impl<const DIMS: usize> LevelAdaptor<DIMS> {
    fn should_make_wall(&self, position: &[usize; DIMS], direction: usize) -> bool {
        if let Some(walkable) = self.maze.can_move(position, direction) {
            !walkable
        } else {
            false
        }
    }
}

impl<const DIMS: usize> MazeLevel for LevelAdaptor<DIMS> {
    fn dims(&self) -> usize {
        DIMS
    }

    fn length_of_dim(&self, dim: usize) -> Option<usize> {
        self.maze.lengths().get(dim).copied()
    }

    fn generate_walls(
        &self,
        dim_x: usize,
        dim_y: usize,
        f: &mut dyn FnMut(&[usize; 2], &[usize; 2]),
    ) {
        // If we're mapping the same dimension, bail out.
        // If we're not accessing the correct dimensions, bail out.
        if dim_x == dim_y || dim_x >= DIMS || dim_y >= DIMS {
            return;
        }
        for cursor_x in 0..self.maze.lengths()[dim_x] {
            for cursor_y in 0..self.maze.lengths()[dim_y] {
                let mut cursor = self.position;
                cursor[dim_x] = cursor_x;
                cursor[dim_y] = cursor_y;

                if self.should_make_wall(&cursor, dim_x) {
                    f(&[cursor_x, cursor_y], &[cursor_x + 1, cursor_y]);
                }
                if self.should_make_wall(&cursor, dim_y) {
                    f(&[cursor_x, cursor_y], &[cursor_x, cursor_y + 1]);
                }
            }
        }
    }
}

trait MazeLevel: Sync + Send {
    fn dims(&self) -> usize;
    fn length_of_dim(&self, dim: usize) -> Option<usize>;
    fn generate_walls(
        &self,
        dim_x: usize,
        dim_y: usize,
        f: &mut dyn FnMut(&[usize; 2], &[usize; 2]),
    );
}

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

impl LevelLoader {
    pub fn load(&self) -> Level {
        let mut rng = match self.rng_source {
            RngSource::Seeded(seed) => StdRng::seed_from_u64(seed),
        };
        match self.dimensions {
            DimensionLength::Two(lengths) => Level::new(&lengths, &mut rng),
            DimensionLength::Three(lengths) => Level::new(&lengths, &mut rng),
            DimensionLength::Four(lengths) => Level::new(&lengths, &mut rng),
            DimensionLength::Five(lengths) => Level::new(&lengths, &mut rng),
            DimensionLength::Six(lengths) => Level::new(&lengths, &mut rng),
        }
    }
}

#[derive(Component)]
pub struct Level {
    maze: Box<dyn MazeLevel>,
    dim_x: usize,
    dim_y: usize,
    axis: FocusedAxis,
    joint: Handle<Mesh>,
    wall: Handle<Mesh>,
    border: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

enum FocusedAxis {
    X,
    Y,
}

enum Direction {
    Positive,
    Negative,
}

impl Level {
    fn new<const DIMS: usize>(lengths: &[usize; DIMS], rng: &mut impl rand::Rng) -> Self {
        Level {
            maze: Box::new(LevelAdaptor {
                maze: maze::Maze::new(lengths, rng),
                position: [0; DIMS],
            }),
            dim_x: 0,
            dim_y: 1,
            axis: FocusedAxis::X,
            joint: Default::default(),
            wall: Default::default(),
            border: Default::default(),
            material: Default::default(),
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
            Direction::Negative => linear_current
                .checked_sub(1)
                .unwrap_or(self.maze.dims() - 2),
        } % (self.maze.dims() - 1);
        let dest = if new_off_axis >= self.axis_current() {
            new_off_axis + 1
        } else {
            new_off_axis
        };

        info!(
            "Shifted off axis from {} to linear {} to {} with intermediate of {} with a limit of {}. Main is at {}",
            current,
            linear_current,
            dest,
            new_off_axis,
            self.maze.dims(),
            self.axis_current()
        );

        match self.axis {
            FocusedAxis::X => self.dim_y = dest,
            FocusedAxis::Y => self.dim_x = dest,
        };
    }

    #[inline]
    fn generate_walls(
        &self,
        dim_x: usize,
        dim_y: usize,
        f: &mut dyn FnMut(&[usize; 2], &[usize; 2]),
    ) {
        self.maze.generate_walls(dim_x, dim_y, f)
    }

    // assume dim_x and dim_y are both together.
    #[inline]
    fn length_x(&self) -> usize {
        self.maze.length_of_dim(self.dim_x).unwrap()
    }

    #[inline]
    fn length_y(&self) -> usize {
        self.maze.length_of_dim(self.dim_y).unwrap()
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_level_system)
            .add_system(level_generation_system)
            .add_system(level_input_system)
            .add_system(crate::async_promoter::promote_task_component::<Level>);
    }
}

fn level_input_system(mut query: Query<&mut Level>, keys: Res<Input<KeyCode>>) {
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
    thread_pool: Res<AsyncComputeTaskPool>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, level_loader) in query.iter() {
        let local_loader = level_loader.clone();
        let joint = meshes.add(Mesh::from(shape::Box {
            min_x: -0.1,
            max_x: 0.1,
            min_y: -0.5,
            max_y: 0.5,
            min_z: -0.1,
            max_z: 0.1,
        }));
        let wall = meshes.add(Mesh::from(shape::Box {
            min_x: -0.05,
            max_x: 0.05,
            min_y: -0.3,
            max_y: 0.3,
            min_z: -0.5,
            max_z: 0.5,
        }));
        let border = meshes.add(Mesh::from(shape::Box {
            min_x: -0.1,
            max_x: 0.1,
            min_y: -0.4,
            max_y: 0.4,
            min_z: -0.5,
            max_z: 0.5,
        }));
        let material = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
        commands
            .entity(entity)
            .insert(thread_pool.spawn(async move {
                info!("Generating maze from {:?}", local_loader);
                let now = Instant::now();
                let mut level = local_loader.load();
                info!("Maze generated in {:?}", now.elapsed());
                level.joint = joint;
                level.wall = wall;
                level.material = material;
                level.border = border;
                level
            }));
    }
}

fn level_generation_system(mut commands: Commands, query: Query<(Entity, &Level), Changed<Level>>) {
    for (entity, level) in query.iter() {
        info!("Spawning level");
        commands.entity(entity).despawn_descendants();

        commands.entity(entity).with_children(move |builder| {
            builder
                .spawn()
                .insert(Transform::default())
                .insert(GlobalTransform::default())
                .with_children(move |builder| {
                    // joints
                    for x in 0..level.length_x() + 1 {
                        for y in 0..level.length_y() + 1 {
                            builder.spawn_bundle(PbrBundle {
                                mesh: level.joint.clone(),
                                material: level.material.clone(),
                                transform: Transform::from_xyz(x as f32 - 0.5, 0.0, y as f32 - 0.5),
                                ..Default::default()
                            });
                        }
                    }

                    // walls
                    level.generate_walls(level.dim_x, level.dim_y, &mut |v1, v2| {
                        let p1 = Vec3::new(v1[0] as f32, 0.0, v1[1] as f32);
                        let p2 = Vec3::new(v2[0] as f32, 0.0, v2[1] as f32);
                        let rotation = if v1[0] != v2[0] {
                            Quat::IDENTITY
                        } else {
                            Quat::from_rotation_y(PI / 2.0)
                        };
                        let position = p1.lerp(p2, 0.5);
                        builder.spawn_bundle(PbrBundle {
                            mesh: level.wall.clone(),
                            material: level.material.clone(),
                            transform: Transform::from_translation(position)
                                .with_rotation(rotation),
                            ..Default::default()
                        });
                    });
                });
        });
    }
}
