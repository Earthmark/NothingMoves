use crate::{
    assets::CommonAssets,
    level::LoadLevel,
    ui::{
        num_box::NumBox,
        visible_in::{visible_in_plugin, VisibleIn},
    },
    AppState,
};
use bevy::{app::AppExit, prelude::*};

pub fn main_menu_plugin(app: &mut App) {
    app.init_state::<MainMenuSelection>()
        .add_plugins(visible_in_plugin::<MainMenuSelection>)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(
            OnExit(AppState::MainMenu),
            (
                remove_all::<MainRootMarker>,
                remove_resource::<CreateMazeInfo>,
            ),
        )
        .add_systems(
            Update,
            (
                exit_action,
                new_action,
                back_to_main_action,
                generate_action,
            ),
        )
        .add_systems(
            Update,
            (
                dimension_max_size_num_box_enforcement,
                dimension_length_visibility,
                reset_invisible,
            )
                .run_if(resource_exists::<CreateMazeInfo>),
        );
}

// Main menu flows:
//
// Start - Pick level kind
//  | - Getting Started
//  | - New Maze (pick dimensions)
//  | | - Pick dimension count, lengths, and seed.
//  | - Load Seed
//    | - Load seed string
// Exit - Close (if not web)

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum MainMenuSelection {
    #[default]
    Main,
    New,
}

#[derive(Component)]
struct GlobalMenuMarker;

#[derive(Component)]
struct MainRootMarker;

fn remove_all<T: Component>(mut c: Commands, q: Query<Entity, With<T>>) {
    for e in &q {
        c.entity(e).despawn_recursive();
    }
}

fn remove_resource<T>(mut c: Commands)
where
    T: Resource,
{
    c.remove_resource::<T>();
}

#[derive(Resource)]
struct CreateMazeInfo {
    max_size: i32,
}

impl Default for CreateMazeInfo {
    fn default() -> Self {
        Self { max_size: 1 << 16 }
    }
}

