use bevy::prelude::*;

use crate::assets::{CommonAssets, CommonSpawnable};

pub struct CommonButtonPlugin;

impl Plugin for CommonButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            active_button_color_updates,
            button_made_inactive_color_update,
            button_made_active_color_update,
        ));
    }
}

#[derive(Component)]
pub enum ButtonKind {
    Primary,
    Normal,
}

impl ButtonKind {
    fn palette_for(&self) -> &ColorPallete {
        match self {
            Self::Normal => &NORMAL_PALLETE,
            Self::Primary => &PRIMARY_PALLETE,
        }
    }

    fn color_for(&self, i: &Interaction) -> Color {
        self.palette_for().color_for(i)
    }

    fn inactive_color(&self) -> Color {
        self.palette_for().inactive
    }
}

const NORMAL_PALLETE: ColorPallete = ColorPallete {
    normal: Color::rgb(0.15, 0.15, 0.15),
    hovered: Color::rgb(0.25, 0.25, 0.25),
    clicked: Color::rgb(0.35, 0.75, 0.35),
    inactive: Color::rgb(0.15, 0.15, 0.15),
};

const PRIMARY_PALLETE: ColorPallete = ColorPallete {
    normal: Color::rgb(0.15, 0.15, 0.15),
    hovered: Color::rgb(0.25, 0.25, 0.25),
    clicked: Color::rgb(0.35, 0.75, 0.35),
    inactive: Color::rgb(0.15, 0.15, 0.15),
};

struct ColorPallete {
    normal: Color,
    hovered: Color,
    clicked: Color,
    inactive: Color,
}

impl ColorPallete {
    fn color_for(&self, i: &Interaction) -> Color {
        match i {
            Interaction::Clicked => self.clicked,
            Interaction::Hovered => self.hovered,
            Interaction::None => self.normal,
        }
    }
}

fn active_button_color_updates(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonKind),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background, kind) in &mut interaction_query {
        *background = kind.color_for(interaction).into();
    }
}

fn button_made_inactive_color_update(
    mut removed_buttons: RemovedComponents<Button>,
    mut interaction_query: Query<(&mut BackgroundColor, &ButtonKind)>,
) {
    let mut iter = interaction_query.iter_many_mut(removed_buttons.iter());
    while let Some((mut background, kind)) = iter.fetch_next() {
        *background = kind.inactive_color().into();
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
    fn spawn_under(self, assets: &CommonAssets, c: &mut ChildBuilder) {
        c.spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::new(Val::Px(16.0), Val::Px(16.0), Val::Px(8.0), Val::Px(8.0)),
                    margin: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                background_color: self.kind.color_for(&Interaction::None).into(),
                ..default()
            },
            self.kind,
        ))
        .with_children(|c| {
            c.spawn(TextBundle {
                text: Text::from_section(self.text, assets.common_text_style()),
                style: Style { ..default() },
                ..default()
            });
        });
    }
}
