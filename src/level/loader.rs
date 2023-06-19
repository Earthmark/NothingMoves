use crate::AppState;
use bevy::prelude::*;
use rand::prelude::*;

use super::maze_level::MazeLevel;

pub struct MazeLoaderPlugin;

impl Plugin for MazeLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_load_system).add_event::<LoadLevel>();
    }
}

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

fn level_load_system(
    mut c: Commands,
    mut events: EventReader<LoadLevel>,
    mut app_state: ResMut<NextState<AppState>>,
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
        app_state.set(AppState::InMaze);
    }
}
