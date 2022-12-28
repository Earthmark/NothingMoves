use bevy::prelude::*;

#[derive(Resource)]
pub struct CommonAssets {
    common_font: Handle<Font>,
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

    pub fn common_text_style(&self) -> TextStyle {
        TextStyle {
            font: self.common_font.clone(),
            font_size: 50.0,
            ..default()
        }
    }
}
