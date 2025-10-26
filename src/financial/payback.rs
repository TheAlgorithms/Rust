/// Returns the payback period in years
/// If investment is not paid back, returns None.

pub fn payback(cash_flow: &[f64]) -> Option<usize> {
    let mut total = 0.00;
    for (year, &cf) in cash_flow.iter().enumerate() {
        total += cf;
        if total >= 0.00 {
            return Some(year);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payback_period() {
        let cash_flows = vec![-1000.0, 300.0, 400.0, 500.0];
        assert_eq!(payback_period(&cash_flows), Some(3)); // paid back in year 3
    }

    #[test]
    fn test_no_payback() {
        let cash_flows = vec![-1000.0, 100.0, 100.0, 100.0];
        assert_eq!(payback_period(&cash_flows), None); // never paid back
    }
}
