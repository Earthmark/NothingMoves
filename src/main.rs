#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod level;
mod maze;
mod menu;

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InMaze,
    Paused,
}

fn main() {
    App::new()
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugins(level::LevelPluginBundle)
        .add_startup_system(setup)
        .run();
}

fn setup(mut c: Commands, mut maze_spawner: EventWriter<level::LoadLevel>) {
    c.spawn_bundle(OrthographicCameraBundle::new_2d());
    c.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    c.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-6.0, 10.0, -4.0)
            .looking_at(Vec3::new(2.0, 0.0, 2.0), Vec3::Y),
        ..Default::default()
    });
    c.spawn_bundle(UiCameraBundle::default());
    maze_spawner.send(level::LoadLevel {
        dimensions: level::DimensionLength::Three([4, 15, 2]),
        ..Default::default()
    });
}
