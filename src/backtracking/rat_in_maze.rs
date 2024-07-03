//! This module contains the implementation of the Rat in Maze problem.
//!
//! The Rat in Maze problem is a classic algorithmic problem where the
//! objective is to find a path from the starting position to the exit
//! position in a maze.

/// Enum representing various errors that can occur while working with mazes.
#[derive(Debug, PartialEq, Eq)]
pub enum MazeError {
    /// Indicates that the maze is empty (zero rows).
    EmptyMaze,
    /// Indicates that the starting position is out of bounds.
    OutOfBoundPos,
    /// Indicates an improper representation of the maze (e.g., non-rectangular maze).
    ImproperMazeRepr,
}

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
/// A `Result` where:
/// - `Ok(Some(solution))` if a path is found and contains the solution matrix.
/// - `Ok(None)` if no path is found.
/// - `Err(MazeError)` for various error conditions such as out-of-bound start position or improper maze representation.
///
/// # Solution Selection
///
/// The function returns the first successful path it discovers based on the predefined order of moves.
/// The order of moves is defined in the `MOVES` constant of the `Maze` struct.
///
/// The backtracking algorithm explores each direction in this order. If multiple solutions exist,
/// the algorithm returns the first path it finds according to this sequence. It recursively explores
/// each direction, marks valid moves, and backtracks if necessary, ensuring that the solution is found
/// efficiently and consistently.
pub fn find_path_in_maze(
    maze: &[Vec<bool>],
    start_x: usize,
    start_y: usize,
) -> Result<Option<Vec<Vec<bool>>>, MazeError> {
    if maze.is_empty() {
        return Err(MazeError::EmptyMaze);
    }

    // Validate start position
    if start_x >= maze.len() || start_y >= maze[0].len() {
        return Err(MazeError::OutOfBoundPos);
    }

    // Validate maze representation (if necessary)
    if maze.iter().any(|row| row.len() != maze[0].len()) {
        return Err(MazeError::ImproperMazeRepr);
    }

    // If validations pass, proceed with finding the path
    let mut maze_instance = Maze::new(maze.to_owned());
    Ok(maze_instance.find_path(start_x, start_y))
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
            && self.maze[x as usize][y as usize]
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
                    if let Ok(Some(expected_solution)) = &solution {
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
        ], Ok(Some(vec![
            vec![true, false, false, false, false],
            vec![true, true, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, true, true],
            vec![false, false, false, false, true],
        ])),
        maze_with_solution_6x6: 0, 0, &[
            vec![true, false, true, false, true, false],
            vec![true, true, false, true, false, true],
            vec![false, true, true, true, true, false],
            vec![false, false, false, true, true, true],
            vec![false, true, false, false, true, false],
            vec![true, true, true, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false, false, false],
            vec![true, true, false, false, false, false],
            vec![false, true, true, true, true, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, false, true, false],
            vec![false, false, false, false, true, true],
        ])),
        maze_with_solution_8x8: 0, 0, &[
            vec![true, false, false, false, false, false, false, true],
            vec![true, true, false, true, true, true, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, false, true, false, true, true, false],
            vec![false, true, false, true, true, true, false, true],
            vec![true, false, true, false, false, true, true, true],
            vec![false, false, true, true, true, false, true, true],
            vec![true, true, true, false, true, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false, false, false, false, false],
            vec![true, true, false, false, false, false, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, false, true, false, false, false, false],
            vec![false, false, false, true, true, true, false, false],
            vec![false, false, false, false, false, true, true, true],
            vec![false, false, false, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, true],
        ])),
        maze_without_solution_4x4: 0, 0, &[
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, false, true, false],
            vec![false, false, false, true],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_solution_3x4: 0, 0, &[
            vec![true, false, true, true],
            vec![true, true, true, false],
            vec![false, true, true, true],
        ], Ok(Some(vec![
            vec![true, false, false, false],
            vec![true, true, true, false],
            vec![false, false, true, true],
        ])),
        maze_without_solution_3x4: 0, 0, &[
            vec![true, false, true, true],
            vec![true, false, true, false],
            vec![false, true, false, true],
        ], Ok(None::<Vec<Vec<bool>>>),
        improper_maze_representation: 0, 0, &[
            vec![true],
            vec![true, true],
            vec![true, true, true],
            vec![true, true, true, true]
        ], Err(MazeError::ImproperMazeRepr),
        out_of_bound_start: 0, 3, &[
            vec![true, false, true],
            vec![true, true],
            vec![false, true, true],
        ], Err(MazeError::OutOfBoundPos),
        empty_maze: 0, 0, &[], Err(MazeError::EmptyMaze),
        maze_with_single_cell: 0, 0, &[
            vec![true],
        ], Ok(Some(vec![
                vec![true]
        ])),
        maze_with_one_row_and_multiple_columns: 0, 0, &[
            vec![true, false, true, true, false]
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_multiple_rows_and_one_column: 0, 0, &[
            vec![true],
            vec![true],
            vec![false],
            vec![true],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_walls_surrounding_border: 0, 0, &[
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ], Ok(None::<Vec<Vec<bool>>>),
        maze_with_no_walls: 0, 0, &[
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ], Ok(Some(vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
        ])),
        maze_with_going_back: 0, 0, &[
            vec![true,  true,  true,  true, true,   true],
            vec![false, false, false, true, false,  true],
            vec![true,  true,  true,  true,  false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, false, false, true, true],
            vec![true,  false, true,  true,  true,  false],
            vec![true,  false, true , false, true,  false],
            vec![true,  true,  true,  false, true,  true],
        ], Ok(Some(vec![
            vec![true,  true,  true,  true, false,  false],
            vec![false, false, false, true, false,  false],
            vec![true,  true,  true,  true,  false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, false, false, false, false],
            vec![true,  false, true,  true,  true,  false],
            vec![true,  false, true , false, true,  false],
            vec![true,  true,  true,  false, true,  true],
        ])),
    }
}
