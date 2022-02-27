use crate::maze;
use bevy::prelude::*;

#[derive(Component)]
pub struct MazeLevel<const DIMS: usize> {
    position: [usize; DIMS],
    maze: maze::Maze<DIMS>,
    dim_x: usize,
    dim_y: usize,
    axis: FocusedAxis,
}

pub enum FocusedAxis {
    X,
    Y,
}

pub enum Direction {
    Positive,
    Negative,
}

impl<const DIMS: usize> Default for MazeLevel<DIMS> {
    fn default() -> Self {
        Self {
            maze: Default::default(),
            dim_x: 0,
            dim_y: 1,
            axis: FocusedAxis::X,
            position: [0; DIMS],
        }
    }
}

impl<const DIMS: usize> MazeLevel<DIMS> {
    pub fn new(lengths: &[usize; DIMS], rng: &mut impl rand::Rng) -> Self {
        Self {
            maze: crate::maze::Maze::new(lengths, rng),
            ..Default::default()
        }
    }

    pub fn flip_axis(&mut self) {
        match self.axis {
            FocusedAxis::X => self.axis = FocusedAxis::Y,
            FocusedAxis::Y => self.axis = FocusedAxis::X,
        }
    }

    fn axis_current(&self) -> usize {
        match self.axis {
            FocusedAxis::X => self.dim_x,
            FocusedAxis::Y => self.dim_y,
        }
    }

    pub fn off_axis_shift(&mut self, dir: Direction) {
        let current = match self.axis {
            FocusedAxis::X => self.dim_y,
            FocusedAxis::Y => self.dim_x,
        };

        let linear_current = if current > self.axis_current() {
            current - 1
        } else {
            current
        };

        let new_off_axis = match dir {
            Direction::Positive => linear_current.checked_add(1).unwrap_or(0),
            Direction::Negative => linear_current.checked_sub(1).unwrap_or(self.dims() - 2),
        } % (self.dims() - 1);
        let dest = if new_off_axis >= self.axis_current() {
            new_off_axis + 1
        } else {
            new_off_axis
        };

        match self.axis {
            FocusedAxis::X => self.dim_y = dest,
            FocusedAxis::Y => self.dim_x = dest,
        };
    }

    // assume dim_x and dim_y are both together.
    #[inline]
    pub fn length_x(&self) -> usize {
        self.length_of_dim(self.dim_x).unwrap()
    }

    #[inline]
    pub fn length_y(&self) -> usize {
        self.length_of_dim(self.dim_y).unwrap()
    }

    fn should_make_wall(&self, position: &[usize; DIMS], direction: usize) -> bool {
        if let Some(walkable) = self.maze.can_move(position, direction) {
            !walkable
        } else {
            false
        }
    }

    fn dims(&self) -> usize {
        DIMS
    }

    fn length_of_dim(&self, dim: usize) -> Option<usize> {
        self.maze.lengths().get(dim).copied()
    }

    pub fn iter_walls(&self) -> impl std::iter::Iterator<Item = ([usize; 2], [usize; 2])> + '_ {
        let length_x = self.length_x();
        let length_y = self.length_y();
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
