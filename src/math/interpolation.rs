/// In mathematics, linear interpolation is a method of curve fitting
/// using linear polynomials to construct new data points within the range of a discrete set of known data points.
/// Formula: y = y0 + (x - x0) * (y1 - y0) / (x1 - x0)
/// Source: https://en.wikipedia.org/wiki/Linear_interpolation
/// point0 and point1 are a tuple containing x and y values we want to interpolate between
pub fn linear_interpolation(x: f64, point0: (f64, f64), point1: (f64, f64)) -> f64 {
    point0.1 + (x - point0.0) * (point1.1 - point0.1) / (point1.0 - point0.0)
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
}
