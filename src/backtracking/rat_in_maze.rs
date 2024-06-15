//! This module contains the implementation of the Rat in Maze problem.
//!
//! The Rat in Maze problem is a classic algorithmic problem where the
//! objective is to find a path from the starting position to the exit
//! position in a maze.

/// Finds a path through the maze starting from the specified position.
///
/// # Arguments
///
/// * `maze` - The maze represented as a vector of vectors where each
/// inner vector represents a row in the maze grid.
/// * `start_x` - The x-coordinate of the starting position.
/// * `start_y` - The y-coordinate of the starting position.
///
/// # Returns
///
/// A solution matrix if a path is found or `None` if not found.
/// During the maze traversal process, the rat explores different paths,
/// marking its progress in the solution matrix. The goal is to find a path
/// from the starting position to the exit (end) of the maze. Once the rat
/// successfully reaches the exit, the solution matrix will contain the path
/// taken by the rat, enabling us to reconstruct the solution and visualize the
/// rat's journey through the maze.
pub fn find_path_in_maze(
    maze: &[Vec<bool>],
    start_x: usize,
    start_y: usize,
) -> Option<Vec<Vec<bool>>> {
    let mut maze_instance = Maze::new(maze.to_owned());
    maze_instance.find_path(start_x, start_y)
}

/// Represents a maze.
struct Maze {
    maze: Vec<Vec<bool>>,
}

impl Maze {
    /// Represents possible moves in the maze.
    const MOVES: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    /// Constructs a new Maze instance.
    /// # Arguments
    ///
    /// * `maze` - The maze represented as a vector of vectors where each
    /// inner vector represents a row in the maze grid.
    ///
    /// # Returns
    ///
    /// A new Maze instance.
    fn new(maze: Vec<Vec<bool>>) -> Self {
        Maze { maze }
    }

    /// Returns the width of the maze.
    ///
    /// # Returns
    ///
    /// The width of the maze.
    fn width(&self) -> usize {
        self.maze.iter().map(|row| row.len()).max().unwrap_or(0)
    }

    /// Returns the height of the maze.
    ///
    /// # Returns
    ///
    /// The height of the maze.
    fn height(&self) -> usize {
        self.maze.len()
    }

    /// Finds a path through the maze starting from the specified position.
    ///
    /// # Arguments
    ///
    /// * `start_x` - The x-coordinate of the starting position.
    /// * `start_y` - The y-coordinate of the starting position.
    ///
    /// # Returns
    ///
    /// A solution matrix if a path is found or None if not found.
    fn find_path(&mut self, start_x: usize, start_y: usize) -> Option<Vec<Vec<bool>>> {
        let mut solution = vec![vec![false; self.width()]; self.height()];
        if self.solve(start_x as isize, start_y as isize, &mut solution) {
            Some(solution)
        } else {
            None
        }
    }

    /// Recursively solves the Rat in Maze problem using backtracking.
    ///
    /// # Arguments
    ///
    /// * `x` - The current x-coordinate.
    /// * `y` - The current y-coordinate.
    /// * `solution` - The current solution matrix.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether a solution was found.
    fn solve(&self, x: isize, y: isize, solution: &mut [Vec<bool>]) -> bool {
        if x == (self.height() as isize - 1) && y == (self.width() as isize - 1) {
            solution[x as usize][y as usize] = true;
            return true;
        }

        if self.is_valid(x, y, solution) {
            solution[x as usize][y as usize] = true;

            for &(dx, dy) in &Self::MOVES {
                if self.solve(x + dx, y + dy, solution) {
                    return true;
                }
            }

            // If none of the directions lead to the solution, backtrack
            solution[x as usize][y as usize] = false;
            return false;
        }
        false
    }

