use crate::AppState;
use bevy::prelude::*;
use rand::prelude::*;

use super::maze_level::MazeLevel;

pub struct MazeLoaderPlugin;

impl Plugin for MazeLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, level_load_system)
            .add_event::<LoadLevel>();
    }
}

#[derive(Clone, Debug, Event)]
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

impl LoadLevel {
    pub fn new(d: &[u8]) -> Self {
        let dimensions = match d.len() {
            2 => DimensionLength::Two([d[0], d[1]]),
            3 => DimensionLength::Three([d[0], d[1], d[2]]),
            4 => DimensionLength::Four([d[0], d[1], d[2], d[3]]),
            5 => DimensionLength::Five([d[0], d[1], d[2], d[3], d[4]]),
            6 => DimensionLength::Six([d[0], d[1], d[2], d[3], d[4], d[5]]),
            _ => panic!("Unexpected dimension length, it must be 2-6."),
        };
        Self {
            dimensions,
            ..default()
        }
    }
}

fn level_load_system(
    mut c: Commands,
    mut events: EventReader<LoadLevel>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for level_loader in events.read() {
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
        app_state.set(AppState::InMaze);
    }
}
