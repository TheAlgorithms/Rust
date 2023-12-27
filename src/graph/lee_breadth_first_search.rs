use std::collections::VecDeque;

// All four potential movements from a cell are listed here.

fn validate(matrix: &[Vec<i32>], visited: &[Vec<bool>], row: isize, col: isize) -> bool {
    // Check if it is possible to move to the position (row, col) from the current cell.
    let (row, col) = (row as usize, col as usize);
    row < matrix.len() && col < matrix[0].len() && matrix[row][col] == 1 && !visited[row][col]
}

pub fn lee(matrix: Vec<Vec<i32>>, source: (usize, usize), destination: (usize, usize)) -> isize {
    const ROW: [isize; 4] = [-1, 0, 0, 1];
    const COL: [isize; 4] = [0, -1, 1, 0];
    let (i, j) = source;
    let (x, y) = destination;

    // Base case: invalid input
    if matrix.is_empty() || matrix[i][j] == 0 || matrix[x][y] == 0 {
        return -1;
    }

    let (m, n) = (matrix.len(), matrix[0].len());
    let mut visited = vec![vec![false; n]; m];
    let mut q = VecDeque::new();
    visited[i][j] = true;
    q.push_back((i, j, 0));
    let mut min_dist = isize::MAX;

    // Loop until the queue is empty
    while let Some((i, j, dist)) = q.pop_front() {
        if i == x && j == y {
            // If the destination is found, update `min_dist` and stop
            min_dist = dist;
            break;
        }

        // Check for all four possible movements from the current cell
        for k in 0..ROW.len() {
            let row = i as isize + ROW[k];
            let col = j as isize + COL[k];
            if validate(&matrix, &visited, row, col) {
                // Mark the next cell as visited and enqueue it
                let (row, col) = (row as usize, col as usize);
                visited[row][col] = true;
                q.push_back((row, col, dist + 1));
            }
        }
    }

    if min_dist != isize::MAX {
        min_dist
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lee_exists() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
        ];
        let source = (0, 0);
        let dest = (2, 1);
        assert_eq!(lee(mat, source, dest), 3);
    }

    #[test]
    fn test_lee_does_not_exist() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
        ];
        let source = (0, 0);
        let dest = (3, 4);
        assert_eq!(lee(mat, source, dest), -1);
    }

    #[test]
    fn test_source_equals_destination() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
        ];
        let source = (2, 1);
        let dest = (2, 1);
        assert_eq!(lee(mat, source, dest), 0);
    }

    #[test]
    fn test_lee_exists_2() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 1, 1, 1, 1, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0],
        ];
        let source = (0, 0);
        let dest = (3, 2);
        assert_eq!(lee(mat, source, dest), 5);
    }
}
