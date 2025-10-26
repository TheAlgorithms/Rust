/// Calculates Net Present Value given a vector of cash flows and a discount rate.
/// cash_flows: Vector of f64 representing cash flows for each period.
/// rate: Discount rate as an f64 (e.g., 0.05 for 5%)

pub fn npv(cash_flows: &[f64], rate: f64) -> f64 {
    cash_flows
        .iter()
        .enumerate()
        .map(|(t, &cf)| cf / (1.00 + rate).powi(t as i32))
        .sum()
}

// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npv_basic() {
        let cash_flows = vec![-1000.00, 300.00, 400.00, -50.00];
        let rate = 0.10;
        let result = npv(&cash_flows, rate);
        assert!((result - 27.30).abs() < 0.05); //small margin of error
    }

    #[test]
    fn test_npv_zero_rate() {
        let cash_flows = vec![100.0, 200.0, -50.0];
        let rate = 0.0;
        let result = npv(&cash_flows, rate);
        assert!((result - 250.0).abs() < 0.05);
    }
    // for empty entry
    #[test]
    fn test_npv_empty() {
        let cash_flows: Vec<f64> = vec![];
        let rate = 0.05;
        let result = npv(&cash_flows, rate);
        assert_eq!(result, 0.0);
    }
}
