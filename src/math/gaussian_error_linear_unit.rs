use std::f64::consts::E;
use std::f64::consts::PI;

fn tanh(vector: f64) -> f64 {
    (2. / (1. + E.powf(-2. * vector.to_owned()))) - 1.
}

pub fn gaussian_error_linear_unit(vector: &Vec<f64>) -> Vec<f64> {
    let mut gelu_vec = vector.to_owned();
    for value in &mut gelu_vec {
        *value = *value
            * 0.5
            * (1. + tanh(f64::powf(2. / PI, 0.5) * (*value + 0.044715 * value.powf(3.))));
    }

    gelu_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_error_linear_unit() {
        let test_vector = vec![-10., 2., -3., 4., -5., 10., 0.05];
        assert_eq!(
            gaussian_error_linear_unit(&test_vector),
            vec![
                -0.0,
                1.9545976940877752,
                -0.0036373920817729943,
                3.9999297540518075,
                -2.2917961972623857e-7,
                10.0,
                0.025996938238622008
            ]
        );
    }
}