fn setup_main_menu(
    mut c: Commands,
    mut setter: ResMut<NextState<MainMenuSelection>>,
    assets: Res<CommonAssets>,
) {
    setter.set(MainMenuSelection::Main);
    c.init_resource::<CreateMazeInfo>();

    c.spawn((
        MainRootMarker,
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|c| {
        c.spawn(TextBundle {
            style: Style {
                margin: UiRect::vertical(Val::Px(30.)),
                ..default()
            },
            text: Text::from_section(
                "Nothing Moves",
                TextStyle {
                    font_size: 150.,
                    ..assets.common_text_style()
                },
            ),
            ..default()
        });

        c.spawn((
            NodeBundle::default(),
            VisibleIn::new(MainMenuSelection::Main),
        ))
        .with_children(|c| {
            assets.spawn_common(
                c,
                crate::ui::button::SpawnableButton::primary("Start"),
                NewMarker,
            );
            if !crate::util::is_wasm() {
                assets.spawn_common(
                    c,
                    crate::ui::button::SpawnableButton::normal("Exit"),
                    CancelMarker,
                );
            }
        });

        c.spawn((
            NodeBundle::default(),
            VisibleIn::new(MainMenuSelection::New),
        ))
        .with_children(|c| {
            assets.spawn_common(
                c,
                crate::ui::num_box::SpawnableNumBox::new(2, 6, 2),
                DimensionCountPicker,
            );
            assets.spawn_common(
                c,
                crate::ui::num_box::SpawnableNumBox::new(1, 25, 2),
                DimensionLengthPicker::new(0),
            );
            assets.spawn_common(
                c,
                crate::ui::num_box::SpawnableNumBox::new(1, 25, 2),
                DimensionLengthPicker::new(1),
            );
            assets.spawn_common(
                c,
                crate::ui::num_box::SpawnableNumBox::new(1, 25, 2),
                DimensionLengthPicker::new(2),
            );
            assets.spawn_common(
                c,
                crate::ui::num_box::SpawnableNumBox::new(1, 25, 2),
                DimensionLengthPicker::new(3),
            );
            assets.spawn_common(
                c,
                crate::ui::num_box::SpawnableNumBox::new(1, 25, 2),
                DimensionLengthPicker::new(4),
            );
            assets.spawn_common(
                c,
                crate::ui::num_box::SpawnableNumBox::new(1, 25, 2),
                DimensionLengthPicker::new(5),
            );
            assets.spawn_common(
                c,
                crate::ui::button::SpawnableButton::primary("Generate"),
                GenerateMarker,
            );
            assets.spawn_common(
                c,
                crate::ui::button::SpawnableButton::normal("Back"),
                BackToMainMarker,
            );
        });
    });
}

#[derive(Component)]
struct DimensionCountPicker;

#[derive(Component)]
struct DimensionLengthPicker {
    dimension_index: i32,
}

impl DimensionLengthPicker {
    fn new(dimension_index: i32) -> Self {
        Self { dimension_index }
    }
}

fn dimension_length_visibility(
    dimension_count: Query<&NumBox, With<DimensionCountPicker>>,
    mut dimension_length: Query<(&DimensionLengthPicker, &mut Visibility)>,
) {
    let active_dimension_count = dimension_count.single().get();
    for (dims, mut visibility) in &mut dimension_length {
        *visibility = if dims.dimension_index < active_dimension_count {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn reset_invisible(
    mut dimension_length: Query<(&Visibility, &mut NumBox), With<DimensionLengthPicker>>,
) {
    for (vis, mut num) in &mut dimension_length {
        if let Visibility::Hidden = vis {
            if num.get() != 1 {
                num.set(1);
            }
        }
    }
}

fn dimension_max_size_num_box_enforcement(
    mut dimension_length_updater: Query<&mut NumBox, With<DimensionLengthPicker>>,
    maze_create_info: Res<CreateMazeInfo>,
) {
    let current_size: i32 = dimension_length_updater.iter().map(|n| n.get()).product();
    for mut num in &mut dimension_length_updater {
        let excluding_this = current_size / num.get();
        let possible_space = maze_create_info.max_size / excluding_this;
        let goal_space = possible_space.min(u8::MAX as i32);
        if num.get_max() != goal_space {
            num.set_max(goal_space);
        }
    }
}

#[derive(Component)]
struct CancelMarker;

fn exit_action(
    mut exit: ResMut<Events<AppExit>>,
    q: Query<&Interaction, (Changed<Interaction>, With<CancelMarker>)>,
) {
    for inter in &q {
        if let Interaction::Pressed = inter {
            exit.send(AppExit);
        }
    }
}

#[derive(Component)]
struct NewMarker;

fn new_action(
    mut state: ResMut<NextState<MainMenuSelection>>,
    q: Query<&Interaction, (Changed<Interaction>, With<NewMarker>)>,
) {
    for inter in &q {
        if let Interaction::Pressed = inter {
            state.set(MainMenuSelection::New);
        }
    }
}

#[derive(Component)]
struct BackToMainMarker;

fn back_to_main_action(
    mut state: ResMut<NextState<MainMenuSelection>>,
    q: Query<&Interaction, (Changed<Interaction>, With<BackToMainMarker>)>,
) {
    for inter in &q {
        if let Interaction::Pressed = inter {
            state.set(MainMenuSelection::Main);
        }
    }
}

#[derive(Component)]
struct GenerateMarker;

fn generate_action(
    q: Query<&Interaction, (Changed<Interaction>, With<GenerateMarker>)>,
    dimension_count: Query<&NumBox, With<DimensionCountPicker>>,
    dimension_length: Query<(&NumBox, &DimensionLengthPicker)>,
    mut loader: EventWriter<LoadLevel>,
) {
    for inter in &q {
        if let Interaction::Pressed = inter {
            let mut dimensions = dimension_length.iter().collect::<Vec<_>>();
            dimensions.sort_by_key(|(_, d)| d.dimension_index);
            let dimensions = dimensions
                .iter()
                .map(|(n, _)| n.get() as u8)
                .take(dimension_count.single().get() as usize)
                .collect::<Vec<_>>();

            let load_level_event = LoadLevel::new(&dimensions);
            loader.send(load_level_event);
        }
    }
}
