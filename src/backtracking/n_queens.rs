//! This module provides functionality to solve the N-Queens problem.
//!
//! The N-Queens problem is a classic chessboard puzzle where the goal is to
//! place N queens on an NxN chessboard so that no two queens threaten each
//! other. Queens can attack each other if they share the same row, column, or
//! diagonal.
//!
//! This implementation solves the N-Queens problem using a backtracking algorithm.
//! It starts with an empty chessboard and iteratively tries to place queens in
//! different rows, ensuring they do not conflict with each other. If a valid
//! solution is found, it's added to the list of solutions.

/// Solves the N-Queens problem for a given size and returns a vector of solutions.
///
/// # Arguments
///
/// * `n` - The size of the chessboard (NxN).
///
/// # Returns
///
/// A vector containing all solutions to the N-Queens problem.
pub fn n_queens_solver(n: usize) -> Vec<Vec<String>> {
    let mut solver = NQueensSolver::new(n);
    solver.solve()
}

/// Represents a solver for the N-Queens problem.
struct NQueensSolver {
    size: usize,
    board: Vec<Vec<char>>,
    solutions: Vec<Vec<String>>,
}

impl NQueensSolver {
    /// Creates a new `NQueensSolver` instance with the given size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the chessboard (N×N).
    ///
    /// # Returns
    ///
    /// A new `NQueensSolver` instance.
    fn new(size: usize) -> Self {
        let board = vec![vec!['.'; size]; size];
        let solutions = Vec::new();
        NQueensSolver {
            size,
            board,
            solutions,
        }
    }

    /// Solves the N-Queens problem and returns a vector of solutions.
    ///
    /// # Returns
    ///
    /// A vector containing all solutions to the N-Queens problem.
    fn solve(&mut self) -> Vec<Vec<String>> {
        self.solve_helper(0);
        self.solutions.clone()
    }

    /// Checks if it's safe to place a queen at the specified position (row, col).
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the position to check.
    /// * `col` - The column index of the position to check.
    ///
    /// # Returns
    ///
    /// `true` if it's safe to place a queen at the specified position, `false` otherwise.
    fn is_safe(&self, row: usize, col: usize) -> bool {
        // Check if there is a queen in the same column
        for (i, _) in self.board.iter().take(row).enumerate() {
            if self.board[i][col] == 'Q' {
                return false;
            }
        }

        // Check if there is a queen in the left upper diagonal
        for i in (0..row).rev() {
            let j = col as isize - (row as isize - i as isize);
            if j >= 0 && self.board[i][j as usize] == 'Q' {
                return false;
            }
        }

        // Check if there is a queen in the right upper diagonal
        for i in (0..row).rev() {
            let j = col + row - i;
            if j < self.size && self.board[i][j] == 'Q' {
                return false;
            }
        }

        true
    }

    /// Recursive helper function to solve the N-Queens problem.
    ///
    /// # Arguments
    ///
    /// * `row` - The current row being processed.
    fn solve_helper(&mut self, row: usize) {
        if row == self.size {
            let solution: Vec<String> = self.board.iter().map(|row| row.iter().collect()).collect();
            self.solutions.push(solution);
            return;
        }

        for col in 0..self.size {
            if self.is_safe(row, col) {
                self.board[row][col] = 'Q';
                self.solve_helper(row + 1);
                self.board[row][col] = '.';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_n_queens_solver {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, expected_solutions) = $tc;
                    let solutions = n_queens_solver(n);
                    assert_eq!(solutions.len(), expected_solutions.len());
                    for (actual, expected) in solutions.iter().zip(expected_solutions.iter()) {
                        assert_eq!(actual, expected);
                    }
                }
            )*
        };
    }

    test_n_queens_solver! {
        test_4_queens: (4, vec![
            vec![".Q..", "...Q", "Q...", "..Q."],
            vec!["..Q.", "Q...", "...Q", ".Q.."],
        ]),
        test_6_queens: (6, vec![
            vec![".Q....", "...Q..", ".....Q", "Q.....", "..Q...", "....Q."],
            vec!["..Q...", ".....Q", ".Q....", "....Q.", "Q.....", "...Q.."],
            vec!["...Q..", "Q.....", "....Q.", ".Q....", ".....Q", "..Q..."],
            vec!["....Q.", "..Q...", "Q.....", ".....Q", "...Q..", ".Q...."],
        ]),
    }
}
