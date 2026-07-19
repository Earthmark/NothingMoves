use bevy::prelude::*;

pub mod button;
pub mod copy_paste_box;
pub mod num_box;
mod ui_root;

pub use ui_root::menu_root;

pub fn plugin(app: &mut App) {
    app.add_plugins((button::plugin, copy_paste_box::plugin, num_box::plugin));
}
