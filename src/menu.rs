use crate::AppState;
use bevy::prelude::*;

struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)));
    }
}

fn setup_main_menu(mut _c: Commands) {}
