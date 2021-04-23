const N: usize = 4;

pub fn print_solution(board: &mut [[char; N]; N]) {

    for i in 0..N {
        for j in 0..N {
            print!("{}  ", board[i][j]);
        }
        print!("\n");
    }
}

pub fn is_safe(board: &mut [[char; N]; N], row: usize, col: usize) -> bool {

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

    while i < N && j > 0 {
        if board[i - 1][j - 1] == 'Q' {
            return false;
        }
        i += 1;
        j -= 1;
    }

    return true;
}

pub fn solve_nq_util(board: &mut [[char; N]; N], col: usize)-> bool {

    if col >= N {
        return true;
    }

    for i in 0..N {

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

pub fn nqueens() -> bool{

    let mut board = [['-'; N]; N];

    if solve_nq_util(&mut board, 0) == false {
        println!("Solution doesn't exist!");
        return false;
    }

    print_solution(&mut board);
    return true;
}
