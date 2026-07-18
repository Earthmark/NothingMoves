use bevy::prelude::*;

use crate::assets::CommonAssets;

use super::button::button_normal;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, num_box_text_label_update)
        .add_observer(on_num_box_apply_shift);
}

pub fn num_box(min: i32, max: i32, initial: i32, assets: &CommonAssets) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::new(Val::Px(4.0), Val::Px(16.0), Val::Px(8.0), Val::Px(8.0)),
            ..default()
        },
        NumBox {
            current: initial,
            min,
            max,
        },
        children![
            (button_normal("-10", assets), NumBoxShift(-10),),
            (button_normal("-1", assets), NumBoxShift(-1),),
            (
                Text::new(initial.to_string()),
                assets.common_text_style(),
                Node {
                    padding: UiRect::new(Val::Px(12.), Val::Px(12.), Val::ZERO, Val::ZERO),
                    ..default()
                },
                NumBoxLabel,
            ),
            (button_normal("+1", assets), NumBoxShift(1),),
            (button_normal("+10", assets), NumBoxShift(10),),
        ],
    )
}

#[derive(Component)]
pub struct NumBox {
    current: i32,
    min: i32,
    max: i32,
}

#[derive(Component)]
struct NumBoxLabel;

#[derive(Component)]
struct NumBoxShift(i32);

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

fn on_num_box_apply_shift(
    on: On<Pointer<Click>>,
    clicked_boxes: Query<(&NumBoxShift, &ChildOf)>,
    mut num_box: Query<&mut NumBox>,
) {
    let Ok((shift, child_of)) = clicked_boxes.get(on.event().entity) else {
        return;
    };
    if let Ok(mut num_box) = num_box.get_mut(child_of.parent()) {
        num_box.shift(shift.0);
    }
}

fn num_box_text_label_update(
    value_watcher: Query<(&NumBox, &Children), Changed<NumBox>>,
    mut label_updater: Query<&mut Text, With<NumBoxLabel>>,
) {
    for (num_box, children) in value_watcher {
        for child in children {
            if let Ok(mut text) = label_updater.get_mut(*child) {
                text.0 = num_box.get().to_string();
            }
        }
    }
}
