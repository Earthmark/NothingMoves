use bevy::asset::{LoadState, UntypedAssetId};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_assets);
}

#[derive(Resource)]
pub struct CommonAssets {
    pub common_font: Handle<Font>,
}

impl CommonAssets {
    fn handles(&self) -> impl Iterator<Item = UntypedAssetId> {
        [self.common_font.id().untyped()].into_iter()
    }
}

#[derive(Resource)]
pub struct MazeAssets {
    pub joint: Handle<Mesh>,
    pub wall: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,

    pub player_mesh: Handle<Mesh>,
    pub player_material: Handle<StandardMaterial>,
}

impl MazeAssets {
    fn handles(&self) -> impl Iterator<Item = UntypedAssetId> {
        [].into_iter()
    }
}

#[derive(Resource)]
pub struct MazeUiAssets {
    pub rotate_arrow: Handle<Image>,
    pub rotate_arrow_inactive: Handle<Image>,
    pub rotate_arrow_flip: Handle<Image>,
    pub rotate_arrow_flip_inactive: Handle<Image>,
    pub move_arrow: Handle<Image>,
    pub move_arrow_inactive: Handle<Image>,
}

impl MazeUiAssets {
    fn handles(&self) -> impl Iterator<Item = UntypedAssetId> {
        [
            self.rotate_arrow.id().untyped(),
            self.rotate_arrow_inactive.id().untyped(),
            self.rotate_arrow_flip.id().untyped(),
            self.rotate_arrow_flip_inactive.id().untyped(),
            self.move_arrow.id().untyped(),
            self.move_arrow_inactive.id().untyped(),
        ]
        .into_iter()
    }
}

#[derive(SystemParam)]
pub struct AssetChecker<'w> {
    server: Res<'w, AssetServer>,

    common: Res<'w, CommonAssets>,
    maze: Res<'w, MazeAssets>,
    maze_ui: Res<'w, MazeUiAssets>,
}

impl AssetChecker<'_> {
    pub fn load_states(&self) -> Vec<LoadState> {
        [].into_iter()
            .chain(self.common.handles())
            .chain(self.maze.handles())
            .chain(self.maze_ui.handles())
            .map(|id| self.server.load_state(id))
            .collect()
    }
}

fn load_assets(
    mut c: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    c.insert_resource(CommonAssets {
        common_font: server.load("fonts/UnicaOne-Regular.ttf"),
    });
    c.insert_resource(MazeAssets {
        joint: meshes.add(Cuboid::new(0.2, 1.0, 0.2)),
        wall: meshes.add(Cuboid::new(0.1, 0.6, 1.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        }),

        player_mesh: meshes.add(Mesh::from(Capsule3d {
            radius: 0.3,
            ..default()
        })),
        player_material: materials.add(StandardMaterial::from(Color::srgb(0.5, 0.5, 0.8))),
    });
    c.insert_resource(MazeUiAssets {
        rotate_arrow: server.load("textures/circle_arrow.png"),
        rotate_arrow_inactive: server.load("textures/circle_dash_arrow.png"),
        rotate_arrow_flip: server.load("textures/circle_arrow_flip.png"),
        rotate_arrow_flip_inactive: server.load("textures/circle_dash_arrow_flip.png"),
        move_arrow: server.load("textures/arrow.png"),
        move_arrow_inactive: server.load("textures/dash_arrow.png"),
    });
}

impl CommonAssets {
    pub fn common_text_style(&self) -> TextFont {
        TextFont {
            font: FontSource::Handle(self.common_font.clone()),
            font_size: FontSize::Px(50.0),
            ..default()
        }
    }
}

impl MazeAssets {
    pub fn wall(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        (
            Mesh3d(self.wall.clone()),
            MeshMaterial3d(self.material.clone()),
        )
    }

    pub fn joint(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        (
            Mesh3d(self.joint.clone()),
            MeshMaterial3d(self.material.clone()),
        )
    }
}
