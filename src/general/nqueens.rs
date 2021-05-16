/// Checks if given position on board is safe
///
/// This function is part of the nqueens algorithms and checks
/// if given positon on the chess board is safe to place a queen.
///
/// See [nqueens problem](https://de.wikipedia.org/wiki/Damenproblem) for the theoretical background.
///
/// # Arguments
///
/// * `board` - two dimensional Vector of chars
/// * `row` - the row of the position as usize
/// * `col` - the column of the position as usize
///
/// # Returns
///
/// * `bool` - true if posititon safe, false otherwise
///
/// # Panic
///
/// This function won't panic
///
/// # Examples
///
/// There are no examples because this function is called inside the nqueens solver function.
///
pub fn is_safe(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
    for i in 0..col {
        if board[row][i] == 'Q' {
            return false;
        }
    }

    let mut i = row + 1;
    let mut j = col + 1;

    while i > 0 && j > 0 {
        if board[i - 1][j - 1] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }

    i = row + 1;
    j = col + 1;

    while i < board.len() && j > 0 {
        if board[i - 1][j - 1] == 'Q' {
            return false;
        }
        i += 1;
        j -= 1;
    }

    return true;
}

/// Solves the nqueens problem (recursive)
///
/// This function is part of the nqueens algorithms and inserts
/// the queens at safe positions recursively.
///
/// See [nqueens problem](https://de.wikipedia.org/wiki/Damenproblem) for the theoretical background.
///
/// # Arguments
///
/// * `board` - two dimensional Vector of chars
/// * `col` - the column of the position as usize
///
/// # Returns
///
/// * `bool` - true if nqueens was solved, false otherwise
///
/// # Panic
///
/// This function won't panic
///
/// # Examples
///
/// There are no examples because this function is called inside the nqueens solver function.
///
pub fn solve_nq_util(board: &mut Vec<Vec<char>>, col: usize) -> bool {
    if col >= board.len() {
        return true;
    }

    for i in 0..board.len() {
        if is_safe(board, i, col) {
            board[i][col] = 'Q';

            if solve_nq_util(board, col + 1) {
                return true;
            }

            board[i][col] = '-';
        }
    }

    return false;
}

/// nqueens solver function
///
/// This function combines the is_safe and solve_nq_util function and is called
/// to receive the solution for a n x n sized board.
///
/// See [nqueens problem](https://de.wikipedia.org/wiki/Damenproblem) for the theoretical background.
///
/// # Arguments
///
/// * `n` - size of the board and amount of queens
///
/// # Returns
///
/// * `board` - two dimensional Vector of chars
///
/// # Panic
///
/// This function won't panic
///
/// # Examples
///
/// let solved = nqueens(4)
///
pub fn nqueens(n: usize) -> Vec<Vec<char>> {
    let mut board = vec![vec!['-'; n]; n];

    if solve_nq_util(&mut board, 0) == false {
        println!("Solution doesn't exist!");
        return board;
    }

    return board;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        assert_eq!(vec![vec!['Q']], nqueens(1));
        assert_eq!(vec![vec!['-', 'Q'], vec!['Q', '-']], nqueens(2));
        assert_eq!(
            vec![
                vec!['Q', '-', '-'],
                vec!['-', '-', 'Q'],
                vec!['-', 'Q', '-']
            ],
            nqueens(3)
        );
        assert_eq!(
            vec![
                vec!['-', '-', 'Q', '-'],
                vec!['Q', '-', '-', '-'],
                vec!['-', '-', '-', 'Q'],
                vec!['-', 'Q', '-', '-']
            ],
            nqueens(4)
        );
    }

    #[test]
    fn test_is_safe() {
        let mut incomplete_board = vec![
            vec!['-', '-', 'Q', '-'],
            vec!['Q', '-', '-', '-'],
            vec!['-', '-', '-', 'Q'],
            vec!['-', '-', '-', '-'],
        ];

        assert_eq!(true, is_safe(&mut incomplete_board, 3, 1));
        assert_eq!(false, is_safe(&mut incomplete_board, 3, 2));
    }
}
