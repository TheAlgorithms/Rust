// Matrix operations using row vectors wrapped in column vectors as matrices.
// Supports i32, should be interchangeable for other types.

pub fn matrix_add(summand0: &Vec<Vec<i32>>, summand1: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Add two matrices of identical dimensions
    let mut result: Vec<Vec<i32>> = vec![];
    if summand0.len() != summand1.len() {
        panic!("Matrix dimensions do not match");
    }
    for row in 0..summand0.len() {
        if summand0[row].len() != summand1[row].len() {
            panic!("Matrix dimensions do not match");
        }
        result.push(vec![]);
        for column in 0..summand1[0].len() {
            result[row].push(summand0[row][column] + summand1[row][column]);
        }
    }
    return result;
}

pub fn matrix_subtract(minuend: &Vec<Vec<i32>>, subtrahend: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Subtract one matrix from another. They need to have identical dimensions.
    let mut result: Vec<Vec<i32>> = vec![];
    if minuend.len() != subtrahend.len() {
        panic!("Matrix dimensions do not match");
    }
    for row in 0..minuend.len() {
        if minuend[row].len() != subtrahend[row].len() {
            panic!("Matrix dimensions do not match");
        }
        result.push(vec![]);
        for column in 0..subtrahend[0].len() {
            result[row].push(minuend[row][column] - subtrahend[row][column]);
        }
    }
    return result;
}

pub fn matrix_multiply(multiplier: &Vec<Vec<i32>>, multiplicand: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Multiply two matching matrices. The multiplier needs to have the same amount
    // of columns as the multiplicand has rows.
    let mut result: Vec<Vec<i32>> = vec![];
    let mut temp;
    // Using variable to compare lenghts of rows in multiplicand later
    let row_right_length = multiplicand[0].len();
    for row_left in 0..multiplier.len() {
        if multiplier[row_left].len() != multiplicand.len() {
            panic!("Matrix dimensions do not match");
        }
        result.push(vec![]);
        for column_right in 0..multiplicand[0].len() {
            temp = 0;
            for row_right in 0..multiplicand.len() {
                if row_right_length != multiplicand[row_right].len() {
                    panic!("Matrix dimensions do not match");
                }
                temp += multiplier[row_left][row_right] * multiplicand[row_right][column_right];
            }
            result[row_left].push(temp);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::matrix_add;
    use super::matrix_multiply;
    use super::matrix_subtract;

    #[test]
    fn test_add() {
        let input0: Vec<Vec<i32>> = vec![vec![1, 0, 1], vec![0, 2, 0], vec![5, 0, 1]];
        let input1: Vec<Vec<i32>> = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let input_wrong0: Vec<Vec<i32>> = vec![vec![1, 0, 0, 4], vec![0, 1, 0], vec![0, 0, 1]];
        let input_wrong1: Vec<Vec<i32>> =
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]];
        let input_wrong2: Vec<Vec<i32>> = vec![vec![]];
        let exp_result: Vec<Vec<i32>> = vec![vec![2, 0, 1], vec![0, 3, 0], vec![5, 0, 2]];
        assert_eq!(matrix_add(&input0, &input1), exp_result);
        let result0 = std::panic::catch_unwind(|| matrix_add(&input0, &input_wrong0));
        assert!(result0.is_err());
        let result1 = std::panic::catch_unwind(|| matrix_add(&input0, &input_wrong1));
        assert!(result1.is_err());
        let result2 = std::panic::catch_unwind(|| matrix_add(&input0, &input_wrong2));
        assert!(result2.is_err());
    }

    #[test]
    fn test_subtract() {
        let input0: Vec<Vec<i32>> = vec![vec![1, 0, 1], vec![0, 2, 0], vec![5, 0, 1]];
        let input1: Vec<Vec<i32>> = vec![vec![1, 0, 0], vec![0, 1, 3], vec![0, 0, 1]];
        let input_wrong0: Vec<Vec<i32>> = vec![vec![1, 0, 0, 4], vec![0, 1, 0], vec![0, 0, 1]];
        let input_wrong1: Vec<Vec<i32>> =
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]];
        let input_wrong2: Vec<Vec<i32>> = vec![vec![]];
        let exp_result: Vec<Vec<i32>> = vec![vec![0, 0, 1], vec![0, 1, -3], vec![5, 0, 0]];
        assert_eq!(matrix_subtract(&input0, &input1), exp_result);
        let result0 = std::panic::catch_unwind(|| matrix_subtract(&input0, &input_wrong0));
        assert!(result0.is_err());
        let result1 = std::panic::catch_unwind(|| matrix_subtract(&input0, &input_wrong1));
        assert!(result1.is_err());
        let result2 = std::panic::catch_unwind(|| matrix_subtract(&input0, &input_wrong2));
        assert!(result2.is_err());
    }

    #[test]
    fn test_multiply() {
        let input0: Vec<Vec<i32>> =
            vec![vec![1, 2, 3], vec![4, 2, 6], vec![3, 4, 1], vec![2, 4, 8]];
        let input1: Vec<Vec<i32>> = vec![vec![1, 3, 3, 2], vec![7, 6, 2, 1], vec![3, 4, 2, 1]];
        let input_wrong0: Vec<Vec<i32>> = vec![
            vec![1, 3, 3, 2, 4, 6, 6],
            vec![7, 6, 2, 1],
            vec![3, 4, 2, 1],
        ];
        let input_wrong1: Vec<Vec<i32>> = vec![
            vec![1, 3, 3, 2],
            vec![7, 6, 2, 1],
            vec![3, 4, 2, 1],
            vec![3, 4, 2, 1],
        ];
        let exp_result: Vec<Vec<i32>> = vec![
            vec![24, 27, 13, 7],
            vec![36, 48, 28, 16],
            vec![34, 37, 19, 11],
            vec![54, 62, 30, 16],
        ];
        assert_eq!(matrix_multiply(&input0, &input1), exp_result);
        let result0 = std::panic::catch_unwind(|| matrix_multiply(&input0, &input_wrong0));
        assert!(result0.is_err());
        let result1 = std::panic::catch_unwind(|| matrix_multiply(&input0, &input_wrong1));
        assert!(result1.is_err());
    }
}
