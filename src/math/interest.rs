// value of e
use std::f64::consts::E;

// function to calculate simple interest
pub fn simple_interest(principal: f64, annual_rate: f64, years: f64) -> (f64, f64) {
    let interest = principal * annual_rate * years;
    let value = principal * (1.0 + (annual_rate * years));

    println!("Interest earned: {interest}");
    println!("Future value: {value}");

    (interest, value)
}

// function to calculate compound interest compounded over periods or continuously
pub fn compound_interest(principal: f64, annual_rate: f64, years: f64, period: Option<f64>) -> f64 {
    // checks if the the period is None type, if so calculates continuous compounding interest
    let value = if period.is_none() {
        principal * E.powf(annual_rate * years)
    } else {
        // unwraps the option type or defaults to 0 if None type and assigns it to prim_period
        let prim_period: f64 = period.unwrap_or(0.0);
        // checks if the period is less than or equal to zero
        if prim_period <= 0.0_f64 {
            return f64::NAN;
        }
        principal * (1.0 + (annual_rate / prim_period).powf(prim_period * years))
    };
    println!("Future value: {value}");
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        let x = 385.65_f64 * 0.03_f64 * 5.0_f64;
        let y = 385.65_f64 * (1.0 + (0.03_f64 * 5.0_f64));
        assert_eq!(simple_interest(385.65_f64, 0.03_f64, 5.0_f64), (x, y));
    }
    #[test]
    fn test_compounding() {
        let x = 385.65_f64 * E.powf(0.03_f64 * 5.0_f64);
        assert_eq!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, None), x);

        let y = 385.65_f64 * (1.0 + (0.03_f64 / 5.0_f64).powf(5.0_f64 * 5.0_f64));
        assert_eq!(
            compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(5.0_f64)),
            y
        );
        assert!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(-5.0_f64)).is_nan());
        assert!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(0.0_f64)).is_nan());
    }
}
