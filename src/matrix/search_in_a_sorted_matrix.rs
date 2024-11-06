///find row and column of given key in given matrix
/// m is row count and n is column count
#[derive(PartialEq, Eq, Debug)]
pub struct SortedMatrixResult {
    pub column: i64,
    pub row: i64,
}
#[derive(PartialEq, Eq, Debug)]
pub enum SortedMatrixErr {
    KeyNotFound,
    InvalidArgument(String),
}
pub fn search_in_a_sorted_matrix(
    mat: Vec<Vec<f64>>,
    m: usize,
    n: usize,
    key: f64,
) -> Result<SortedMatrixResult, SortedMatrixErr> {
    if m < 1 {
        return Err(SortedMatrixErr::InvalidArgument(String::from(
            "m must be greater than or equal to 1",
        )));
    }

    if n < 1 {
        return Err(SortedMatrixErr::InvalidArgument(String::from(
            "n must be greater than or equal to 1",
        )));
    }

    if mat.is_empty() {
        return Err(SortedMatrixErr::InvalidArgument(String::from(
            "mat must be non-empty",
        )));
    }

    let (mut i, mut j) = (m - 1, 0);

    while j < n {
        if (mat[i][j] - key).abs() < f64::EPSILON {
            return Ok(SortedMatrixResult {
                column: (j + 1) as i64,
                row: (i + 1) as i64,
            });
        }
        if key < mat[i][j] {
            i -= 1;
        } else {
            j += 1;
        }
    }
    Err(SortedMatrixErr::KeyNotFound)
}
#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_search_in_a_sorted_matrix_err {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((mat,m,n,key), expected) = $inputs;
                    assert_eq!(search_in_a_sorted_matrix(mat,m,n,key).unwrap_err(), expected);
                }
            )*
        }
    }
    macro_rules! test_search_in_a_sorted_matrix {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((mat,m,n,key), expected) = $inputs;
                    assert_eq!(search_in_a_sorted_matrix(mat,m,n,key).unwrap(), expected);
                }
            )*
        }
    }

    test_search_in_a_sorted_matrix_err! {
        key_not_found_test: ((vec![vec![1.0,1.0],vec![1.0,1.0]],2,2,4.0), SortedMatrixErr::KeyNotFound),
        invalid_m_test: ((vec![vec![1.0,1.0],vec![1.0,1.0]],0,2,4.0), SortedMatrixErr::InvalidArgument(String::from("m must be greater than or equal to 1"))),
        invalid_n_test: ((vec![vec![1.0,1.0],vec![1.0,1.0]],2,0,4.0), SortedMatrixErr::InvalidArgument(String::from("n must be greater than or equal to 1"))),
    }

    test_search_in_a_sorted_matrix! {
        kgeneral_test1: ((vec![vec![2.1, 5.0, 7.0], vec![4.0, 8.0, 13.0], vec![9.0, 11.0, 15.0], vec![12.0, 17.0, 20.0]], 3, 3, 2.1), SortedMatrixResult{
            row:1,
            column:1
        }),
        kgeneral_test2: (( vec![vec![2.0, 5.0, 7.0], vec![4.0, 8.0, 13.0], vec![9.0, 11.0, 15.0], vec![12.0, 17.0, 20.0]], 3, 3, 5.0), SortedMatrixResult{
            row:1,
            column:2
        }),
    }
}
