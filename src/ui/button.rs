use bevy::prelude::*;

use crate::assets::{CommonAssets, CommonSpawnable};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            active_button_color_updates,
            button_made_inactive_color_update,
            button_made_active_color_update,
        ),
    );
}

#[derive(Component)]
pub enum ButtonKind {
    Primary,
    Normal,
}

impl ButtonKind {
    fn palette_for(&self) -> &ColorPalette {
        match self {
            Self::Normal => &NORMAL_PALETTE,
            Self::Primary => &PRIMARY_PALETTE,
        }
    }

    fn color_for(&self, i: &Interaction) -> Color {
        self.palette_for().color_for(i)
    }

    fn inactive_color(&self) -> Color {
        self.palette_for().inactive
    }
}

const NORMAL_PALETTE: ColorPalette = ColorPalette {
    normal: Color::srgb(0.15, 0.15, 0.15),
    hovered: Color::srgb(0.25, 0.25, 0.25),
    clicked: Color::srgb(0.35, 0.75, 0.35),
    inactive: Color::srgb(0.15, 0.15, 0.15),
};

const PRIMARY_PALETTE: ColorPalette = ColorPalette {
    normal: Color::srgb(0.15, 0.15, 0.15),
    hovered: Color::srgb(0.25, 0.25, 0.25),
    clicked: Color::srgb(0.35, 0.75, 0.35),
    inactive: Color::srgb(0.15, 0.15, 0.15),
};

struct ColorPalette {
    normal: Color,
    hovered: Color,
    clicked: Color,
    inactive: Color,
}

impl ColorPalette {
    fn color_for(&self, i: &Interaction) -> Color {
        match i {
            Interaction::Pressed => self.clicked,
            Interaction::Hovered => self.hovered,
            Interaction::None => self.normal,
        }
    }
}

type ButtonHoverProps<'a> = (&'a Interaction, &'a mut BackgroundColor, &'a ButtonKind);
type ButtonHoverWatch = (Changed<Interaction>, With<Button>);

fn active_button_color_updates(mut interaction_query: Query<ButtonHoverProps, ButtonHoverWatch>) {
    for (interaction, mut background, kind) in &mut interaction_query {
        *background = kind.color_for(interaction).into();
    }
}

fn button_made_inactive_color_update(
    mut removed_buttons: RemovedComponents<Button>,
    mut interaction_query: Query<(&mut BackgroundColor, &ButtonKind)>,
) {
    for e in removed_buttons.read() {
        if let Ok((mut background, kind)) = interaction_query.get_mut(e) {
            *background = kind.inactive_color().into();
        }
    }
}

fn button_made_active_color_update(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &ButtonKind), Added<Button>>,
) {
    for (interaction, mut background, kind) in &mut interaction_query {
        *background = kind.color_for(interaction).into();
    }
}

pub struct SpawnableButton {
    kind: ButtonKind,
    text: String,
}

impl SpawnableButton {
    pub fn primary(text: impl Into<String>) -> Self {
        Self {
            kind: ButtonKind::Primary,
            text: text.into(),
        }
    }

    pub fn normal(text: impl Into<String>) -> Self {
        Self {
            kind: ButtonKind::Normal,
            text: text.into(),
        }
    }
}

impl CommonSpawnable for SpawnableButton {
    fn spawn_under(self, assets: &CommonAssets, c: &mut ChildSpawnerCommands, bundle: impl Bundle) {
        c.spawn((
            Node {
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::new(Val::Px(12.0), Val::Px(12.0), Val::Px(4.0), Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(self.kind.color_for(&Interaction::None)),
            self.kind,
            Button,
            bundle,
        ))
        .with_children(|c| {
            c.spawn((Text::new(self.text), assets.common_text_style()));
        });
    }
}
