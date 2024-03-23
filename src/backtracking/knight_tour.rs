//! This module contains the implementation of the Knight's Tour problem.
//!
//! The Knight's Tour is a classic chess problem where the objective is to move a knight to every square on a chessboard exactly once.

/// Represents the KnightTour struct which implements the Knight's Tour problem.
pub struct KnightTour {
    board_size: usize,
    board: Vec<Vec<i32>>,
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
    pub fn new(size: usize) -> Self {
        let board = vec![vec![0; size]; size];
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
            board_size: size,
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
            && x < self.board_size as i32
            && y < self.board_size as i32
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
        if move_count == self.board_size as i32 * self.board_size as i32 {
            return true;
        }

        for i in 0..8 {
            let next_x = x + self.moves[i].0;
            let next_y = y + self.moves[i].1;

            if self.is_safe(next_x, next_y) {
                self.board[next_x as usize][next_y as usize] = move_count + 1;

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
    /// A tuple containing a boolean indicating whether a tour was found and the tour matrix if found.
    pub fn find_tour(&mut self, start_x: usize, start_y: usize) -> (bool, Vec<Vec<i32>>) {
        if !self.is_safe(start_x as i32, start_y as i32) {
            println!("Invalid starting position");
            return (false, vec![]);
        }

        self.board[start_x][start_y] = 1;

        if !self.solve_tour(start_x as i32, start_y as i32, 1) {
            println!("No solution exists");
            return (false, vec![]);
        }

        println!("Knight's Tour:");
        (true, self.board.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knights_tour_5x5() {
        let mut tour = KnightTour::new(5);
        let (found, tour_matrix) = tour.find_tour(0, 0);
        assert_eq!(found, true);

        // Define the expected tour matrix for an 8x8 board (assuming a successful tour)
        let expected_matrix: Vec<Vec<i32>> = vec![
            vec![1, 6, 15, 10, 21],
            vec![14, 9, 20, 5, 16],
            vec![19, 2, 7, 22, 11],
            vec![8, 13, 24, 17, 4],
            vec![25, 18, 3, 12, 23],
        ];

        // Compare each cell of the generated tour matrix with the expected matrix
        for i in 0..5 {
            assert_eq!(tour_matrix[i], expected_matrix[i]);
        }
    }

    #[test]
    fn test_knights_tour_6x6() {
        let mut tour = KnightTour::new(6);
        let (found, tour_matrix) = tour.find_tour(0, 0);
        assert_eq!(found, true);

        // Define the expected tour matrix for an 6x6 board (assuming a successful tour)
        let expected_matrix: Vec<Vec<i32>> = vec![
            vec![1, 16, 7, 26, 11, 14],
            vec![34, 25, 12, 15, 6, 27],
            vec![17, 2, 33, 8, 13, 10],
            vec![32, 35, 24, 21, 28, 5],
            vec![23, 18, 3, 30, 9, 20],
            vec![36, 31, 22, 19, 4, 29],
        ];

        // Compare each cell of the generated tour matrix with the expected matrix
        for i in 0..6 {
            assert_eq!(tour_matrix[i], expected_matrix[i]);
        }
    }

    #[test]
    fn test_knights_tour_8x8() {
        let mut tour = KnightTour::new(8);
        let (found, tour_matrix) = tour.find_tour(0, 0);
        assert_eq!(found, true);

        // Define the expected tour matrix for an 5x5 board (assuming a successful tour)
        let expected_matrix: Vec<Vec<i32>> = vec![
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
        for i in 0..8 {
            assert_eq!(tour_matrix[i], expected_matrix[i]);
        }
    }

    #[test]
    fn test_no_solution_exist() {
        // Define the expected tour matrix for an 5x5 board
        let mut tour = KnightTour::new(5);
        let (found, _) = tour.find_tour(2, 1);
        assert_eq!(found, false);
    }

    #[test]
    fn test_invalid_start_position() {
        let mut tour = KnightTour::new(8);
        let (found, _) = tour.find_tour(10, 10);
        assert_eq!(found, false);
    }
}
