use bevy::prelude::*;
use rand::prelude::*;

use super::{maze_renderer::MazeRenderer, maze_renderer::MazeRendererBundle, MazeLevel};

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
    mut commands: Commands,
    mut events: EventReader<LoadLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    for level_loader in events.iter() {
        let mut rng = match level_loader.rng_source {
            RngSource::Seeded(seed) => StdRng::seed_from_u64(seed),
        };
        let entity = match level_loader.dimensions {
            DimensionLength::Two(lengths) => commands
                .spawn()
                .insert(MazeLevel::new(&lengths, &mut rng))
                .id(),
            DimensionLength::Three(lengths) => commands
                .spawn()
                .insert(MazeLevel::new(&lengths, &mut rng))
                .id(),
            DimensionLength::Four(lengths) => commands
                .spawn()
                .insert(MazeLevel::new(&lengths, &mut rng))
                .id(),
            DimensionLength::Five(lengths) => commands
                .spawn()
                .insert(MazeLevel::new(&lengths, &mut rng))
                .id(),
            DimensionLength::Six(lengths) => commands
                .spawn()
                .insert(MazeLevel::new(&lengths, &mut rng))
                .id(),
        };

        commands.spawn_bundle(MazeRendererBundle {
            renderer: MazeRenderer::new(entity, &mut meshes, &mut materials),
            transform: Default::default(),
            global_transform: Default::default(),
        });
    }
}
