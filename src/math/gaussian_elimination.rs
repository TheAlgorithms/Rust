// Gaussian Elimination of Quadratic Matrices
// Takes an augmented matrix as input, returns vector of results
// Wikipedia reference: augmented matrix: https://en.wikipedia.org/wiki/Augmented_matrix
// Wikipedia reference: algorithm: https://en.wikipedia.org/wiki/Gaussian_elimination

pub fn gaussian_elimination(matrix: &mut Vec<Vec<f32>>) -> Vec<f32> {
    let size = matrix.len();
    assert_eq!(size, matrix[0].len() - 1);

    for i in 0..size -1 {
        for j in i..size -1 {
            echelon(matrix, i, j);
        }
    }

    for i in (1..size).rev() {
        eliminate(matrix, i);
    }

    for i in 0..size {
        if matrix[i][i] == 0f32 {
            println!("Infnitely many solutions");
        }
        else {
            matrix[i][size] /= matrix[i][i] as f32;
            matrix[i][i] = 1f32;
            println!("X{} = {}", i + 1, matrix[i][size]);
        }
    }

    let mut result: Vec<f32> = vec![0f32; size];
    for i in 0..size {
        result[i] = matrix[i][size] / matrix[i][i];
    }
    result
}


fn echelon(matrix: &mut Vec<Vec<f32>>, i: usize, j: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f32 {
        return;
    } else {
        let factor = matrix[j + 1][i] as f32 / matrix[i][i] as f32;
        (i..size+1).for_each(|k|{
            matrix[j + 1][k] -= factor * matrix[i][k];
        });
    }
}


fn eliminate(matrix: &mut Vec<Vec<f32>>, i: usize) {
    let size = matrix.len();
    if matrix[i][i] == 0f32 {
        return;
    } else {
        for j in (1..i+1).rev() {
            let factor = matrix[j - 1][i] as f32 / matrix[i][i] as f32;
            for k in (0..size+1).rev() {
                matrix[j - 1][k] -= factor * matrix[i][k] as f32;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::gaussian_elimination;

    #[test]
    fn test_gauss(){
        let mut matrix: Vec<Vec<f32>> = vec![vec![1.5, 2.0, 1.0, -1.0, -2.0, 1.0, 1.0],
                                             vec![3.0, 3.0, -1.0, 16.0, 18.0, 1.0, 1.0],
                                             vec![1.0, 1.0, 3.0, -2.0, -6.0, 1.0, 1.0],
                                             vec![1.0, 1.0, 99.0, 19.0, 2.0, 1.0, 1.0],
                                             vec![1.0, -2.0, 16.0, 1.0, 9.0, 10.0, 1.0],
                                             vec![1.0, 3.0, 1.0, -5.0, 1.0, 1.0, 95.0]];
        let result = vec![-264.05893, 159.63196, -6.156921, 35.310387, -18.806696, 81.67839];
        assert_eq!(gaussian_elimination(&mut matrix), result);
    }
}