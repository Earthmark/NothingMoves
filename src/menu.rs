use crate::{assets::CommonAssets, AppState};
use bevy::prelude::*;

struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenuMarker;

fn setup_main_menu(mut c: Commands, assets: Res<CommonAssets>) {
    c.spawn((
        MainMenuMarker,
        TextBundle {
            text: Text::from_section("Nothing Moves", assets.common_text_style()),
            ..default()
        },
    ));
}
