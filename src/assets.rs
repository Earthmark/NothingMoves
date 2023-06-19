use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Resource)]
pub struct CommonAssets {
    common_font: Handle<Font>,
}

pub trait CommonSpawnable {
    fn spawn_under(self, assets: &CommonAssets, c: &mut ChildBuilder);
}

impl CommonAssets {
    pub fn load_resource(mut c: Commands, assets: Res<AssetServer>) {
        c.insert_resource(Self::load(assets))
    }

    fn load(assets: Res<AssetServer>) -> Self {
        Self {
            common_font: assets.load("fonts\\UnicaOne-Regular.ttf"),
        }
    }

    pub fn spawn_common<Common: CommonSpawnable>(&self, c: &mut EntityCommands, common: Common) {
        c.with_children(|c| {
            common.spawn_under(self, c);
        });
    }

    pub fn common_text_style(&self) -> TextStyle {
        TextStyle {
            font: self.common_font.clone(),
            font_size: 50.0,
            ..default()
        }
    }
}
