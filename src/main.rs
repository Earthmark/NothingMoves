mod assets;
mod level;
mod maze;
mod menu;
mod ui;
mod util;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use assets::CommonAssets;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    InMaze,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(level::LevelPluginBundle)
        .add_plugins(ui::plugin)
        .add_plugins(menu::main_menu_plugin)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, setup)
        .add_systems(Startup, CommonAssets::load_resource)
        .add_systems(Startup, loading_done)
        .run();
}

fn loading_done(mut main_state: ResMut<NextState<AppState>>) {
    main_state.set(AppState::MainMenu);
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
