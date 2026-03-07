//! Calculates simple, compound, and APR interest on a principal amount.
//!
//! Formulas:
//!   Simple Interest:   I = p * r * t
//!   Compound Interest: I = p * ((1 + r)^n - 1)
//!   APR Interest:      Compound interest with r = annual_rate / 365
//!                      and n = years * 365
//! where:
//!   - `p` is the principal
//!   - `r` is the interest rate per period
//!   - `t` is the number of periods (days)
//!   - `n` is the total number of compounding periods
//!
//! Reference: https://www.investopedia.com/terms/i/interest.asp

/// Calculates simple interest earned over a number of days.
///
/// # Arguments
/// * `principal`             - The initial amount of money (must be > 0)
/// * `daily_interest_rate`   - The daily interest rate as a decimal (must be >= 0)
/// * `days_between_payments` - The number of days between payments (must be > 0)
///
/// # Errors
/// Returns an `Err(&'static str)` if any argument is out of range.
pub fn simple_interest(
    principal: f64,
    daily_interest_rate: f64,
    days_between_payments: f64,
) -> Result<f64, &'static str> {
    if principal <= 0.0 {
        return Err("principal must be > 0");
    }
    if daily_interest_rate < 0.0 {
        return Err("daily_interest_rate must be >= 0");
    }
    if days_between_payments <= 0.0 {
        return Err("days_between_payments must be > 0");
    }
    Ok(principal * daily_interest_rate * days_between_payments)
}

/// Calculates compound interest earned over a number of compounding periods.
///
/// # Arguments
/// * `principal`                     - The initial amount of money (must be > 0)
/// * `nominal_annual_interest_rate`  - The rate per compounding period as a decimal (must be >= 0)
/// * `number_of_compounding_periods` - The total number of compounding periods (must be > 0)
///
/// # Errors
/// Returns an `Err(&'static str)` if any argument is out of range.
pub fn compound_interest(
    principal: f64,
    nominal_annual_interest_rate: f64,
    number_of_compounding_periods: f64,
) -> Result<f64, &'static str> {
    if principal <= 0.0 {
        return Err("principal must be > 0");
    }
    if nominal_annual_interest_rate < 0.0 {
        return Err("nominal_annual_interest_rate must be >= 0");
    }
    if number_of_compounding_periods <= 0.0 {
        return Err("number_of_compounding_periods must be > 0");
    }
    Ok(
        principal
            * ((1.0 + nominal_annual_interest_rate).powf(number_of_compounding_periods) - 1.0),
    )
}

/// Calculates interest using the Annual Percentage Rate (APR), compounded daily.
///
/// Converts the APR to a daily rate and compounds over the equivalent number
/// of days, giving a more accurate real-world figure than simple annual compounding.
///
/// # Arguments
/// * `principal`                      - The initial amount of money (must be > 0)
/// * `nominal_annual_percentage_rate` - The APR as a decimal (must be >= 0)
/// * `number_of_years`                - The loan or investment duration in years (must be > 0)
///
/// # Errors
/// Returns an `Err(&'static str)` if any argument is out of range.
pub fn apr_interest(
    principal: f64,
    nominal_annual_percentage_rate: f64,
    number_of_years: f64,
) -> Result<f64, &'static str> {
    if principal <= 0.0 {
        return Err("principal must be > 0");
    }
    if nominal_annual_percentage_rate < 0.0 {
        return Err("nominal_annual_percentage_rate must be >= 0");
    }
    if number_of_years <= 0.0 {
        return Err("number_of_years must be > 0");
    }
    // Divide annual rate by 365 to obtain the daily rate
    // Multiply years by 365 to obtain the total number of daily compounding periods
    compound_interest(
        principal,
        nominal_annual_percentage_rate / 365.0,
        number_of_years * 365.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interest() {
        const EPSILON: f64 = 1e-9;

        // Standard cases
        assert!((simple_interest(18000.0, 0.06, 3.0).unwrap() - 3240.0).abs() < EPSILON);
        assert!((simple_interest(0.5, 0.06, 3.0).unwrap() - 0.09).abs() < EPSILON);
        assert!((simple_interest(18000.0, 0.01, 10.0).unwrap() - 1800.0).abs() < EPSILON);
        assert!((simple_interest(5500.0, 0.01, 100.0).unwrap() - 5500.0).abs() < EPSILON);

        // Zero interest rate yields zero interest
        assert!((simple_interest(18000.0, 0.0, 3.0).unwrap() - 0.0).abs() < EPSILON);

        // Error cases
        assert_eq!(
            simple_interest(-10000.0, 0.06, 3.0),
            Err("principal must be > 0")
        );
        assert_eq!(
            simple_interest(0.0, 0.06, 3.0),
            Err("principal must be > 0")
        );
        assert_eq!(
            simple_interest(10000.0, -0.06, 3.0),
            Err("daily_interest_rate must be >= 0")
        );
        assert_eq!(
            simple_interest(5500.0, 0.01, -5.0),
            Err("days_between_payments must be > 0")
        );
        assert_eq!(
            simple_interest(5500.0, 0.01, 0.0),
            Err("days_between_payments must be > 0")
        );
    }

    #[test]
    fn test_compound_interest() {
        const EPSILON: f64 = 1e-9;

        // Standard cases
        assert!(
            (compound_interest(10000.0, 0.05, 3.0).unwrap() - 1_576.250_000_000_001_4).abs()
                < EPSILON
        );
        assert!(
            (compound_interest(10000.0, 0.05, 1.0).unwrap() - 500.000_000_000_000_45).abs()
                < EPSILON
        );
        assert!(
            (compound_interest(0.5, 0.05, 3.0).unwrap() - 0.078_812_500_000_000_06).abs() < EPSILON
        );

        // Zero interest rate yields zero interest
        assert!((compound_interest(10000.0, 0.0, 5.0).unwrap() - 0.0).abs() < EPSILON);

        // Error cases
        assert_eq!(
            compound_interest(-5500.0, 0.01, 5.0),
            Err("principal must be > 0")
        );
        assert_eq!(
            compound_interest(10000.0, -3.5, 3.0),
            Err("nominal_annual_interest_rate must be >= 0")
        );
        assert_eq!(
            compound_interest(10000.0, 0.06, -4.0),
            Err("number_of_compounding_periods must be > 0")
        );
    }

    #[test]
    fn test_apr_interest() {
        const EPSILON: f64 = 1e-9;

        // Standard cases
        assert!(
            (apr_interest(10000.0, 0.05, 3.0).unwrap() - 1_618.223_072_263_547).abs() < EPSILON
        );
        assert!(
            (apr_interest(10000.0, 0.05, 1.0).unwrap() - 512.674_964_674_473_2).abs() < EPSILON
        );
        assert!((apr_interest(0.5, 0.05, 3.0).unwrap() - 0.080_911_153_613_177_36).abs() < EPSILON);

        // Zero interest rate yields zero interest
        assert!((apr_interest(10000.0, 0.0, 5.0).unwrap() - 0.0).abs() < EPSILON);

        // Error cases
        assert_eq!(
            apr_interest(-5500.0, 0.01, 5.0),
            Err("principal must be > 0")
        );
        assert_eq!(
            apr_interest(10000.0, -3.5, 3.0),
            Err("nominal_annual_percentage_rate must be >= 0")
        );
        assert_eq!(
            apr_interest(10000.0, 0.06, -4.0),
            Err("number_of_years must be > 0")
        );
    }
}
