#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod level;
mod maze;
mod menu;
mod ui;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        .init_state::<AppState>()
        .add_plugins(DefaultPlugins.set(AssetPlugin { ..default() }))
        .add_plugins(level::LevelPluginBundle)
        .add_plugins(ui::button::CommonButtonPlugin)
        .add_plugins(menu::MainMenuPlugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .add_systems(Startup, CommonAssets::load_resource)
        .run();
}

fn setup(mut c: Commands, mut _maze_spawner: EventWriter<level::LoadLevel>) {
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

    //maze_spawner.send(level::LoadLevel {
    //    dimensions: level::DimensionLength::Three([4, 15, 2]),
    //    ..Default::default()
    //});
}
