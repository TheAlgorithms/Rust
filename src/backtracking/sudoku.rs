/*
    A Rust implementation of Sudoku solver using Backtracking.
    GeeksForGeeks: https://www.geeksforgeeks.org/sudoku-backtracking-7/
*/

pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Sudoku {
        Sudoku { board }
    }

    fn find_empty_cell(&self) -> Option<(usize, usize)> {
        // Find a empty cell in the board (returns (-1, -1) if all cells are filled)
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn check(&self, index_tuple: (usize, usize), value: u8) -> bool {
        let (y, x) = index_tuple;

        // checks if the value to be added in the board is an acceptable value for the cell

        // checking through the row
        for i in 0..9 {
            if self.board[i][x] == value {
                return false;
            }
        }
        // checking through the column
        for i in 0..9 {
            if self.board[y][i] == value {
                return false;
            }
        }

        // checking through the 3x3 block of the cell
        let sec_row = y / 3;
        let sec_col = x / 3;

        for i in (sec_row * 3)..(sec_row * 3 + 3) {
            for j in (sec_col * 3)..(sec_col * 3 + 3) {
                if y != i && x != j && self.board[i][j] == value {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        let empty_cell = self.find_empty_cell();

        if let Some((y, x)) = empty_cell {
            for val in 1..10 {
                if self.check((y, x), val) {
                    self.board[y][x] = val;
                    if self.solve() {
                        return true;
                    }
                    // backtracking if the board cannot be solved using current configuration
                    self.board[y][x] = 0
                }
            }
        } else {
            // if the board is complete
            return true;
        }

        // returning false the board cannot be solved using current configuration
        false
    }

    pub fn print_board(&self) {
        // helper function to display board

        let print_3_by_1 = |arr: Vec<u8>, last: bool| {
            let str = arr
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            if last {
                println!("{str}",);
            } else {
                print!("{str} | ",);
            }
        };

        for i in 0..9 {
            if i % 3 == 0 && i != 0 {
                println!("- - - - - - - - - - - - - -")
            }

            print_3_by_1(self.board[i][0..3].to_vec(), false);
            print_3_by_1(self.board[i][3..6].to_vec(), false);
            print_3_by_1(self.board[i][6..9].to_vec(), true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_correct() {
        let board: [[u8; 9]; 9] = [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let board_result = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        let mut sudoku = Sudoku::new(board);
        let is_solved = sudoku.solve();

        assert!(is_solved);
        assert_eq!(sudoku.board, board_result);
    }

    #[test]
    fn test_sudoku_incorrect() {
        let board: [[u8; 9]; 9] = [
            [6, 0, 3, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let mut sudoku = Sudoku::new(board);
        let is_solved = sudoku.solve();

        assert!(!is_solved);
    }
}
