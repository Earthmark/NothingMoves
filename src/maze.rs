use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(PartialEq)]
pub enum MazeMoveDir {
    Forward,
    Backward,
}

pub trait Maze<const DIMS: usize> {
    fn can_move(
        &self,
        point: &[usize; DIMS],
        dimension: usize,
        direction: MazeMoveDir,
    ) -> Option<bool>;
}

struct WalkMaze<const DIMS: usize> {
    walks: std::collections::HashSet<([usize; DIMS], [usize; DIMS])>,
    lengths: [usize; DIMS],
}

impl<const DIMS: usize> WalkMaze<DIMS> {
    fn check_pair(&self, a: &[usize; DIMS], b: &[usize; DIMS]) -> Option<bool> {
        for index in 0..DIMS {
            let length = self.lengths[index];
            if a[index] >= length || b[index] >= length {
                return None;
            }
        }

        // Check for either direction because it's cheaper to check twice
        // than store an exponential memory problem.
        return Some(
            self.walks.contains(&(a.clone(), b.clone()))
                || self.walks.contains(&(b.clone(), a.clone())),
        );
    }
}

impl<const DIMS: usize> Maze<DIMS> for WalkMaze<DIMS> {
    fn can_move(
        &self,
        point: &[usize; DIMS],
        dimension: usize,
        direction: MazeMoveDir,
    ) -> Option<bool> {
        let mut target_point = point.clone();
        if let Some(shift_axis) = target_point.get_mut(dimension) {
            if let Some(new_shifted) = match direction {
                MazeMoveDir::Forward => shift_axis.checked_add(1),
                MazeMoveDir::Backward => shift_axis.checked_sub(1),
            } {
                *shift_axis = new_shifted;
                return self.check_pair(point, &target_point);
            }
        }
        return None;
    }
}

struct MazeGenCell {
    id: usize,
    parent: Weak<RefCell<Self>>,
}

type MazeGenCellRef = Rc<RefCell<MazeGenCell>>;

impl MazeGenCell {
    fn new(id: usize) -> MazeGenCellRef {
        Rc::new(RefCell::new(MazeGenCell {
            id,
            parent: Weak::default(),
        }))
    }

    /// Gets the root of the particular cell tree.
    fn get_root(s: &MazeGenCellRef) -> MazeGenCellRef {
        let rc = s.as_ref().borrow();
        if let Some(p) = rc.parent.upgrade() {
            Self::get_root(&p)
        } else {
            s.clone()
        }
    }
    /// Attempts to merge both cells, returning true if they were different trees previously.
    fn try_merge(a: &MazeGenCellRef, b: &MazeGenCellRef) -> bool {
        let ra = Self::get_root(a);
        let rb = Self::get_root(b);
        let pa = ra.borrow();
        if pa.id != rb.borrow().id {
            rb.borrow_mut().parent = Rc::downgrade(&ra);
            true
        } else {
            false
        }
    }
}

fn unwrap_index<const DIMS: usize>(lengths: &[usize; DIMS], index: usize) -> Option<[usize; DIMS]> {
    let mut result = [0; DIMS];
    let mut remaining_index = index;
    for (length, res) in lengths.iter().zip(result.iter_mut()) {
        *res = remaining_index % length;
        remaining_index /= length;
    }
    if remaining_index == 0 {
        Some(result)
    } else {
        None
    }
}

