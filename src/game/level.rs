use crate::maze;
use crate::screens::AppState;
use bevy::prelude::*;
use rand::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, level_load_system)
        .add_message::<LoadLevel>();
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn invert(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }

    pub fn get<'a, T>(&self, v: &'a [T; 2]) -> &'a T {
        match self {
            Axis::X => &v[0],
            Axis::Y => &v[1],
        }
    }

    pub fn get_mut<'a, T>(&self, v: &'a mut [T; 2]) -> &'a mut T {
        match self {
            Axis::X => &mut v[0],
            Axis::Y => &mut v[1],
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Positive,
    Negative,
}

impl Direction {
    fn shift(&self, val: &mut u8) {
        *val = match self {
            Direction::Positive => val.saturating_add(1),
            Direction::Negative => val.saturating_sub(1),
        };
    }

    fn shift_clamped(&self, value: usize, limit: usize) -> usize {
        match self {
            Direction::Positive => value.saturating_add(1),
            Direction::Negative => value.saturating_sub(1),
        }
        .clamp(0, limit - 1)
    }
}

#[derive(Resource)]
pub struct MazeLevel {
    maze: maze::Maze,
    pos: Box<[u8]>,
    axis: [usize; 2],
}

impl MazeLevel {
    pub fn new(lengths: &[u8], rng: &mut impl Rng) -> Self {
        Self {
            maze: maze::Maze::new(lengths, rng),
            pos: vec![0; lengths.len()].into_boxed_slice(),
            axis: [0, 1],
        }
    }

    pub fn sides(&self) -> &[u8] {
        self.maze.sides()
    }

    pub fn axis(&self) -> [usize; 2] {
        self.axis
    }

    pub fn pos(&self) -> &[u8] {
        &self.pos
    }

    pub fn pos_cur_limit(&self) -> [u8; 2] {
        [
            self.maze.sides()[self.axis[0]],
            self.maze.sides()[self.axis[1]],
        ]
    }

    pub fn pos_other(&self, axis: [usize; 2]) -> [u8; 2] {
        [self.pos[axis[0]], self.pos[axis[1]]]
    }

    pub fn shift_axis(&mut self, axis: Axis, dir: Direction) {
        let target_axis = *axis.get(&self.axis());
        let off_target_axis = *axis.invert().get(&self.axis());

        let linear_current = if target_axis > off_target_axis {
            target_axis - 1
        } else {
            target_axis
        };

        let new_off_axis = dir.shift_clamped(linear_current, self.pos.len() - 1);
        let dest = if new_off_axis >= off_target_axis {
            new_off_axis + 1
        } else {
            new_off_axis
        };

        *axis.get_mut(&mut self.axis) = dest;
    }

    pub fn move_pos(&mut self, axis: Axis, dir: Direction) {
        let dim = *axis.get(&self.axis);
        if let Some(true) = self.can_move(dim, dir) {
            dir.shift(&mut self.pos[dim]);
        }
    }

    pub fn can_move(&self, dim: usize, dir: Direction) -> Option<bool> {
        let mut pos = self.pos.clone();
        if dir == Direction::Negative {
            let new_pos = pos[dim].checked_sub(1)?;
            pos[dim] = new_pos;
        }
        self.maze.can_move(&pos, dim)
    }

    pub fn wall_in_current(&self, position: [u8; 2], axis: Axis) -> bool {
        let mut cursor = self.pos.clone();
        cursor[self.axis[0]] = position[0];
        cursor[self.axis[1]] = position[1];
        if let Some(walkable) = self.maze.can_move(&cursor, *axis.get(&self.axis)) {
            !walkable
        } else {
            false
        }
    }

    pub fn iter_walls(&self) -> impl Iterator<Item = ([u8; 2], [u8; 2])> + '_ {
        let [length_x, length_y] = self.pos_cur_limit();

        (0..length_x)
            .flat_map(move |x| (0..length_y).map(move |y| [x, y]))
            .flat_map(move |cursor| {
                [
                    if self.wall_in_current(cursor, Axis::X) {
                        Some((cursor, [cursor[0] + 1, cursor[1]]))
                    } else {
                        None
                    },
                    if self.wall_in_current(cursor, Axis::Y) {
                        Some((cursor, [cursor[0], cursor[1] + 1]))
                    } else {
                        None
                    },
                ]
            })
            .flatten()
    }
}

#[derive(Clone, Debug, Message)]
pub struct LoadLevel {
    pub rng_source: u64,
    pub dimensions: Box<[u8]>,
}

impl LoadLevel {
    pub fn new(dimensions: &[u8], seed: u64) -> Self {
        Self {
            dimensions: dimensions.into(),
            rng_source: seed,
        }
    }
}

fn level_load_system(
    mut c: Commands,
    mut events: MessageReader<LoadLevel>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for level_loader in events.read() {
        c.insert_resource(MazeLevel::new(
            &level_loader.dimensions,
            &mut StdRng::seed_from_u64(level_loader.rng_source),
        ));
        app_state.set(AppState::InMaze);
    }
}
