use crate::ui::button::{button_normal, button_primary};
use crate::ui::copy_paste_box::{copy_paste_button, CopyPastePicker};
use crate::ui::menu_root;
use crate::ui::num_box::num_box;
use crate::{assets::CommonAssets, game::level::LoadLevel, screens::AppState, ui::num_box::NumBox};
use bevy::{app::AppExit, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_sub_state::<MainMenuSelection>()
        .add_systems(OnEnter(MainMenuSelection::Main), setup_main_menu)
        .add_systems(OnEnter(MainMenuSelection::New), setup_new_menu)
        .add_systems(
            Update,
            (
                dimension_max_size_num_box_enforcement,
                dimension_length_visibility,
                reset_invisible,
            )
                .run_if(in_state(MainMenuSelection::New)),
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

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[source(AppState = AppState::MainMenu)]
enum MainMenuSelection {
    #[default]
    Main,
    New,
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

fn setup_main_menu(mut c: Commands, assets: Res<CommonAssets>) {
    c.init_resource::<CreateMazeInfo>();

    c.spawn((menu_root(), DespawnOnExit(MainMenuSelection::Main)))
        .with_children(|c| {
            c.spawn((
                Node {
                    margin: UiRect::vertical(Val::Px(30.)),
                    ..default()
                },
                Text::new("Nothing Moves"),
                TextFont {
                    font_size: FontSize::Px(150.),
                    ..assets.common_text_style()
                },
            ));

            c.spawn((Node {
                column_gap: Val::Px(8.),
                ..default()
            },))
                .with_children(|c| {
                    c.spawn(button_primary("Start", &assets)).observe(
                        |_: On<Pointer<Click>>, mut menu: ResMut<NextState<MainMenuSelection>>| {
                            menu.set(MainMenuSelection::New);
                        },
                    );
                    if !cfg!(target_arch = "wasm32") {
                        c.spawn(button_primary("Exit", &assets)).observe(
                            |_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>| {
                                exit.write(AppExit::Success);
                            },
                        );
                    }
                });
        });
}

fn setup_new_menu(mut c: Commands, assets: Res<CommonAssets>) {
    c.spawn((menu_root(), DespawnOnExit(MainMenuSelection::New)))
        .with_children(|c| {
            c.spawn((Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },))
                .with_children(|c| {
                    c.spawn((num_box(2, 6, 2, &assets), DimensionCountPicker));
                    c.spawn((num_box(1, 25, 2, &assets), DimensionLengthPicker::new(0)));
                    c.spawn((num_box(1, 25, 2, &assets), DimensionLengthPicker::new(1)));
                    c.spawn((num_box(1, 25, 2, &assets), DimensionLengthPicker::new(2)));
                    c.spawn((num_box(1, 25, 2, &assets), DimensionLengthPicker::new(3)));
                    c.spawn((num_box(1, 25, 2, &assets), DimensionLengthPicker::new(4)));
                    c.spawn((num_box(1, 25, 2, &assets), DimensionLengthPicker::new(5)));
                    c.spawn((copy_paste_button(&assets), RngPicker));
                    c.spawn(button_primary("Generate", &assets))
                        .observe(generate_action);
                    c.spawn(button_normal("Back", &assets)).observe(
                        |_: On<Pointer<Click>>, mut state: ResMut<NextState<MainMenuSelection>>| {
                            state.set(MainMenuSelection::Main);
                        },
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

#[derive(Component)]
struct RngPicker;

impl DimensionLengthPicker {
    fn new(dimension_index: i32) -> Self {
        Self { dimension_index }
    }
}

fn dimension_length_visibility(
    dimension_count: Query<&NumBox, With<DimensionCountPicker>>,
    mut dimension_length: Query<(&DimensionLengthPicker, &mut Node)>,
) {
    let Ok(active_dimension_count) = dimension_count.single() else {
        return;
    };
    for (dims, mut node) in &mut dimension_length {
        node.display = if dims.dimension_index < active_dimension_count.get() {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn reset_invisible(mut dimension_length: Query<(&Node, &mut NumBox), With<DimensionLengthPicker>>) {
    for (node, mut num) in &mut dimension_length {
        if let Display::None = node.display {
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

fn generate_action(
    _: On<Pointer<Click>>,
    dimension_count: Query<&NumBox, With<DimensionCountPicker>>,
    dimension_length: Query<(&NumBox, &DimensionLengthPicker)>,
    seed: Query<&CopyPastePicker, With<RngPicker>>,
    mut loader: MessageWriter<LoadLevel>,
) {
    let Ok(dim_count) = dimension_count.single() else {
        return;
    };
    let mut dimensions = dimension_length.iter().collect::<Vec<_>>();
    dimensions.sort_by_key(|(_, d)| d.dimension_index);
    let dimensions = dimensions
        .iter()
        .map(|(n, _)| n.get() as u8)
        .take(dim_count.get() as usize)
        .collect::<Vec<_>>();

    let Ok(seed) = seed.single() else {
        return;
    };

    let load_level_event = LoadLevel::new(&dimensions, seed.val());
    loader.write(load_level_event);
}
