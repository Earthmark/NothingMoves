use std::time::Instant;

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

    fn generate_walls(&self, dim_x: usize, dim_y: usize) {
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

                if self.should_make_wall(&cursor, dim_x) {}
                if self.should_make_wall(&cursor, dim_y) {}
            }
        }
    }
}

trait MazeLevel: Sync + Send {
    fn dims(&self) -> usize;
    fn length_of_dim(&self, dim: usize) -> Option<usize>;
    fn generate_walls(&self, dim_x: usize, dim_y: usize);
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
    joint: Handle<Mesh>,
    wall: Handle<Mesh>,
    material: Handle<StandardMaterial>,
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
            joint: Default::default(),
            wall: Default::default(),
            material: Default::default(),
        }
    }
    fn generate_walls(&self, dim_x: usize, dim_y: usize) {
        self.maze.generate_walls(dim_x, dim_y)
    }

    // assume dim_x and dim_y are both together.
    fn length_x(&self) -> usize {
        self.maze.length_of_dim(self.dim_x).unwrap()
    }

    fn length_y(&self) -> usize {
        self.maze.length_of_dim(self.dim_y).unwrap()
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_level_system)
            .add_system(level_generation_system)
            .add_system(crate::async_promoter::promote_task_component::<Level>);
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
        let joint = meshes.add(Mesh::from(shape::Cube { size: 0.3 }));
        let wall = meshes.add(Mesh::from(shape::Cube { size: 0.1 }));
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
                level
            }));
    }
}

fn level_generation_system(mut commands: Commands, query: Query<(Entity, &Level), Changed<Level>>) {
    for (entity, level) in query.iter() {
        info!("Spawning level");
        commands.entity(entity).with_children(move |builder| {
            builder.spawn().with_children(move |builder| {
                for x in 0..level.length_x() {
                    for y in 0..level.length_y() {
                        info!("Spawning pillar at [{},{}]", x, y);
                        builder.spawn_bundle(PbrBundle {
                            mesh: level.joint.clone(),
                            material: level.material.clone(),
                            transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                            ..Default::default()
                        });
                    }
                }

                level.generate_walls(level.dim_x, level.dim_y);
            });
        });
    }
}
