use bevy::prelude::*;

use crate::assets::{CommonAssets, CommonSpawnable};

use super::button::SpawnableButton;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        ((num_box_apply_shift, num_box_text_label_update).chain(),),
    );
}

pub struct SpawnableNumBox {
    initial: i32,
    min: i32,
    max: i32,
}

impl Default for SpawnableNumBox {
    fn default() -> Self {
        Self {
            initial: Default::default(),
            min: Default::default(),
            max: 100,
        }
    }
}

impl SpawnableNumBox {
    pub fn new(min: i32, max: i32, initial: i32) -> Self {
        Self { initial, min, max }
    }
}

impl CommonSpawnable for SpawnableNumBox {
    fn spawn_under(self, assets: &CommonAssets, c: &mut ChildSpawnerCommands, bundle: impl Bundle) {
        let mut label_target = Entity::PLACEHOLDER;
        c.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::new(Val::Px(16.0), Val::Px(16.0), Val::Px(8.0), Val::Px(8.0)),
                margin: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            bundle,
            NumBox {
                current: self.initial,
                min: self.min,
                max: self.max,
            },
            NumBoxTextVisualLink {
                target: Entity::PLACEHOLDER,
            },
        ))
        .with_children(|c| {
            SpawnableButton::normal("+10").spawn_under(
                assets,
                c,
                NumBoxShift {
                    src: c.target_entity(),
                    offset: 10,
                },
            );
            SpawnableButton::normal("+1").spawn_under(
                assets,
                c,
                NumBoxShift {
                    src: c.target_entity(),
                    offset: 1,
                },
            );
            let label = c.spawn((
                Text2d::new(self.initial.to_string()),
                assets.common_text_style(),
            ));
            label_target = label.id();

            SpawnableButton::normal("-1").spawn_under(
                assets,
                c,
                NumBoxShift {
                    src: c.target_entity(),
                    offset: -1,
                },
            );
            SpawnableButton::normal("-10").spawn_under(
                assets,
                c,
                NumBoxShift {
                    src: c.target_entity(),
                    offset: -10,
                },
            );
        })
        .insert(NumBoxTextVisualLink {
            target: label_target,
        });
    }
}

#[derive(Component)]
pub struct NumBox {
    current: i32,
    min: i32,
    max: i32,
}

impl NumBox {
    pub fn get(&self) -> i32 {
        self.current
    }

    pub fn set(&mut self, val: i32) -> i32 {
        self.current = i32::min(i32::max(val, self.min), self.max);
        self.current
    }

    pub fn shift(&mut self, offset: i32) -> i32 {
        self.set(self.current + offset)
    }

    pub fn get_max(&self) -> i32 {
        self.max
    }

    pub fn set_max(&mut self, val: i32) {
        self.max = val;
        self.set(self.current);
    }
}

#[derive(Component)]
struct NumBoxShift {
    src: Entity,
    offset: i32,
}

fn num_box_apply_shift(
    clicked_boxes: Query<(&Interaction, &NumBoxShift), Changed<Interaction>>,
    mut count_modifier: Query<&mut NumBox>,
) {
    for (inter, shifter) in &clicked_boxes {
        if let Interaction::Pressed = inter {
            if let Ok(mut num_box) = count_modifier.get_mut(shifter.src) {
                num_box.shift(shifter.offset);
            }
        }
    }
}

#[derive(Component)]
struct NumBoxTextVisualLink {
    target: Entity,
}

fn num_box_text_label_update(
    value_watcher: Query<(&NumBox, &NumBoxTextVisualLink), Changed<NumBox>>,
    mut label_updater: Query<&mut Text>,
) {
    for (num, link) in &value_watcher {
        if let Ok(mut label) = label_updater.get_mut(link.target) {
            label.0 = num.current.to_string();
        }
    }
}
