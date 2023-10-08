/*
    The N-Queens problem is a classic chessboard puzzle where the goal is to
    place N queens on an N×N chessboard so that no two queens threaten each
    other. Queens can attack each other if they share the same row, column, or
    diagonal.

    We solve this problem using a backtracking algorithm. We start with an empty
    chessboard and iteratively try to place queens in different rows, ensuring
    they do not conflict with each other. If a valid solution is found, it's added
    to the list of solutions.

    Time Complexity: O(N!), where N is the size of the chessboard.

    nqueens_solver(4) => Returns two solutions:
    Solution 1:
    [
        ".Q..",
        "...Q",
        "Q...",
        "..Q."
    ]

    Solution 2:
    [
        "..Q.",
        "Q...",
        "...Q",
        ".Q.."
    ]
*/

pub fn n_queens_solver(n: usize) -> Vec<Vec<String>> {
    let mut board = vec![vec!['.'; n]; n];
    let mut solutions = Vec::new();
    solve(&mut board, 0, &mut solutions);
    solutions
}

fn is_safe(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // Check if there is a queen in the same column
    for (i, _) in board.iter().take(row).enumerate() {
        if board[i][col] == 'Q' {
            return false;
        }
    }

    // Check if there is a queen in the left upper diagonal
    for i in (0..row).rev() {
        let j = col as isize - (row as isize - i as isize);
        if j >= 0 && board[i][j as usize] == 'Q' {
            return false;
        }
    }

    // Check if there is a queen in the right upper diagonal
    for i in (0..row).rev() {
        let j = col + row - i;
        if j < board.len() && board[i][j] == 'Q' {
            return false;
        }
    }

    true
}

fn solve(board: &mut Vec<Vec<char>>, row: usize, solutions: &mut Vec<Vec<String>>) {
    let n = board.len();
    if row == n {
        let solution: Vec<String> = board.iter().map(|row| row.iter().collect()).collect();
        solutions.push(solution);
        return;
    }

    for col in 0..n {
        if is_safe(board, row, col) {
            board[row][col] = 'Q';
            solve(board, row + 1, solutions);
            board[row][col] = '.';
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_queens_solver() {
        // Test case: Solve the 4-Queens problem
        let solutions = n_queens_solver(4);

        assert_eq!(solutions.len(), 2); // There are two solutions for the 4-Queens problem

        // Verify the first solution
        assert_eq!(solutions[0], vec![".Q..", "...Q", "Q...", "..Q."]);

        // Verify the second solution
        assert_eq!(solutions[1], vec!["..Q.", "Q...", "...Q", ".Q.."]);
    }
}
