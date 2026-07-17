use bevy::prelude::*;

pub mod button;
pub mod num_box;
pub mod visible_in;

pub fn plugin(app: &mut App) {
    app.add_plugins((button::plugin, num_box::plugin));
}
