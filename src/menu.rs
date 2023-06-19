use crate::{assets::CommonAssets, AppState};
use bevy::prelude::*;

pub struct MainMenuPlugin;

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
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                size: Size::width(Val::Percent(100.)),
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                gap: Size::all(Val::Px(5.0)),
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
            text: Text::from_section("Nothing Moves", assets.common_text_style()),
            ..default()
        });
        assets.spawn_common(
            &mut c.spawn(NodeBundle {
                style: Style {
                    ..default()
                },
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
