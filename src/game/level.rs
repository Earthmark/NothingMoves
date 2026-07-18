use crate::maze;
use crate::screens::AppState;
use bevy::prelude::*;
use rand::prelude::*;
use std::ops::{Deref, DerefMut};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, level_load_system)
        .add_message::<LoadLevel>();
}

struct MazeImpl<const DIMS: usize> {
    maze: maze::Maze<DIMS>,
    position: [u8; DIMS],
    axis: [u8; 2],
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
    /*
    fn shift_wrapped(&self, value: u8, limit: u8) -> u8 {
        (match self {
            Direction::Positive => value.checked_add(1).unwrap_or(0),
            Direction::Negative => value.checked_sub(1).unwrap_or(limit - 1),
        } % limit)
    }
     */

    fn shift_clamped(&self, value: u8, limit: u8) -> u8 {
        match self {
            Direction::Positive => value.checked_add(1).unwrap_or(limit - 1),
            Direction::Negative => value.saturating_sub(1),
        }
        .clamp(0, limit - 1)
    }
}

impl<const DIMS: usize> Default for MazeImpl<DIMS> {
    fn default() -> Self {
        Self {
            maze: Default::default(),
            axis: [0, 1],
            position: [0; DIMS],
        }
    }
}

impl<const DIMS: usize> MazeImpl<DIMS> {
    pub fn new(lengths: &[u8; DIMS], rng: &mut impl rand::Rng) -> Self {
        Self {
            maze: crate::maze::Maze::new(lengths, rng),
            axis: [0, 1],
            position: [0; DIMS],
        }
    }
}

impl<const DIMS: usize> MazeView for MazeImpl<DIMS> {
    fn axis(&self) -> [u8; 2] {
        self.axis
    }

    fn shift_axis(&mut self, axis: Axis, dir: Direction) {
        let target_axis = *axis.get(&self.axis());
        let off_target_axis = *axis.invert().get(&self.axis());

        let linear_current = if target_axis > off_target_axis {
            target_axis - 1
        } else {
            target_axis
        };

        let new_off_axis = dir.shift_clamped(linear_current, DIMS as u8 - 1);
        let dest = if new_off_axis >= off_target_axis {
            new_off_axis + 1
        } else {
            new_off_axis
        };

        *axis.get_mut(&mut self.axis) = dest;
    }

    fn dims_limit(&self) -> &[u8] {
        self.maze.lengths()
    }

    fn dims(&self) -> &[u8] {
        &self.position
    }

    // assume dim_x and dim_y are both together.
    fn pos_limit(&self) -> [u8; 2] {
        [
            self.maze.lengths()[self.axis[0] as usize],
            self.maze.lengths()[self.axis[1] as usize],
        ]
    }

    fn pos(&self) -> [u8; 2] {
        [
            self.position[self.axis[0] as usize],
            self.position[self.axis[1] as usize],
        ]
    }

    fn pos_in(&self, axis: [u8; 2]) -> [u8; 2] {
        [
            self.position[axis[0] as usize],
            self.position[axis[1] as usize],
        ]
    }

    fn move_pos(&mut self, axis: Axis, dir: Direction) {
        let dim = *axis.get(&self.axis) as usize;
        if let Some(true) = self.can_move(dim as u8, dir) {
            if let Some(new_pos) = if dir == Direction::Positive {
                self.position[dim].checked_add(1)
            } else {
                self.position[dim].checked_sub(1)
            } {
                self.position[dim] = new_pos;
            }
        }
    }

    fn can_move(&self, dim: u8, dir: Direction) -> Option<bool> {
        let dim = dim as usize;
        let mut pos = self.position;
        if dir == Direction::Negative {
            let new_pos = pos[dim].checked_sub(1)?;
            pos[dim] = new_pos;
        }
        self.maze.can_move(&pos, dim)
    }

