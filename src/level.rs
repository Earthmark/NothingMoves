use std::f32::consts::PI;

use crate::maze;
use bevy::prelude::*;
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

struct NullMaze;

impl MazeLevel for NullMaze {
    fn dims(&self) -> usize {
        2
    }

    fn length_of_dim(&self, dim: usize) -> Option<usize> {
        if dim < 2 {
            Some(1)
        } else {
            None
        }
    }

    fn generate_walls(
        &self,
        _dim_x: usize,
        _dim_y: usize,
        _f: &mut dyn FnMut(&[usize; 2], &[usize; 2]),
    ) {
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

fn create_adapted_maze<const DIMS: usize>(
    lengths: &[usize; DIMS],
    rng: &mut impl rand::Rng,
) -> Box<dyn MazeLevel> {
    Box::new(LevelAdaptor {
        maze: maze::Maze::new(lengths, rng),
        position: [0; DIMS],
    })
}

impl LevelLoader {
    fn load(&self) -> Box<dyn MazeLevel> {
        let mut rng = match self.rng_source {
            RngSource::Seeded(seed) => StdRng::seed_from_u64(seed),
        };
        match self.dimensions {
            DimensionLength::Two(lengths) => create_adapted_maze(&lengths, &mut rng),
            DimensionLength::Three(lengths) => create_adapted_maze(&lengths, &mut rng),
            DimensionLength::Four(lengths) => create_adapted_maze(&lengths, &mut rng),
            DimensionLength::Five(lengths) => create_adapted_maze(&lengths, &mut rng),
            DimensionLength::Six(lengths) => create_adapted_maze(&lengths, &mut rng),
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
    material: Handle<StandardMaterial>,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            maze: Box::new(NullMaze),
            // These must be different or the maze won't render.
            dim_x: 0,
            dim_y: 1,
            axis: FocusedAxis::X,
            joint: Default::default(),
            wall: Default::default(),
            material: Default::default(),
        }
    }
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
            .add_system(level_input_system);
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, level_loader) in query.iter() {
        commands.entity(entity).insert(Level {
            // TODO: Migrate maze to load async once that works for wasm.
            maze: level_loader.load(),
            joint: meshes.add(Mesh::from(shape::Box::new(0.2, 1.0, 0.2))),
            wall: meshes.add(Mesh::from(shape::Box::new(0.1, 0.6, 1.0))),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        });
    }
}

fn level_generation_system(mut commands: Commands, query: Query<(Entity, &Level), Changed<Level>>) {
    for (entity, level) in query.iter() {
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
                    builder.spawn_bundle(PbrBundle {
                        mesh: level.wall.clone(),
                        material: level.material.clone(),
                        transform: Transform::from_xyz((lx / 2.0) - 0.5, 0.0, -0.5)
                            .with_scale(Vec3::new(1.0, 1.0, lx))
                            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                        ..Default::default()
                    });
                    builder.spawn_bundle(PbrBundle {
                        mesh: level.wall.clone(),
                        material: level.material.clone(),
                        transform: Transform::from_xyz((lx / 2.0) - 0.5, 0.0, ly - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, lx))
                            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
                        ..Default::default()
                    });
                    builder.spawn_bundle(PbrBundle {
                        mesh: level.wall.clone(),
                        material: level.material.clone(),
                        transform: Transform::from_xyz(-0.5, 0.0, (ly / 2.0) - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, ly)),
                        ..Default::default()
                    });
                    builder.spawn_bundle(PbrBundle {
                        mesh: level.wall.clone(),
                        material: level.material.clone(),
                        transform: Transform::from_xyz(lx - 0.5, 0.0, (ly / 2.0) - 0.5)
                            .with_scale(Vec3::new(1.0, 1.0, ly)),
                        ..Default::default()
                    });

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
