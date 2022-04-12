use crate::AppState;
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

pub fn level_load_system(
    mut c: Commands,
    mut events: EventReader<LoadLevel>,
    mut app_state: ResMut<State<AppState>>,
) {
    for level_loader in events.iter() {
        let mut rng = match level_loader.rng_source {
            RngSource::Seeded(seed) => StdRng::seed_from_u64(seed),
        };
        c.insert_resource(match level_loader.dimensions {
            DimensionLength::Two(lengths) => MazeLevel::new(&lengths, &mut rng),
            DimensionLength::Three(lengths) => MazeLevel::new(&lengths, &mut rng),
            DimensionLength::Four(lengths) => MazeLevel::new(&lengths, &mut rng),
            DimensionLength::Five(lengths) => MazeLevel::new(&lengths, &mut rng),
            DimensionLength::Six(lengths) => MazeLevel::new(&lengths, &mut rng),
        });
        app_state.push(AppState::InMaze).unwrap();
    }
}

pub fn initial_events_on_load(
    maze: Res<MazeLevel>,
    mut position_changed: EventWriter<PositionChanged>,
    mut axis_changed: EventWriter<AxisChanged>,
) {
    position_changed.send(PositionChanged {
        position: maze.pos().into(),
    });
    axis_changed.send(AxisChanged { axis: maze.axis() });
}