    /// Checks if a given position is valid in the maze.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the position.
    /// * `y` - The y-coordinate of the position.
    /// * `solution` - The current solution matrix.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the position is valid.
    fn is_valid(&self, x: isize, y: isize, solution: &[Vec<bool>]) -> bool {
        x >= 0
            && y >= 0
            && x < self.height() as isize
            && y < self.width() as isize
            && self.maze[x as usize].get(y as usize) == Some(&true)
            && !solution[x as usize][y as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_find_path_in_maze {
        ($($name:ident: $start_x:expr, $start_y:expr, $maze:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let solution = find_path_in_maze($maze, $start_x, $start_y);
                    assert_eq!(solution, $expected);
                    if let Some(expected_solution) = &$expected {
                        assert_eq!(expected_solution[$start_x][$start_y], true);
                    }
                }
            )*
        }
    }

    test_find_path_in_maze! {
        maze_with_solution_5x5: 0, 0, &[
            vec![true, false, true, false, false],
            vec![true, true, false, true, false],
            vec![false, true, true, true, false],
            vec![false, false, false, true, true],
            vec![false, true, false, false, true],
        ], Some(vec![
            vec![true, false, false, false, false],
            vec![true, true, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, true, true],
            vec![false, false, false, false, true],
        ]),
        maze_with_solution_6x6: 0, 0, &[
            vec![true, false, true, false, true, false],
            vec![true, true, false, true, false, true],
            vec![false, true, true, true, true, false],
            vec![false, false, false, true, true, true],
            vec![false, true, false, false, true, false],
            vec![true, true, true, true, true, true],
        ], Some(vec![
            vec![true, false, false, false, false, false],
            vec![true, true, false, false, false, false],
            vec![false, true, true, true, true, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, false, true, true],
        ]),
        maze_with_solution_8x8: 0, 0, &[
            vec![true, false, false, false, false, false, false, true],
            vec![true, true, false, true, true, true, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, false, true, false, true, true, false],
            vec![false, true, false, true, true, true, false, true],
            vec![true, false, true, false, false, true, true, true],
            vec![false, false, true, true, true, false, true, true],
            vec![true, true, true, false, true, true, true, true],
        ], Some(vec![
            vec![true, false, false, false, false, false, false, false],
            vec![true, true, false, false, false, false, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, false, true, false, false, false, false],
            vec![false, false, false, true, true, true, false, false],
            vec![false, false, false, false, false, true, true, true],
            vec![false, false, false, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, true],
        ]),
        maze_without_solution_4x4: 0, 0, &[
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, false, true, false],
            vec![false, false, false, true],
        ], None::<Vec<Vec<bool>>>,
        maze_no_solution_possible_loops: 0, 0, &[
            vec![true, true, true, true],
            vec![true, true, true, true],
            vec![true, true, true, false],
            vec![true, true, false, true],
        ], None::<Vec<Vec<bool>>>,
        maze_with_start_outside: 5, 0, &[
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, true, true, false],
            vec![false, false, true, true],
        ], None::<Vec<Vec<bool>>>,
        unproper_maze_with_solution: 0, 0, &[
            vec![true],
            vec![true, true],
            vec![true, true, true],
            vec![true, true, true, true]
        ], Some(vec![
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, true, true, false],
            vec![false, false, true, true]
        ]),
        unproper_maze_no_solution_possible_loops: 0, 0, &[
            vec![true, true, true, true],
            vec![true, true],
            vec![true, true, false],
            vec![false, false, true],
        ], None::<Vec<Vec<bool>>>,
        maze_non_square_with_solution: 0, 0, &[
            vec![true, false, true, true],
            vec![true, true, true, false],
            vec![false, true, true, true],
        ], Some(vec![
            vec![true, false, false, false],
            vec![true, true, true, false],
            vec![false, false, true, true],
        ]),
        maze_non_square_without_solution: 0, 0, &[
            vec![true, false, true, true],
            vec![true, false, true, false],
            vec![false, true, false, true],
        ], None::<Vec<Vec<bool>>>,
    }
}
