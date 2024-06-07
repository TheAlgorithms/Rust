/// The Minkowski distance or Minkowski metric is a metric in a normed vector space which
/// can be considered as a generalization of both the Euclidean distance and the Manhattan distance.
/// It is named after the Polish mathematician Hermann Minkowski.
/// https://en.wikipedia.org/wiki/Minkowski_distance
///
/// This metric is useful in the field of machine learning to judge the similarity between data points.
/// It's also employed for collision detection in game engines.

use num_traits::Pow;

pub struct Point {
    pub x: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub enum ParameterError {
    DifferentDimensions,
    InvalidOrder,
}

pub fn minkowski_difference(a: &Point, b: &Point, order: u32) -> Result<f64, ParameterError> {
    if order < 1 {
        return Err(ParameterError::DifferentDimensions);
    }

    if a.x.len() != b.x.len() {
        return Err(ParameterError::DifferentDimensions);
    }

    let order = order as f64;

    let sum = a.x.iter()
            .zip(b.x.iter())
            .map(|(x, y)| (x - y).abs().pow(order)).sum::<f64>();

    Ok(sum.pow(1.0 / order))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minkowski_difference() {
        let a = Point { x: vec![1.0, 1.0] };
        let b = Point { x: vec![2.0, 2.0] };

        assert_eq!(minkowski_difference(&a, &b, 1), Ok(2.0));

        let a = Point { x: vec![1.0, 2.0, 3.0, 4.0] };
        let b = Point { x: vec![5.0, 6.0, 7.0, 8.0] };

        assert_eq!(minkowski_difference(&a, &b, 2), Ok(8.0));
    }

    #[test]
    fn test_minkowski_difference_different_dimensions() {
        let a = Point { x: vec![1.0, 1.0] };
        let b = Point { x: vec![2.0, 2.0, 2.0] };

        assert_eq!(minkowski_difference(&a, &b, 1), Err(ParameterError::DifferentDimensions));
    }
}