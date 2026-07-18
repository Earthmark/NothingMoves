mod loading;
mod maze;
mod menu;

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InMaze,
}

pub fn plugin(app: &mut App) {
    app.init_state::<AppState>()
        .add_systems(Startup, init)
        .add_plugins((loading::plugin, maze::plugin, menu::plugin));
}

fn init(mut c: Commands) {
    c.spawn((
        Camera3d::default(),
        Transform::from_xyz(-6.0, 10.0, -4.0).looking_at(Vec3::new(2.0, 0.0, 2.0), Vec3::Y),
    ));
    c.spawn((
        PointLight {
            intensity: light_consts::lumens::VERY_LARGE_CINEMA_LIGHT,
            shadow_maps_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
