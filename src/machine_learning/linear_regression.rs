/// Returns the parameters of the line after performing simple linear regression on the input data.
pub fn linear_regression(data_points: Vec<(f64, f64)>) -> Option<(f64, f64)> {
    if data_points.is_empty() {
        return None;
    }

    let count = data_points.len() as f64;
    let mean_x = data_points.iter().fold(0.0, |sum, y| sum + y.0) / count;
    let mean_y = data_points.iter().fold(0.0, |sum, y| sum + y.1) / count;

    let mut covariance = 0.0;
    let mut std_dev_sqr_x = 0.0;
    let mut std_dev_sqr_y = 0.0;

    for data_point in data_points {
        covariance += (data_point.0 - mean_x) * (data_point.1 - mean_y);
        std_dev_sqr_x += (data_point.0 - mean_x).powi(2);
        std_dev_sqr_y += (data_point.1 - mean_y).powi(2);
    }

    let std_dev_x = std_dev_sqr_x.sqrt();
    let std_dev_y = std_dev_sqr_y.sqrt();
    let std_dev_prod = std_dev_x * std_dev_y;

    let pcc = covariance / std_dev_prod; //Pearson's correlation constant
    let b = pcc * (std_dev_y / std_dev_x); //Slope of the line
    let a = mean_y - b * mean_x; //Y-Intercept of the line

    Some((a, b))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_regression() {
        assert_eq!(
            linear_regression(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]),
            Some((2.220446049250313e-16, 0.9999999999999998))
        );
    }

    #[test]
    fn test_empty_list_linear_regression() {
        assert_eq!(linear_regression(vec![]), None);
    }
}
