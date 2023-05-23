/// In mathematics, linear interpolation is a method of curve fitting
/// using linear polynomials to construct new data points within the range of a discrete set of known data points.
/// Formula: y = y0 + (x - x0) * (y1 - y0) / (x1 - x0)
/// Source: https://en.wikipedia.org/wiki/Linear_interpolation
/// point0 and point1 are a tuple containing x and y values we want to interpolate between
pub fn linear_interpolation(x: f64, point0: (f64, f64), point1: (f64, f64)) -> f64 {
    point0.1 + (x - point0.0) * (point1.1 - point0.1) / (point1.0 - point0.0)
}

/// In numerical analysis, the Lagrange interpolating polynomial
/// is the unique polynomial of lowest degree that interpolates a given set of data.
///
/// Source: https://en.wikipedia.org/wiki/Lagrange_polynomial
/// Source: https://mathworld.wolfram.com/LagrangeInterpolatingPolynomial.html
/// x is the point we wish to interpolate
/// defined points are a vector of tuples containing known x and y values of our function
pub fn lagrange_polynomial_interpolation(x: f64, defined_points: &Vec<(f64, f64)>) -> f64 {
    let mut defined_x_values: Vec<f64> = Vec::new();
    let mut defined_y_values: Vec<f64> = Vec::new();

    for (x, y) in defined_points {
        defined_x_values.push(*x);
        defined_y_values.push(*y);
    }

    let mut sum = 0.0;

    for y_index in 0..defined_y_values.len() {
        let mut numerator = 1.0;
        let mut denominator = 1.0;
        for x_index in 0..defined_x_values.len() {
            if y_index == x_index {
                continue;
            }
            denominator *= defined_x_values[y_index] - defined_x_values[x_index];
            numerator *= x - defined_x_values[x_index];
        }

        sum += numerator / denominator * defined_y_values[y_index];
    }
    sum
}

#[cfg(test)]
mod tests {

    use std::assert_eq;

    use super::*;
    #[test]
    fn test_linear_intepolation() {
        let point1 = (0.0, 0.0);
        let point2 = (1.0, 1.0);
        let point3 = (2.0, 2.0);

        let x1 = 0.5;
        let x2 = 1.5;

        let y1 = linear_interpolation(x1, point1, point2);
        let y2 = linear_interpolation(x2, point2, point3);

        assert_eq!(y1, x1);
        assert_eq!(y2, x2);
        assert_eq!(
            linear_interpolation(x1, point1, point2),
            linear_interpolation(x1, point2, point1)
        );
    }

    #[test]
    fn test_lagrange_polynomial_interpolation() {
        // defined values for x^2 function
        let defined_points = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0)];

        // check for equality
        assert_eq!(lagrange_polynomial_interpolation(1.0, &defined_points), 1.0);
        assert_eq!(lagrange_polynomial_interpolation(2.0, &defined_points), 4.0);
        assert_eq!(lagrange_polynomial_interpolation(3.0, &defined_points), 9.0);

        //other
        assert_eq!(
            lagrange_polynomial_interpolation(0.5, &defined_points),
            0.25
        );
        assert_eq!(
            lagrange_polynomial_interpolation(2.5, &defined_points),
            6.25
        );
    }
}
