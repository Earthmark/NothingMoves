#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod level;
mod maze;
mod menu;

use bevy::prelude::*;

use assets::CommonAssets;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InMaze,
    Paused,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_plugins(level::LevelPluginBundle)
        .add_startup_system(setup)
        .add_startup_system(CommonAssets::load_resource)
        .run();
}

fn setup(mut c: Commands, mut maze_spawner: EventWriter<level::LoadLevel>) {
    c.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-6.0, 10.0, -4.0)
            .looking_at(Vec3::new(2.0, 0.0, 2.0), Vec3::Y),
        ..Default::default()
    });
    c.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    maze_spawner.send(level::LoadLevel {
        dimensions: level::DimensionLength::Three([4, 15, 2]),
        ..Default::default()
    });
}
