use crate::assets::AssetChecker;
use crate::screens::AppState;
use crate::ui::menu_root;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Loading), spawn_loading_ui)
        .add_systems(Update, check_loaded.run_if(in_state(AppState::Loading)));
}

fn spawn_loading_ui(mut c: Commands) {
    c.spawn((menu_root(), DespawnOnExit(AppState::Loading)))
        .with_children(|c| {
            c.spawn(Text::new("Loading..."));
            c.spawn((Text::new("~/~"), StatusMarker, TextColor::default()));
        });
}

#[derive(Component)]
struct StatusMarker;

fn check_loaded(
    assets: AssetChecker,
    mut next: ResMut<NextState<AppState>>,
    mut status_updater: Query<(&mut Text, &mut TextColor), With<StatusMarker>>,
) {
    let stats = assets.load_states();
    let possible = stats.len();
    let loaded = stats.iter().filter(|s| s.is_loaded()).count();
    let errors = stats
        .iter()
        .filter_map(|s| match s {
            LoadState::Failed(e) => Some(e.to_string()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    let has_errors = !errors.is_empty();

    let status_message = if has_errors {
        errors
    } else {
        format!("{}/{}", loaded, possible)
    };

    if let Ok((mut txt, mut col)) = status_updater.single_mut() {
        txt.set_if_neq(Text(status_message));
        if has_errors {
            col.set_if_neq(TextColor(Color::srgb(1.0, 0.6, 0.6)));
        }
    }

    if !has_errors && possible == loaded {
        next.set(AppState::MainMenu);
    }
}