    fn wall_in_current(&self, position: [u8; 2], axis: Axis) -> bool {
        let mut cursor = self.position;
        cursor[self.axis[0] as usize] = position[0];
        cursor[self.axis[1] as usize] = position[1];
        if let Some(walkable) = self.maze.can_move(&cursor, *axis.get(&self.axis) as usize) {
            !walkable
        } else {
            false
        }
    }
}

pub trait MazeView: Sync + Send {
    fn axis(&self) -> [u8; 2];
    fn shift_axis(&mut self, axis: Axis, dir: Direction);

    fn dims_limit(&self) -> &[u8];
    fn dims(&self) -> &[u8];
    fn pos_limit(&self) -> [u8; 2];
    fn pos(&self) -> [u8; 2];
    fn pos_in(&self, axis: [u8; 2]) -> [u8; 2];
    fn move_pos(&mut self, axis: Axis, dir: Direction);

    fn can_move(&self, dim: u8, dir: Direction) -> Option<bool>;

    fn wall_in_current(&self, position: [u8; 2], axis: Axis) -> bool;
}

#[derive(Resource)]
pub struct MazeLevel {
    inner: Box<dyn MazeView>,
}

impl Default for MazeLevel {
    fn default() -> Self {
        Self {
            inner: Box::<MazeImpl<2>>::default(),
        }
    }
}

impl MazeLevel {
    pub fn new<const DIMS: usize>(lengths: &[u8; DIMS], rng: &mut impl rand::Rng) -> Self {
        Self {
            inner: Box::new(MazeImpl::new(lengths, rng)),
        }
    }
}

impl Deref for MazeLevel {
    type Target = dyn MazeView;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl DerefMut for MazeLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

impl MazeLevel {
    pub fn iter_walls(&self) -> impl std::iter::Iterator<Item = ([u8; 2], [u8; 2])> + '_ {
        let [length_x, length_y] = self.pos_limit();

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
    pub rng_source: RngSource,
    pub dimensions: DimensionLength,
}

#[derive(Clone, Debug)]
pub enum RngSource {
    Seeded(u64),
}

// Remove this once construction methods for dimensions are found.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum DimensionLength {
    Two([u8; 2]),
    Three([u8; 3]),
    Four([u8; 4]),
    Five([u8; 5]),
    Six([u8; 6]),
}

impl Default for LoadLevel {
    fn default() -> Self {
        Self {
            rng_source: RngSource::Seeded(rand::random()),
            dimensions: DimensionLength::Two([2, 2]),
        }
    }
}

impl LoadLevel {
    pub fn new(d: &[u8], seed: u64) -> Self {
        let dimensions = match d.len() {
            2 => DimensionLength::Two([d[0], d[1]]),
            3 => DimensionLength::Three([d[0], d[1], d[2]]),
            4 => DimensionLength::Four([d[0], d[1], d[2], d[3]]),
            5 => DimensionLength::Five([d[0], d[1], d[2], d[3], d[4]]),
            6 => DimensionLength::Six([d[0], d[1], d[2], d[3], d[4], d[5]]),
            _ => panic!("Unexpected dimension length, it must be 2-6."),
        };
        Self {
            dimensions,
            rng_source: RngSource::Seeded(seed),
        }
    }
}

fn level_load_system(
    mut c: Commands,
    mut events: MessageReader<LoadLevel>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for level_loader in events.read() {
        let mut rng = match level_loader.rng_source {
            RngSource::Seeded(seed) => StdRng::seed_from_u64(seed),
        };
        c.insert_resource(match level_loader.dimensions {
            DimensionLength::Two(lengths) => MazeLevel::new(&lengths, &mut rng),
            DimensionLength::Three(lengths) => MazeLevel::new(&lengths, &mut rng),
            DimensionLength::Four(lengths) => MazeLevel::new(&lengths, &mut rng),
            DimensionLength::Five(lengths) => MazeLevel::new(&lengths, &mut rng),
            DimensionLength::Six(lengths) => MazeLevel::new(&lengths, &mut rng),
        });
        app_state.set(AppState::InMaze);
    }
}