// Generate a maze with the provided number of side lengths.
pub fn generate_maze<const DIMS: usize>(
    lengths: &[usize; DIMS],
    rng: impl rand::Rng,
) -> impl Maze<DIMS> {
    let mut maze = WalkMaze::<DIMS> {
        lengths: lengths.clone(),
        walks: Default::default(),
    };

    let cell_count = lengths.iter().product();

    // Indexed by dimension sums (higher is higher power).
    let mut cells = HashMap::<[usize; DIMS], MazeGenCellRef>::with_capacity(cell_count);
    for index in 0..cell_count {
        let pos = unwrap_index(lengths, index).unwrap();
        cells.insert(pos, MazeGenCell::new(index));
    }

    let mut pending_edges = Vec::with_capacity(cell_count * DIMS);
    for index in 0..cell_count {
        for dim in 0..DIMS {
            pending_edges.push((index, dim))
        }
    }

    while let Some((target_index, dim)) = pending_edges.pop() {
        let a = unwrap_index(lengths, target_index).unwrap();
        if a[dim] == lengths[dim] {
            continue;
        }
        let mut b = a.clone();
        b[dim] += 1;
        if let Some(cell_a) = cells.get(&a) {
            if let Some(cell_b) = cells.get(&b) {
                if MazeGenCell::try_merge(cell_a, cell_b) {
                    maze.walks.insert((a, b));
                }
            }
        }
    }

    maze
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn maze_cell_ref_merge_roots() {
        let c0 = MazeGenCell::new(0);
        let c1 = MazeGenCell::new(1);
        let c2 = MazeGenCell::new(2);

        assert_eq!(MazeGenCell::try_merge(&c0, &c1), true);
        assert_eq!(MazeGenCell::try_merge(&c0, &c1), false);
        assert_eq!(MazeGenCell::try_merge(&c1, &c0), false);

        assert_eq!(MazeGenCell::try_merge(&c1, &c2), true);
        assert_eq!(MazeGenCell::try_merge(&c0, &c2), false);
    }

    #[test]
    fn maze_cell_ref_merge_roots_alternate() {
        let c0 = MazeGenCell::new(0);
        let c1 = MazeGenCell::new(1);
        let c2 = MazeGenCell::new(2);

        assert_eq!(MazeGenCell::try_merge(&c0, &c1), true);
        assert_eq!(MazeGenCell::try_merge(&c0, &c1), false);
        assert_eq!(MazeGenCell::try_merge(&c1, &c0), false);

        assert_eq!(MazeGenCell::try_merge(&c0, &c2), true);
        assert_eq!(MazeGenCell::try_merge(&c1, &c2), false);
    }

    #[test]
    fn unwrap_index_verify() {
        assert_eq!(unwrap_index(&[2], 0), Some([0]));
        assert_eq!(unwrap_index(&[2], 1), Some([1]));
        assert_eq!(unwrap_index(&[2], 2), None);
    }

    #[test]
    fn verify_generates() {
        let rng = StdRng::seed_from_u64(684153987);
        let maze = generate_maze(&[5, 5, 5, 5, 5], rng);

        assert_eq!(
            maze.can_move(&[1, 2, 3214, 2, 2], 2, MazeMoveDir::Forward),
            None
        );
    }

    #[test]
    fn verify_generates_single() {
        let rng = StdRng::seed_from_u64(684153987);
        let maze = generate_maze(&[5, 1, 1], rng);

        assert_eq!(
            maze.can_move(&[0, 0, 0], 0, MazeMoveDir::Forward),
            Some(true)
        );
        assert_eq!(
            maze.can_move(&[1, 0, 0], 0, MazeMoveDir::Forward),
            Some(true)
        );
        assert_eq!(
            maze.can_move(&[2, 0, 0], 0, MazeMoveDir::Forward),
            Some(true)
        );
        assert_eq!(
            maze.can_move(&[3, 0, 0], 0, MazeMoveDir::Forward),
            Some(true)
        );
        assert_eq!(maze.can_move(&[4, 0, 0], 0, MazeMoveDir::Forward), None);

        assert_eq!(maze.can_move(&[0, 0, 0], 0, MazeMoveDir::Backward), None);
        assert_eq!(
            maze.can_move(&[1, 0, 0], 0, MazeMoveDir::Backward),
            Some(true)
        );
        assert_eq!(
            maze.can_move(&[2, 0, 0], 0, MazeMoveDir::Backward),
            Some(true)
        );
        assert_eq!(
            maze.can_move(&[3, 0, 0], 0, MazeMoveDir::Backward),
            Some(true)
        );
        assert_eq!(
            maze.can_move(&[4, 0, 0], 0, MazeMoveDir::Backward),
            Some(true)
        );
    }
}
