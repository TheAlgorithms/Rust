use std::io;

struct Solution {
    found_at_row: i32,
    found_at_col: i32,
    matrix: Vec<Vec<i32>>,
}

impl Solution {
    fn new(matrix: Vec<Vec<i32>>) -> Solution {
        Solution {
            found_at_row: -1,
            found_at_col: -1,
            matrix,
        }
    }

    fn search_matrix(&mut self, target: i32) -> bool {
        let n = self.matrix.len();
        let m = self.matrix[0].len();

        for i in 0..n {
            if self.matrix[i][0] <= target && self.matrix[i][m - 1] >= target {
                let mut low = 0;
                let mut high = m - 1;

                while low <= high {
                    let mid = (low + high) / 2;

                    if self.matrix[i][mid] == target {
                        self.found_at_row = i as i32;
                        self.found_at_col = mid as i32;
                        return true;
                    }

                    if self.matrix[i][mid] <= target {
                        low = mid + 1;
                    } else {
                        high = mid - 1;
                    }
                }
                return false;
            }
        }

        false
    }
}

fn main() {
    // Practice link: https://leetcode.com/problems/search-a-2d-matrix/description/
    // search a sorted 2D matrix
    // implementation: binary search
    // searches in constant space and O(n) + O(log(m)) time
    
    let mut input = String::new();
    println!("Number of rows and columns: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.trim().split_whitespace();
    let m: usize = parts.next().unwrap().parse().expect("");
    let n: usize = parts.next().unwrap().parse().expect("");

    let mut matrix = vec![vec![0; n]; m];

    println!("The grid:");
    for i in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let line = input.trim();
        let row_data: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        matrix[i] = row_data;
    }

    println!("Enter target value: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target: i32 = input.trim().parse().expect("Failed to parse target");

    let mut soln = Solution::new(matrix);
    let flag = soln.search_matrix(target);

    if flag {
        println!(
            "{} found at (row,col): ({}, {}).",
            target, soln.found_at_row, soln.found_at_col
        );
    } else {
        println!("{} not found.", target);
    }
}
