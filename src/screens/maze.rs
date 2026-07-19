use super::AppState;
use crate::assets::CommonAssets;
use crate::game::level::MazeLevel;
use crate::ui::button::button_primary;
use crate::ui::menu_root;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameScreen>()
        .add_systems(
            Update,
            (
                watch_for_win.run_if(resource_exists_and_changed::<MazeLevel>),
                level_escape,
            )
                .run_if(in_state(AppState::InMaze)),
        )
        .add_systems(OnEnter(GameScreen::Paused), pause_menu)
        .add_systems(OnEnter(GameScreen::Victory), victory_menu);
}

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[source(AppState = AppState::InMaze)]
pub enum GameScreen {
    #[default]
    Level,
    Paused,
    Victory,
}

fn watch_for_win(level: Res<MazeLevel>, mut w: ResMut<NextState<GameScreen>>) {
    if level
        .sides()
        .iter()
        .zip(level.pos())
        .all(|(s, p)| *s == *p + 1)
    {
        w.set(GameScreen::Victory);
    }
}

fn level_escape(
    keys: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameScreen>>,
    mut game: ResMut<NextState<GameScreen>>,
    mut app: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            GameScreen::Level => game.set(GameScreen::Paused),
            GameScreen::Paused => game.set(GameScreen::Level),
            GameScreen::Victory => app.set(AppState::MainMenu),
        }
    }
}

fn pause_menu(mut c: Commands, assets: Res<CommonAssets>) {
    c.spawn((
        menu_root(),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        DespawnOnExit(GameScreen::Paused),
    ))
    .with_children(|c| {
        c.spawn((Text::new("Paused"), assets.common_text_style()));
        c.spawn(Node { ..default() }).with_children(|c| {
            c.spawn(button_primary("Resume", &assets)).observe(
                |_: On<Pointer<Click>>, mut game: ResMut<NextState<GameScreen>>| {
                    game.set(GameScreen::Level);
                },
            );
            c.spawn(button_primary("Exit", &assets)).observe(
                |_: On<Pointer<Click>>, mut app: ResMut<NextState<AppState>>| {
                    app.set(AppState::MainMenu);
                },
            );
        });
    });
}

fn victory_menu(mut c: Commands, assets: Res<CommonAssets>) {
    c.spawn((
        menu_root(),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        DespawnOnExit(GameScreen::Victory),
    ))
    .with_children(|c| {
        c.spawn((Text::new("Victory!"), assets.common_text_style()));
        c.spawn(Node { ..default() }).with_children(|c| {
            c.spawn(button_primary("Exit", &assets)).observe(
                |_: On<Pointer<Click>>, mut app: ResMut<NextState<AppState>>| {
                    app.set(AppState::MainMenu);
                },
            );
        });
    });
}
