// https://en.wikipedia.org/wiki/Series_and_parallel_circuits

pub fn resistor_parallel(resistors: &[f64]) -> Result<f64, String> {
    // 1/Rtotal = 1/R1 + 1/R2 + ... + 1/Rn
    let mut sum_r: f64 = 0.00;

    for (index, resistor) in resistors.iter().enumerate() {
        if *resistor <= 0.0 {
            return Err(format!(
                "Resistor at index {} has a negative or zero value, namely {}",
                index, resistors[index]
            ));
        }
        sum_r += 1.0 / resistor;
    }

    Ok(1.0 / sum_r)
}

pub fn resistor_series(resistors: &[f64]) -> Result<f64, String> {
    // Rtotal = Rs = R1 + R2 + ... + Rn
    let mut sum_r: f64 = 0.00;

    for (index, resistor) in resistors.iter().enumerate() {
        if *resistor < 0.0 {
            return Err(format!(
                "Resistor at index {} has a negative value, namely {}",
                index, resistors[index]
            ));
        }
        sum_r += resistor;
    }

    Ok(sum_r)
}

#[cfg(test)]
mod test {
    use super::*;
    fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_parallel() {
        let series = [0.1, 2.0, 5.0];
        assert!(almost_equal(
            resistor_parallel(&series).unwrap(),
            0.0934579,
            EPSILON
        ));
    }

    #[test]
    fn test_series() {
        let series = [0.0, 2.0, 5.0];
        assert_eq!(resistor_series(&series).unwrap(), 7.0);
    }

    #[test]
    fn test_invalid_parallel() {
        let series = [0.0, 2.0];
        assert!(resistor_parallel(&series).is_err());
    }

    #[test]
    fn test_invalid_series() {
        let series = [-1.0, 2.0];
        assert!(resistor_series(&series).is_err());
    }
}
