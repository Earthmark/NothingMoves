use crate::{assets::CommonAssets, AppState};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuScene>()
            .add_systems(OnEnter(AppState::MainMenu), (reset_state, setup_main_menu));
    }
}

fn reset_state(mut setter: ResMut<NextState<MenuScene>>) {
    setter.set(MenuScene::Main);
}

// Main menu flows:
//
// Start - Pick level kind
//  | - Getting Started
//  | - New Maze (pick dimensions)
//  | | - Pick dimension count, lengths, and seed.
//  | - Load Seed
//    | - Load seed string
// Exit - Close (if not web)

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum MenuScene {
    #[default]
    Main,
    Start,
    NewMaze,
    LoadSeed,
}

#[derive(Component)]
struct GlobalMenuMarker;

#[derive(Component)]
struct MenuRootMarker(MenuScene);

fn remove_scene(mut c: Commands, q: Query<&MenuRootMarker>) {}

fn setup_main_menu(mut c: Commands, assets: Res<CommonAssets>) {
    c.spawn((
        MenuRootMarker(MenuScene::Main),
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|c| {
        c.spawn(TextBundle {
            style: Style {
                margin: UiRect::vertical(Val::Px(30.)),
                ..default()
            },
            text: Text::from_section(
                "Nothing Moves",
                TextStyle {
                    font_size: 150.,
                    ..assets.common_text_style()
                },
            ),
            ..default()
        });
        assets.spawn_common(
            &mut c.spawn(NodeBundle {
                style: Style { ..default() },
                ..default()
            }),
            crate::ui::button::SpawnableButton::primary("Start"),
        );
        assets.spawn_common(
            &mut c.spawn(NodeBundle::default()),
            crate::ui::button::SpawnableButton::normal("Exit"),
        );
    });
}
