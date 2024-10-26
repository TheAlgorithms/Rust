pub fn price_plus_tax(price: f64, tax_rate: f64) -> f64 {
    price * (1_f64 + tax_rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_plus_tax() {
        assert_eq!(131.775, price_plus_tax(125.50, 0.05));
        assert_eq!(125.0, price_plus_tax(100.0, 0.25));
    }
}
