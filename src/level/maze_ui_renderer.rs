use super::maze_level::{self, Axis, *};
use bevy::prelude::*;

// font: "fonts\\UnicaOne-Regular.ttf"

pub fn spawn_ui<const DIMS: usize>(
    mut commands: Commands,
    query: Query<(Entity, &MazeLevel<DIMS>), Added<MazeLevel<DIMS>>>,
) {
    for (entity, _maze) in query.iter() {
        commands
            .spawn_bundle(TextBundle::default())
            .insert(MazePositionLabel::<2> {
                level: entity,
                text_section_to_dim: [1, 2],
            });
        commands
            .spawn_bundle(TextBundle::default())
            .insert(MazeAxisLabel::<2> {
                level: entity,
                dim: 2,
                axis: maze_level::Axis::X,
            });
    }
}

// Current dimension status text layout:
// Z| |*| | | | |X
//  |1|2|3|4|5|6|
// Q| | |*| | | |E

#[derive(Component)]
pub struct MazeAxisLabel<const DIMS: usize> {
    level: Entity,
    dim: u8,
    axis: Axis,
}

pub fn maze_axis_label_update_listener<const DIMS: usize>(
    mut query: Query<(&MazeAxisLabel<DIMS>, &mut Visibility)>,
    mut axis_changed: EventReader<AxisChanged>,
) {
    for changed in axis_changed.iter() {
        for (label, mut vis) in query.iter_mut() {
            if label.level == changed.level {
                vis.is_visible = *label.axis.get(&changed.axis) == label.dim;
            }
        }
    }
}

// Current position status text
// Position:|[X,|Y,| Z,|W,| Q]
//     Goal:|[3,|4,|12,|3,|23]

#[derive(Component)]
pub struct MazePositionLabel<const DIMS: usize> {
    level: Entity,
    text_section_to_dim: [usize; DIMS],
}

pub fn maze_position_label_update_listener<const DIMS: usize>(
    mut query: Query<(&MazePositionLabel<DIMS>, &mut Text)>,
    mut position_changed: EventReader<PositionChanged<DIMS>>,
) {
    for changed in position_changed.iter() {
        for (label, mut text) in query.iter_mut() {
            if label.level == changed.level {
                for (section_index, dimension) in label.text_section_to_dim.iter().enumerate() {
                    if let Some(section) = text.sections.get_mut(section_index) {
                        section.value = format!("{}", changed.position[*dimension]);
                    }
                }
            }
        }
    }
}
