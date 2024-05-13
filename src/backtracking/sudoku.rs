//! A Rust implementation of Sudoku solver using Backtracking.
//!
//! This module provides functionality to solve Sudoku puzzles using the backtracking algorithm.
//!
//! GeeksForGeeks: [Sudoku Backtracking](https://www.geeksforgeeks.org/sudoku-backtracking-7/)

/// Solves a Sudoku puzzle.
///
/// Given a partially filled Sudoku puzzle represented by a 9x9 grid, this function attempts to
/// solve the puzzle using the backtracking algorithm.
///
/// Returns the solved Sudoku board if a solution exists, or `None` if no solution is found.
pub fn sudoku_solver(board: &[[u8; 9]; 9]) -> Option<[[u8; 9]; 9]> {
    let mut solver = SudokuSolver::new(*board);
    if solver.solve() {
        Some(solver.board)
    } else {
        None
    }
}

/// Represents a Sudoku puzzle solver.
struct SudokuSolver {
    /// The Sudoku board represented by a 9x9 grid.
    board: [[u8; 9]; 9],
}

impl SudokuSolver {
    /// Creates a new Sudoku puzzle solver with the given board.
    fn new(board: [[u8; 9]; 9]) -> SudokuSolver {
        SudokuSolver { board }
    }

    /// Finds an empty cell in the Sudoku board.
    ///
    /// Returns the coordinates of an empty cell `(row, column)` if found, or `None` if all cells are filled.
    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        // Find an empty cell in the board (returns None if all cells are filled)
        for row in 0..9 {
            for column in 0..9 {
                if self.board[row][column] == 0 {
                    return Some((row, column));
                }
            }
        }

        None
    }

    /// Checks whether a given value can be placed in a specific cell according to Sudoku rules.
    ///
    /// Returns `true` if the value can be placed in the cell, otherwise `false`.
    fn is_value_valid(&self, coordinates: (usize, usize), value: u8) -> bool {
        let (row, column) = coordinates;

        // Checks if the value to be added in the board is an acceptable value for the cell
        // Checking through the row
        for current_column in 0..9 {
            if self.board[row][current_column] == value {
                return false;
            }
        }

        // Checking through the column
        for current_row in 0..9 {
            if self.board[current_row][column] == value {
                return false;
            }
        }

        // Checking through the 3x3 block of the cell
        let start_row = row / 3 * 3;
        let start_column = column / 3 * 3;

        for current_row in start_row..start_row + 3 {
            for current_column in start_column..start_column + 3 {
                if self.board[current_row][current_column] == value {
                    return false;
                }
            }
        }

        true
    }

    /// Solves the Sudoku puzzle recursively using backtracking.
    ///
    /// Returns `true` if a solution is found, otherwise `false`.
    fn solve(&mut self) -> bool {
        let empty_cell = self.find_empty_cell();

        if let Some((row, column)) = empty_cell {
            for value in 1..=9 {
                if self.is_value_valid((row, column), value) {
                    self.board[row][column] = value;
                    if self.solve() {
                        return true;
                    }
                    // Backtracking if the board cannot be solved using the current configuration
                    self.board[row][column] = 0;
                }
            }
        } else {
            // If the board is complete
            return true;
        }

        // Returning false if the board cannot be solved using the current configuration
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_sudoku_solver {
        ($($name:ident: $board:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let result = sudoku_solver(&$board);
                    assert_eq!(result, $expected);
                }
            )*
        };
    }

    test_sudoku_solver! {
        test_sudoku_correct: [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ], Some([
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ]),

        test_sudoku_incorrect: [
            [6, 0, 3, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ], None::<[[u8; 9]; 9]>,
    }
}
