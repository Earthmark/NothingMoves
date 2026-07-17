mod assets;
mod level;
mod maze;
mod menu;
mod ui;
mod util;

use bevy::prelude::*;

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
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(level::LevelPluginBundle)
        .add_plugins(ui::plugin)
        .add_plugins(menu::main_menu_plugin)
        //.add_plugins(EguiPlugin::default())
        //.add_plugins(
        //    WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        //)
        .add_systems(Startup, setup)
        .add_systems(Startup, CommonAssets::load_resource)
        .add_systems(Startup, loading_done)
        .run();
}

fn loading_done(mut main_state: ResMut<NextState<AppState>>) {
    main_state.set(AppState::MainMenu);
}

fn setup(mut c: Commands, mut _maze_spawner: MessageWriter<level::LoadLevel>) {
    c.spawn((
        Camera3d::default(),
        Transform::from_xyz(-6.0, 10.0, -4.0).looking_at(Vec3::new(2.0, 0.0, 2.0), Vec3::Y),
    ));
    c.spawn((
        PointLight {
            intensity: 1500.0,
            shadow_maps_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    //maze_spawner.send(level::LoadLevel {
    //    dimensions: level::DimensionLength::Three([4, 15, 2]),
    //    ..Default::default()
    //});
}
