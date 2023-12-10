/**
 * @file
 * @brief Calculate the area of a polygon defined by a vector of points.
 *
 * @details
 * This program provides a function to calculate the area of a polygon defined by a vector of points.
 * The area is calculated using the formula: A = |Σ((xi - xi-1) * (yi + yi-1))| / 2
 * where (xi, yi) are the coordinates of the points in the vector.
 *
 * @param fig A vector of points defining the polygon.
 * @return The area of the polygon.
 *
 * @author [Gyandeep](https://github.com/Gyan172004)
 * @see [Wikipedia - Polygon](https://en.wikipedia.org/wiki/Polygon)
 */

pub struct Point {
    x: f64,
    y: f64,
}

/**
 * Calculate the area of a polygon defined by a vector of points.
 * @param fig A vector of points defining the polygon.
 * @return The area of the polygon.
 */

pub fn area_of_polygon(fig: &[Point]) -> f64 {
    let mut res = 0.0;

    for i in 0..fig.len() {
        let p = if i > 0 {
            &fig[i - 1]
        } else {
            &fig[fig.len() - 1]
        };
        let q = &fig[i];

        res += (p.x - q.x) * (p.y + q.y);
    }

    f64::abs(res) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Test case for calculating the area of a triangle.
     */
    #[test]
    fn test_area_triangle() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 0.0, y: 1.0 },
        ];

        assert_eq!(area_of_polygon(&points), 0.5);
    }

    /**
     * Test case for calculating the area of a square.
     */
    #[test]
    fn test_area_square() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 0.0, y: 1.0 },
        ];

        assert_eq!(area_of_polygon(&points), 1.0);
    }

    /**
     * Test case for calculating the area of a hexagon.
     */
    #[test]
    fn test_area_hexagon() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 1.0, y: 0.0 },
            Point { x: 1.5, y: 0.866 },
            Point { x: 1.0, y: 1.732 },
            Point { x: 0.0, y: 1.732 },
            Point { x: -0.5, y: 0.866 },
        ];

        assert_eq!(area_of_polygon(&points), 2.598);
    }
}
