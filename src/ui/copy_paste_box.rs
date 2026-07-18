use bevy::prelude::*;

use crate::assets::CommonAssets;
use crate::ui::button::button_normal;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (maintain_picker_label, resolve_pending_paste))
        .add_observer(copy_watcher)
        .add_observer(paste_watcher);
}

#[derive(Component, PartialEq, Debug)]
pub struct CopyPastePicker(u64);

impl CopyPastePicker {
    pub fn val(&self) -> u64 {
        self.0
    }
}

#[derive(Component)]
struct CopyPastePickerLabel;

#[derive(Component)]
struct CopyButton;

#[derive(Component)]
struct PasteButton;

pub fn copy_paste_button(assets: &CommonAssets) -> impl Bundle {
    let picked = rand::random();
    (
        Node::default(),
        CopyPastePicker(picked),
        children![
            (CopyPastePickerLabel, Text(format!("{}", picked))),
            (CopyButton, button_normal("Copy", assets)),
            (PasteButton, button_normal("Paste", assets)),
        ],
    )
}

fn maintain_picker_label(
    picker: Query<(&CopyPastePicker, &Children), Changed<CopyPastePicker>>,
    mut text: Query<&mut Text, With<CopyPastePickerLabel>>,
) {
    for (seed, children) in picker {
        for child in children {
            let Ok(mut text) = text.get_mut(*child) else {
                continue;
            };
            text.set_if_neq(Text(format!("{}", seed.0)));
        }
    }
}

fn copy_watcher(
    on: On<Pointer<Click>>,
    copy: Query<&ChildOf, With<CopyButton>>,
    picker: Query<&CopyPastePicker>,
    mut board: ResMut<Clipboard>,
) {
    let Ok(parent) = copy.get(on.event().entity) else {
        return;
    };

    let Ok(picker) = picker.get(parent.0) else {
        return;
    };

    if let Err(e) = board.set_text(picker.0.to_string()) {
        warn!("Failed to set clipboard: {}", e);
    }
}

fn paste_watcher(
    on: On<Pointer<Click>>,
    mut c: Commands,
    copy: Query<&ChildOf, With<PasteButton>>,
    mut board: ResMut<Clipboard>,
) {
    let Ok(parent) = copy.get(on.event().entity) else {
        return;
    };

    let read = board.fetch_text();
    c.entity(parent.0).insert(PendingPaste(read));
}

#[derive(Component)]
struct PendingPaste(ClipboardRead);

fn resolve_pending_paste(
    mut c: Commands,
    q: Query<(Entity, &mut PendingPaste, &mut CopyPastePicker)>,
) {
    for (e, mut p, mut picker) in q {
        let Some(result) = p.0.poll_result() else {
            continue;
        };

        if let Ok(text) = result {
            if let Ok(num) = text.parse::<u64>() {
                picker.set_if_neq(CopyPastePicker(num));
            }
        }

        c.entity(e).remove::<PendingPaste>();
    }
}
