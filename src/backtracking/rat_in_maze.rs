//! This module contains the implementation of the Rat in Maze problem.
//!
//! The Rat in Maze problem is a classic algorithmic problem where the
//! objective is to find a path from the starting position to the exit
//! position in a maze.

/// Finds a path through the maze starting from the specified position.
///
/// # Arguments
///
/// * `maze` - The maze represented as a 2D array where 1 represents a
/// valid path and 0 represents a wall.
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
    maze: &[Vec<usize>],
    start_x: usize,
    start_y: usize,
) -> Option<Vec<Vec<usize>>> {
    let mut maze_instance = Maze::new(maze.to_owned());
    maze_instance.find_path(start_x, start_y)
}

/// Represents a maze.
struct Maze {
    maze: Vec<Vec<usize>>,
}

impl Maze {
    /// Represents possible moves in the maze.
    const MOVES: [(isize, isize); 4] = [
        (0, 1),  // Move right
        (1, 0),  // Move down
        (0, -1), // Move left
        (-1, 0), // Move up
    ];

    /// Constructs a new Maze instance.
    /// # Arguments
    ///
    /// * `maze` - The maze represented as a 2D array where 1 represents a valid path and 0 represents a wall.
    ///
    /// # Returns
    ///
    /// A new Maze instance.
    fn new(maze: Vec<Vec<usize>>) -> Self {
        Maze { maze }
    }

    /// Returns the width of the maze.
    ///
    /// # Returns
    ///
    /// The width of the maze.
    fn width(&self) -> usize {
        self.maze[0].len()
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
    fn find_path(&mut self, start_x: usize, start_y: usize) -> Option<Vec<Vec<usize>>> {
        let mut solution = vec![vec![0; self.width()]; self.height()];
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
    fn solve(&self, x: isize, y: isize, solution: &mut [Vec<usize>]) -> bool {
        if x == (self.height() as isize - 1) && y == (self.width() as isize - 1) {
            solution[x as usize][y as usize] = 1;
            return true;
        }

        if self.is_valid(x, y, solution) {
            solution[x as usize][y as usize] = 1;

            for &(dx, dy) in &Self::MOVES {
                if self.solve(x + dx, y + dy, solution) {
                    return true;
                }
            }

            // If none of the directions lead to the solution, backtrack
            solution[x as usize][y as usize] = 0;
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
    fn is_valid(&self, x: isize, y: isize, solution: &[Vec<usize>]) -> bool {
        x >= 0
            && y >= 0
            && x < self.height() as isize
            && y < self.width() as isize
            && self.maze[x as usize][y as usize] == 1
            && solution[x as usize][y as usize] == 0
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
                        assert_eq!(expected_solution[$start_x][$start_y], 1);
                    }
                }
            )*
        }
    }

    test_find_path_in_maze! {
        test_find_path_in_maze_with_solution_5x5: 0, 0, &[
            vec![1, 0, 1, 0, 0],
            vec![1, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 1, 0, 0, 1],
        ], Some(vec![
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 1],
        ]),
        test_find_path_in_maze_with_solution_6x6: 0, 0, &[
            vec![1, 0, 1, 0, 1, 0],
            vec![1, 1, 0, 1, 0, 1],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 1, 1],
            vec![0, 1, 0, 0, 1, 0],
            vec![1, 1, 1, 1, 1, 1],
        ], Some(vec![
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 1],
        ]),
        test_find_path_in_maze_with_solution_8x8: 0, 0, &[
            vec![1, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 1, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 0, 1, 1, 1],
            vec![0, 0, 1, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 0, 1, 1, 1, 1],
        ], Some(vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
        ]),
        test_find_path_in_maze_without_solution_4x4: 0, 0, &[
            vec![1, 0, 0, 0],
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
        ], None::<Vec<Vec<usize>>>,
    }
}
