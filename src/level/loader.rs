use bevy::prelude::*;
use rand::prelude::*;

use super::MazeLevel;

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

pub fn level_load_system(
    mut commands: Commands,
    query: Query<(Entity, &LevelLoader), Added<LevelLoader>>,
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
    }
}
