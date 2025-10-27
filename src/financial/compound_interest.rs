// compound interest is given by A = P(1+r/n)^nt
// where: A = Final Amount, P = Principal Amount, r = rate of interest,
// n = number of times interest is compounded per year and t = time (in years)

pub fn compound_interest(principal: f64, rate: f64, comp_per_year: u32, years: f64) -> f64 {
    principal * (1.00 + rate / comp_per_year as f64).powf(comp_per_year as f64 * years)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_interest() {
        let principal = 1000.0;
        let rate = 0.05; // 5% annual interest
        let times_per_year = 4; // interest compounded quarterly
        let years = 2.0; // 2 years tenure
        let result = compound_interest(principal, rate, times_per_year, years);
        assert!((result - 1104.486).abs() < 0.001); // expected value rounded to 3 decimal
                                                    // places
    }
}
