// matrix_chain_multiply finds the minimum number of multiplications to perform a chain of matrix
// multiplications. The input matrices represents the dimensions of matrices. For example [1,2,3,4]
// represents matrices of dimension (1x2), (2x3), and (3x4)
//
// Lets say we are given [4, 3, 2, 1]. If we naively multiply left to right, we get:
//
// (4*3*2) + (4*2*1) = 20
//
// We can reduce the multiplications by reordering the matrix multiplications:
//
// (3*2*1) + (4*3*1) = 18
//
// We solve this problem with dynamic programming and tabulation. table[i][j] holds the optimal
// number of multiplications in range matrices[i..j] (inclusive). Note this means that table[i][i]
// and table[i][i+1] are always zero, since those represent a single vector/matrix and do not
// require any multiplications.
//
// For any i, j, and k = i+1, i+2, ..., j-1:
//
// table[i][j] = min(table[i][k] + table[k][j] + matrices[i] * matrices[k] * matrices[j])
//
// table[i][k] holds the optimal solution to matrices[i..k]
//
// table[k][j] holds the optimal solution to matrices[k..j]
//
// matrices[i] * matrices[k] * matrices[j] computes the number of multiplications to join the two
// matrices together.
//
// Runs in O(n^3) time and O(n^2) space.

pub fn matrix_chain_multiply(matrices: Vec<u32>) -> u32 {
    let n = matrices.len();
    if n <= 2 {
        // No multiplications required.
        return 0;
    }
    let mut table = vec![vec![0; n]; n];

    for length in 2..n {
        for i in 0..n - length {
            let j = i + length;
            table[i][j] = u32::MAX;
            for k in i + 1..j {
                let multiplications =
                    table[i][k] + table[k][j] + matrices[i] * matrices[k] * matrices[j];
                if multiplications < table[i][j] {
                    table[i][j] = multiplications;
                }
            }
        }
    }

    table[0][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(matrix_chain_multiply(vec![1, 2, 3, 4]), 18);
        assert_eq!(matrix_chain_multiply(vec![4, 3, 2, 1]), 18);
        assert_eq!(matrix_chain_multiply(vec![40, 20, 30, 10, 30]), 26000);
        assert_eq!(matrix_chain_multiply(vec![1, 2, 3, 4, 3]), 30);
        assert_eq!(matrix_chain_multiply(vec![1, 2, 3, 4, 3]), 30);
        assert_eq!(matrix_chain_multiply(vec![4, 10, 3, 12, 20, 7]), 1344);
    }

    #[test]
    fn zero() {
        assert_eq!(matrix_chain_multiply(vec![]), 0);
        assert_eq!(matrix_chain_multiply(vec![10]), 0);
        assert_eq!(matrix_chain_multiply(vec![10, 20]), 0);
    }
}
