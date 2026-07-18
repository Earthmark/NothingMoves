mod assets;
mod dbg;
mod game;
mod maze;
mod renderer;
mod screens;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..Default::default()
            }),
            game::plugin,
            renderer::plugin,
            screens::plugin,
            ui::plugin,
            assets::plugin,
            dbg::plugin,
        ))
        .run();
}
