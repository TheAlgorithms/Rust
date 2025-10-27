/// Computes the Net Present Value (NPV) of a cash flow series
/// at multiple discount rates to show sensitivity.
///
/// # Inputs:
/// - `cash_flows`: A slice of cash flows, where each entry is a period value
///   e.g., year 0 is initial investment, year 1+ are returns or costs
/// - `discount_rates`: A slice of discount rates, e.g. `[0.05, 0.10, 0.20]`,
///   where each rate is evaluated independently.
///
/// # Output:
/// - Returns a vector of NPV values, each corresponding to a rate in `discount_rates`.
///   For example, output is `[npv_rate1, npv_rate2, ...]`.

pub fn npv_sensitivity(cash_flows: &[f64], discount_rates: &[f64]) -> Vec<f64> {
    discount_rates
        .iter()
        .cloned()
        .map(|rate| {
            cash_flows
                .iter()
                .enumerate()
                .map(|(t, &cf)| cf / (1.0 + rate).powi(t as i32))
                .sum()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_npv_sensitivity() {
        let cashflows = [-1000.00, 400.00, 400.00, 400.00];
        let rates = vec![0.05, 0.1, 0.2];
        let expected = vec![89.30, -5.26, -157.41];
        let out = npv_sensitivity(&cashflows, &rates);
        assert_eq!(out.len(), 3);
        // value check
        for (o, e) in out.iter().zip(expected.iter()) {
            assert!((o - e).abs() < 0.1);
        }
    }
}
