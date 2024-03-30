//! This module contains the implementation of the Knight's Tour problem.
//!
//! The Knight's Tour is a classic chess problem where the objective is to move a knight to every square on a chessboard exactly once.

/// Finds the Knight's Tour starting from the specified position.
///
/// # Arguments
///
/// * `size_x` - The width of the chessboard.
/// * `size_y` - The height of the chessboard.
/// * `start_x` - The x-coordinate of the starting position.
/// * `start_y` - The y-coordinate of the starting position.
///
/// # Returns
///
/// A tour matrix if the tour was found or None if not found.
/// The tour matrix returned is essentially the board field of the `KnightTour`
/// struct `Vec<Vec<i32>>`. It represents the sequence of moves made by the
/// knight on the chessboard, with each cell containing the order in which the knight visited that square.
pub fn find_knight_tour(
    size_x: usize,
    size_y: usize,
    start_x: usize,
    start_y: usize,
) -> Option<Vec<Vec<usize>>> {
    let mut tour = KnightTour::new(size_x, size_y);
    tour.find_tour(start_x, start_y)
}

/// Represents the KnightTour struct which implements the Knight's Tour problem.
struct KnightTour {
    board_size_x: usize,
    board_size_y: usize,
    board: Vec<Vec<usize>>,
    moves: [(i32, i32); 8],
}

impl KnightTour {
    /// Constructs a new KnightTour instance with the given board size.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the chessboard.
    ///
    /// # Returns
    ///
    /// A new KnightTour instance.
    fn new(size_x: usize, size_y: usize) -> Self {
        let board = vec![vec![0; size_x]; size_y];
        let moves = [
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
        ];
        KnightTour {
            board_size_x: size_x,
            board_size_y: size_y,
            board,
            moves,
        }
    }

    /// Checks if the given position is safe to move to.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the position.
    /// * `y` - The y-coordinate of the position.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the position is safe to move to.
    fn is_safe(&self, x: i32, y: i32) -> bool {
        x >= 0
            && y >= 0
            && x < self.board_size_x as i32
            && y < self.board_size_y as i32
            && self.board[x as usize][y as usize] == 0
    }

    /// Recursively solves the Knight's Tour problem.
    ///
    /// # Arguments
    ///
    /// * `x` - The current x-coordinate of the knight.
    /// * `y` - The current y-coordinate of the knight.
    /// * `move_count` - The current move count.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether a solution was found.
    fn solve_tour(&mut self, x: i32, y: i32, move_count: i32) -> bool {
        if move_count == (self.board_size_x * self.board_size_y) as i32 {
            return true;
        }

        for i in 0..8 {
            let next_x = x + self.moves[i].0;
            let next_y = y + self.moves[i].1;

            if self.is_safe(next_x, next_y) {
                self.board[next_x as usize][next_y as usize] = (move_count as usize) + 1;

                if self.solve_tour(next_x, next_y, move_count + 1) {
                    return true;
                }
                // Backtrack
                self.board[next_x as usize][next_y as usize] = 0;
            }
        }

        false
    }

    /// Finds the Knight's Tour starting from the specified position.
    ///
    /// # Arguments
    ///
    /// * `start_x` - The x-coordinate of the starting position.
    /// * `start_y` - The y-coordinate of the starting position.
    ///
    /// # Returns
    ///
    /// A tour matrix if the tour was found or None if not found.
    fn find_tour(&mut self, start_x: usize, start_y: usize) -> Option<Vec<Vec<usize>>> {
        if !self.is_safe(start_x as i32, start_y as i32) {
            return None;
        }

        self.board[start_x][start_y] = 1;

        if !self.solve_tour(start_x as i32, start_y as i32, 1) {
            return None;
        }

        Some(self.board.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knights_tour_5x5() {
        let tour_matrix = find_knight_tour(5, 5, 0, 0);

        // Define the expected tour matrix for an 8x8 board (assuming a successful tour)
        let expected_matrix: Vec<Vec<usize>> = vec![
            vec![1, 6, 15, 10, 21],
            vec![14, 9, 20, 5, 16],
            vec![19, 2, 7, 22, 11],
            vec![8, 13, 24, 17, 4],
            vec![25, 18, 3, 12, 23],
        ];

        // Compare each cell of the generated tour matrix with the expected matrix
        assert_eq!(tour_matrix, Some(expected_matrix));
    }

    #[test]
    fn test_knights_tour_6x6() {
        let tour_matrix = find_knight_tour(6, 6, 0, 0);

        // Define the expected tour matrix for an 6x6 board (assuming a successful tour)
        let expected_matrix: Vec<Vec<usize>> = vec![
            vec![1, 16, 7, 26, 11, 14],
            vec![34, 25, 12, 15, 6, 27],
            vec![17, 2, 33, 8, 13, 10],
            vec![32, 35, 24, 21, 28, 5],
            vec![23, 18, 3, 30, 9, 20],
            vec![36, 31, 22, 19, 4, 29],
        ];

        // Compare each cell of the generated tour matrix with the expected matrix
        assert_eq!(tour_matrix, Some(expected_matrix));
    }

    #[test]
    fn test_knights_tour_8x8() {
        let tour_matrix = find_knight_tour(8, 8, 0, 0);

        // Define the expected tour matrix for an 5x5 board (assuming a successful tour)
        let expected_matrix: Vec<Vec<usize>> = vec![
            vec![1, 60, 39, 34, 31, 18, 9, 64],
            vec![38, 35, 32, 61, 10, 63, 30, 17],
            vec![59, 2, 37, 40, 33, 28, 19, 8],
            vec![36, 49, 42, 27, 62, 11, 16, 29],
            vec![43, 58, 3, 50, 41, 24, 7, 20],
            vec![48, 51, 46, 55, 26, 21, 12, 15],
            vec![57, 44, 53, 4, 23, 14, 25, 6],
            vec![52, 47, 56, 45, 54, 5, 22, 13],
        ];

        // Compare each cell of the generated tour matrix with the expected matrix
        assert_eq!(tour_matrix, Some(expected_matrix));
    }

    #[test]
    fn test_no_solution_exist() {
        // Define the expected tour matrix for an 5x5 board
        let tour_matrix = find_knight_tour(5, 5, 2, 1);
        assert_eq!(tour_matrix, None);
    }

    #[test]
    fn test_invalid_start_position() {
        let tour_matrix = find_knight_tour(8, 8, 10, 10);
        assert_eq!(tour_matrix, None);
    }
}
