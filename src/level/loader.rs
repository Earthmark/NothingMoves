use bevy::prelude::*;
use rand::prelude::*;

use super::{
    maze_level::{AxisChanged, PositionChanged},
    MazeLevel,
};

#[derive(Clone, Debug)]
pub struct LoadLevel {
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
    Two([u8; 2]),
    Three([u8; 3]),
    Four([u8; 4]),
    Five([u8; 5]),
    Six([u8; 6]),
}

impl Default for LoadLevel {
    fn default() -> Self {
        Self {
            rng_source: RngSource::Seeded(123456789),
            dimensions: DimensionLength::Two([2, 2]),
        }
    }
}

pub fn level_load_system(mut commands: Commands, mut events: EventReader<LoadLevel>) {
    for level_loader in events.iter() {
        let mut rng = match level_loader.rng_source {
            RngSource::Seeded(seed) => StdRng::seed_from_u64(seed),
        };
        match level_loader.dimensions {
            DimensionLength::Two(lengths) => {
                commands.spawn().insert(MazeLevel::new(&lengths, &mut rng))
            }
            DimensionLength::Three(lengths) => {
                commands.spawn().insert(MazeLevel::new(&lengths, &mut rng))
            }
            DimensionLength::Four(lengths) => {
                commands.spawn().insert(MazeLevel::new(&lengths, &mut rng))
            }
            DimensionLength::Five(lengths) => {
                commands.spawn().insert(MazeLevel::new(&lengths, &mut rng))
            }
            DimensionLength::Six(lengths) => {
                commands.spawn().insert(MazeLevel::new(&lengths, &mut rng))
            }
        };
    }
}

pub fn initial_events_on_load<const DIMS: usize>(
    query: Query<(Entity, &MazeLevel<DIMS>), Added<MazeLevel<DIMS>>>,
    mut position_changed: EventWriter<PositionChanged<DIMS>>,
    mut axis_changed: EventWriter<AxisChanged>,
) {
    for (level, maze) in query.iter() {
        position_changed.send(PositionChanged::<DIMS> {
            level,
            position: maze.position,
        });
        axis_changed.send(AxisChanged {
            level,
            axis: maze.axis,
        });
    }
}
