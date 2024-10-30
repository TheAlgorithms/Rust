#[derive(PartialEq, Eq,Debug)]
pub enum PresentValueError{
    NegetiveDiscount,
    EmptyCashFlow
}


pub fn present_value(discount_rate: f64, cash_flows: Vec<f64>) -> Result<f64, PresentValueError> {
    if discount_rate < 0.0 {
        return Err(PresentValueError::NegetiveDiscount);
    }
    if cash_flows.is_empty() {
        return Err(PresentValueError::EmptyCashFlow);
    }

    let present_value = cash_flows.iter().enumerate()
        .map(|(i, &cash_flow)| cash_flow / (1.0 + discount_rate).powi(i as i32))
        .sum::<f64>();

    Ok((present_value * 100.0).round() / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_value() {

        assert_eq!(4.69,present_value(0.13, vec![10.0, 20.70, -293.0, 297.0]).unwrap());

        assert_eq!( -42739.63,present_value(0.07, vec![-109129.39, 30923.23, 15098.93, 29734.0,39.0]).unwrap());

        assert_eq!(175519.15,present_value(0.07, vec![109129.39, 30923.23, 15098.93, 29734.0,39.0]).unwrap());
    }

    #[test]
    fn test_present_value_negative_discount_rate() {
        assert_eq!(PresentValueError::NegetiveDiscount,present_value(-1.0, vec![10.0, 20.70, -293.0, 297.0]).unwrap_err());

    }

    #[test]
    fn test_present_value_empty_cash_flow() {
        assert_eq!(PresentValueError::EmptyCashFlow,present_value(1.0, vec![]).unwrap_err());
    }

    #[test]
    fn test_present_value_zero_discount_rate() {
        assert_eq!(184924.55,present_value(0.0, vec![109129.39, 30923.23, 15098.93, 29734.0,39.0]).unwrap());
    }
}
