// https://en.wikipedia.org/wiki/Electric_power

#[allow(dead_code)]
pub struct Outcome {
    name: &'static str,
    value: f64,
}

pub fn electric_power(voltage: f64, current: f64, power: f64) -> Result<Outcome, String> {

    if [voltage, current, power].iter().filter(|&&x| x == 0.0).count() != 1 {
        Err("Only one argument must be 0".to_string())
    } else if power < 0.0 {
        Err("Power cannot be negative in any electrical/electronics system".to_string())
    } else if voltage == 0.0 {
        Ok(Outcome { name: "voltage", value: power / current })
    } else if current == 0.0 {
        Ok(Outcome { name: "current", value: power / voltage })
    } else if power == 0.0 {
        Ok(Outcome { name: "power", value: (voltage * current).abs() })
    } else {
        Err("Exactly one argument must be 0".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_electric_power() {
        let res = electric_power(0.0, 2.0, 5.0).unwrap();
        assert_eq!(res.name, "voltage");
        assert!(almost_equal(res.value, 2.5, EPSILON));

        let res = electric_power(2.0, 2.0, 0.0).unwrap();
        assert_eq!(res.name, "power");
        assert!(almost_equal(res.value, 4.0, EPSILON));

        let res = electric_power(-2.0, 3.0, 0.0).unwrap();
        assert_eq!(res.name, "power");
        assert!(almost_equal(res.value, 6.0, EPSILON));

        let res = electric_power(2.2, -2.2, 0.0).unwrap();
        assert_eq!(res.name, "power");
        assert!(almost_equal(res.value, 4.84, EPSILON));
    }

    #[test]
    fn test_invalid_electric_power() {
        let res = electric_power(0.0, 2.0, -5.0).is_err();
        assert!(res);

        let res = electric_power(0.0, 0.0, 5.0).is_err();
        assert!(res);

        let res = electric_power(1.0, 5.0, 5.0).is_err();
        assert!(res);
    }
}
