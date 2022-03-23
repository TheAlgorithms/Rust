/*
  Matrix Operations
  transpose, sum & difference, scalar multiplication
  matrix multiplication, matrix-vector product
  matrix inverse
*/

/*
  transpose of m × n matrix A, denoted A^T or A′, is n × m matrix with
  (A^T)ij = Aji
  rows and columns of A are transposed in A
   (A^T)^T = A 
*/

pub fn matrix_transpose(input: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![];
    let dimension_check = input[0].len();

    for column in 0..input[0].len() {
        ans.push(vec![]);
        
        for row in 0..input.len() {  
            if dimension_check != input[row].len() {
                panic!("invalid matrix dimensions");
            }  
            ans[column].push(input[row][column]);
           }
     }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::matrix_transpose;

    #[test]
    fn test_transpose() {
        let input: Vec<Vec<i32>> = vec![
                vec![1, 0], 
                vec![2, 0],
                vec![3, 0]];
        let expected: Vec<Vec<i32>> = vec![
                vec![1, 2, 3],
                vec![0, 0, 0]];

        assert_eq!(expected, matrix_transpose(&input));
        assert_eq!(input, matrix_transpose(&matrix_transpose(&input)))
    }
}
