use std::borrow::Cow;

use bevy::prelude::*;

use crate::assets::{CommonAssets, CommonSpawnable};

#[derive(Component)]
enum ButtonKind {
    Primary,
    Secondary,
}

#[derive(Component)]
struct Button {
    kind: ButtonKind,
    text: Cow<'static, str>,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const PRIM_NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const PRIM_HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRIM_PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_interaction_updates(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonKind),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background, kind) in &mut interaction_query {
        let bg_color = match (interaction, kind) {
            (Interaction::Clicked, ButtonKind::Primary) => PRIM_PRESSED_BUTTON,
            (Interaction::Hovered, ButtonKind::Primary) => PRIM_HOVERED_BUTTON,
            (Interaction::None, ButtonKind::Primary) => PRIM_NORMAL_BUTTON,
            (Interaction::Clicked, ButtonKind::Secondary) => PRESSED_BUTTON,
            (Interaction::Hovered, ButtonKind::Secondary) => HOVERED_BUTTON,
            (Interaction::None, ButtonKind::Secondary) => NORMAL_BUTTON,
        };
        *background = bg_color.into();
    }
}

impl CommonSpawnable for Button {
    fn spawn_under(self, assets: &CommonAssets, c: &mut ChildBuilder) {
        c.spawn(ButtonBundle { ..default() }).with_children(|c| {
            c.spawn(TextBundle {
                text: Text::from_section(self.text, assets.common_text_style()),
                ..default()
            });
        });
    }
}
