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

pub fn solve_nq_util(board: &mut Vec<Vec<char>>, col: usize)-> bool {

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
        assert_eq!(vec![vec!['-', '-', 'Q', '-'], vec!['Q', '-', '-', '-'], vec!['-', '-', '-', 'Q'], vec!['-', 'Q', '-', '-']], nqueens(4))
    }
}
