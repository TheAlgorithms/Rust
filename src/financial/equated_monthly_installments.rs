//! Calculates the Equated Monthly Installment (EMI) for a loan.
//!
//! Formula: A = p * r * (1 + r)^n / ((1 + r)^n - 1)
//! where:
//!   - `p` is the principal
//!   - `r` is the monthly interest rate (annual rate / 12)
//!   - `n` is the total number of monthly payments (years * 12)
//!
//! Wikipedia Reference: https://en.wikipedia.org/wiki/Equated_monthly_installment

/// Computes the monthly EMI for a loan.
///
/// # Arguments
/// * `principal`        - The total amount borrowed (must be > 0)
/// * `rate_per_annum`   - Annual interest rate as a decimal, e.g. 0.12 for 12% (must be >= 0)
/// * `years_to_repay`   - Loan term in whole years (must be > 0)
///
/// # Errors
/// Returns an `Err(&'static str)` if any argument is out of range.
pub fn equated_monthly_installments(
    principal: f64,
    rate_per_annum: f64,
    years_to_repay: u32,
) -> Result<f64, &'static str> {
    if principal <= 0.0 {
        return Err("Principal borrowed must be > 0");
    }
    if rate_per_annum < 0.0 {
        return Err("Rate of interest must be >= 0");
    }
    if years_to_repay == 0 {
        return Err("Years to repay must be an integer > 0");
    }

    // Divide annual rate by 12 to obtain the monthly rate
    let rate_per_month = rate_per_annum / 12.0;

    // Multiply years by 12 to obtain the total number of monthly payments
    let number_of_payments = f64::from(years_to_repay * 12);

    // Handle the edge case where the interest rate is 0 (simple division)
    if rate_per_month == 0.0 {
        return Ok(principal / number_of_payments);
    }

    let factor = (1.0 + rate_per_month).powf(number_of_payments);
    Ok(principal * rate_per_month * factor / (factor - 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equated_monthly_installments() {
        const EPSILON: f64 = 1e-8;

        // Standard cases
        let result = equated_monthly_installments(25000.0, 0.12, 3).unwrap();
        assert!((result - 830.357_745_321_279_3).abs() < EPSILON);

        let result = equated_monthly_installments(25000.0, 0.12, 10).unwrap();
        assert!((result - 358.677_371_006_468_26).abs() < EPSILON);

        // With 0% interest the EMI is simply principal / number_of_payments
        let result = equated_monthly_installments(12000.0, 0.0, 1).unwrap();
        assert!((result - 1000.0).abs() < EPSILON);

        // Error cases
        assert_eq!(
            equated_monthly_installments(0.0, 0.12, 3),
            Err("Principal borrowed must be > 0")
        );
        assert_eq!(
            equated_monthly_installments(-5000.0, 0.12, 3),
            Err("Principal borrowed must be > 0")
        );
        assert_eq!(
            equated_monthly_installments(25000.0, -1.0, 3),
            Err("Rate of interest must be >= 0")
        );
        assert_eq!(
            equated_monthly_installments(25000.0, 0.12, 0),
            Err("Years to repay must be an integer > 0")
        );
    }
}
