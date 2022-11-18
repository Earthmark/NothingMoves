use crate::AppState;
use bevy::prelude::*;

struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu));
    }
}

fn setup_main_menu(mut _c: Commands) {}
