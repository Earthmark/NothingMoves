use crate::maze;
use bevy::prelude::*;

#[derive(Component)]
pub struct MazeLevel<const DIMS: usize> {
    position: [u8; DIMS],
    maze: maze::Maze<DIMS>,
    dim_x: usize,
    dim_y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    fn invert(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Positive,
    Negative,
}

impl Direction {
    fn shift_wrapped(&self, value: usize, limit: usize) -> usize {
        (match self {
            Direction::Positive => value.checked_add(1).unwrap_or(0),
            Direction::Negative => value.checked_sub(1).unwrap_or(limit - 2),
        } % (limit - 1))
    }
}

impl<const DIMS: usize> Default for MazeLevel<DIMS> {
    fn default() -> Self {
        Self {
            maze: Default::default(),
            dim_x: 0,
            dim_y: 1,
            position: [0; DIMS],
        }
    }
}

impl<const DIMS: usize> MazeLevel<DIMS> {
    pub fn new(lengths: &[u8; DIMS], rng: &mut impl rand::Rng) -> Self {
        Self {
            maze: crate::maze::Maze::new(lengths, rng),
            ..Default::default()
        }
    }

    fn axis(&self, axis: Axis) -> usize {
        match axis {
            Axis::X => self.dim_x,
            Axis::Y => self.dim_y,
        }
    }

    fn axis_mut(&mut self, axis: Axis) -> &mut usize {
        match axis {
            Axis::X => &mut self.dim_x,
            Axis::Y => &mut self.dim_y,
        }
    }

    pub fn shift_axis(&mut self, axis: Axis, dir: Direction) {
        let target_axis = self.axis(axis);
        let off_target_axis = self.axis(axis.invert());

        let linear_current = if target_axis > off_target_axis {
            target_axis - 1
        } else {
            target_axis
        };

        let new_off_axis = dir.shift_wrapped(linear_current, self.dims());
        let dest = if new_off_axis >= off_target_axis {
            new_off_axis + 1
        } else {
            new_off_axis
        };

        *self.axis_mut(axis) = dest;
    }

    // assume dim_x and dim_y are both together.
    #[inline]
    pub fn pos_limit(&self, axis: Axis) -> u8 {
        self.length_of_dim(self.axis(axis)).unwrap()
    }

    pub fn pos(&self, axis: Axis) -> u8 {
        self.position[self.axis(axis)]
    }

    pub fn move_pos(&mut self, axis: Axis, dir: Direction) {
        let dim = self.axis(axis);
        let mut pos = self.position;
        if dir == Direction::Negative {
            if let Some(new_pos) = pos[dim].checked_sub(1) {
                pos[dim] = new_pos;
            } else {
                return;
            }
        }
        if let Some(true) = self.maze.can_move(&pos, dim) {
            if let Some(new_pos) = if dir == Direction::Positive {
                self.position[dim].checked_add(1)
            } else {
                self.position[dim].checked_sub(1)
            } {
                self.position[dim] = new_pos;
            }
        }
    }

    fn should_make_wall(&self, position: &[u8; DIMS], direction: usize) -> bool {
        if let Some(walkable) = self.maze.can_move(position, direction) {
            !walkable
        } else {
            false
        }
    }

    fn dims(&self) -> usize {
        DIMS
    }

    fn length_of_dim(&self, dim: usize) -> Option<u8> {
        self.maze.lengths().get(dim).copied()
    }

    pub fn iter_walls(&self) -> impl std::iter::Iterator<Item = ([u8; 2], [u8; 2])> + '_ {
        let length_x = self.pos_limit(Axis::X);
        let length_y = self.pos_limit(Axis::Y);
        let position = self.position;

        (0..length_x)
            .flat_map(move |x| (0..length_y).map(move |y| (x, y)))
            .flat_map(move |(cursor_x, cursor_y)| {
                let mut cursor = position;
                cursor[self.dim_x] = cursor_x;
                cursor[self.dim_y] = cursor_y;
                [
                    if self.should_make_wall(&cursor, self.dim_x) {
                        Some(([cursor_x, cursor_y], [cursor_x + 1, cursor_y]))
                    } else {
                        None
                    },
                    if self.should_make_wall(&cursor, self.dim_y) {
                        Some(([cursor_x, cursor_y], [cursor_x, cursor_y + 1]))
                    } else {
                        None
                    },
                ]
            })
            .flatten()
    }
}
