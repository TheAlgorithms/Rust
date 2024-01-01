/// Least Square Approximation <p>
/// Function that returns a polynomial which very closely passes through the given points (in 2D)
///
/// The result is made of coeficients, in descending order (from x^degree to free term)
///
/// Parameters:
///
/// points -> coordinates of given points
///
/// degree -> degree of the polynomial
///
pub fn least_square_approx<T: Into<f64> + Copy, U: Into<f64> + Copy>(
    points: &[(T, U)],
    degree: i32,
) -> Option<Vec<f64>> {
    use nalgebra::{DMatrix, DVector};

    /* Used for rounding floating numbers */
    fn round_to_decimals(value: f64, decimals: i32) -> f64 {
        let multiplier = 10f64.powi(decimals);
        (value * multiplier).round() / multiplier
    }

    /* Casting the data parsed to this function to f64 (as some points can have decimals) */
    let vals: Vec<(f64, f64)> = points
        .iter()
        .map(|(x, y)| ((*x).into(), (*y).into()))
        .collect();
    /* Because of collect we need the Copy Trait for T and U */

    /* Computes the sums in the system of equations */
    let mut sums = Vec::<f64>::new();
    for i in 1..=(2 * degree + 1) {
        sums.push(vals.iter().map(|(x, _)| x.powi(i - 1)).sum());
    }

    /* Compute the free terms column vector */
    let mut free_col = Vec::<f64>::new();
    for i in 1..=(degree + 1) {
        free_col.push(vals.iter().map(|(x, y)| y * (x.powi(i - 1))).sum());
    }
    let b = DVector::from_row_slice(&free_col);

    /* Create and fill the system's matrix */
    let size = (degree + 1) as usize;
    let a = DMatrix::from_fn(size, size, |i, j| sums[degree as usize + i - j]);

    /* Solve the system of equations: A * x = b */
    match a.qr().solve(&b) {
        Some(x) => {
            let rez: Vec<f64> = x.iter().map(|x| round_to_decimals(*x, 5)).collect();
            Some(rez)
        }
        None => None, //<-- The system cannot be solved (badly conditioned system's matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_points_1st_degree() {
        let points = vec![
            (5.3, 7.8),
            (4.9, 8.1),
            (6.1, 6.9),
            (4.7, 8.3),
            (6.5, 7.7),
            (5.6, 7.0),
            (5.8, 8.2),
            (4.5, 8.0),
            (6.3, 7.2),
            (5.1, 8.4),
        ];

        assert_eq!(
            least_square_approx(&points, 1),
            Some(vec![-0.49069, 10.44898])
        );
    }

    #[test]
    fn eight_points_5th_degree() {
        let points = vec![
            (4f64, 8f64),
            (8f64, 2f64),
            (1f64, 7f64),
            (10f64, 3f64),
            (11.0, 0.0),
            (7.0, 3.0),
            (10.0, 1.0),
            (13.0, 13.0),
        ];

        assert_eq!(
            least_square_approx(&points, 5),
            Some(vec![
                0.00603, -0.21304, 2.79929, -16.53468, 40.29473, -19.35771
            ])
        );
    }

    #[test]
    fn four_points_2nd_degree() {
        let points = vec![
            (2.312, 8.345344),
            (-2.312, 8.345344),
            (-0.7051, 3.49716601),
            (0.7051, 3.49716601),
        ];

        assert_eq!(least_square_approx(&points, 2), Some(vec![1.0, 0.0, 3.0]));
    }
}
