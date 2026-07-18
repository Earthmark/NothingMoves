use bevy::prelude::*;

use crate::assets::CommonAssets;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, button_color_updates);
}

pub const NORMAL_PALETTE: InteractionPalette = InteractionPalette {
    normal: Color::srgb(0.15, 0.15, 0.15),
    hovered: Color::srgb(0.25, 0.25, 0.25),
    clicked: Color::srgb(0.35, 0.75, 0.35),
};

pub const PRIMARY_PALETTE: InteractionPalette = InteractionPalette {
    normal: Color::srgb(0.15, 0.15, 0.15),
    hovered: Color::srgb(0.25, 0.25, 0.25),
    clicked: Color::srgb(0.35, 0.75, 0.35),
};

pub fn button(
    text: impl Into<String>,
    palette: InteractionPalette,
    assets: &CommonAssets,
) -> impl Bundle {
    let bg_color = palette.normal;
    (
        Button,
        Node {
            padding: UiRect::axes(Val::Px(12.0), Val::Px(4.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        palette,
        BackgroundColor(bg_color),
        children![(Text::new(text), assets.common_text_style())],
    )
}

pub fn button_normal(text: impl Into<String>, assets: &CommonAssets) -> impl Bundle {
    button(text, NORMAL_PALETTE, assets)
}

pub fn button_primary(text: impl Into<String>, assets: &CommonAssets) -> impl Bundle {
    button(text, PRIMARY_PALETTE, assets)
}

#[derive(Component, Clone)]
pub struct InteractionPalette {
    normal: Color,
    hovered: Color,
    clicked: Color,
}

fn button_color_updates(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &InteractionPalette),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background, palette) in &mut interaction_query {
        background.0 = match interaction {
            Interaction::None => palette.normal,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.clicked,
        };
    }
}
