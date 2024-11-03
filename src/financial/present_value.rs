/// In economics and finance, present value (PV), also known as present discounted value,
/// is the value of an expected income stream determined as of the date of valuation.
///
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Present_value

#[derive(PartialEq, Eq, Debug)]
pub enum PresentValueError {
    NegetiveDiscount,
    EmptyCashFlow,
}

pub fn present_value(discount_rate: f64, cash_flows: Vec<f64>) -> Result<f64, PresentValueError> {
    if discount_rate < 0.0 {
        return Err(PresentValueError::NegetiveDiscount);
    }
    if cash_flows.is_empty() {
        return Err(PresentValueError::EmptyCashFlow);
    }

    let present_value = cash_flows
        .iter()
        .enumerate()
        .map(|(i, &cash_flow)| cash_flow / (1.0 + discount_rate).powi(i as i32))
        .sum::<f64>();

    Ok(round(present_value))
}

fn round(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_present_value {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((discount_rate,cash_flows), expected) = $inputs;
                    assert_eq!(present_value(discount_rate,cash_flows).unwrap(), expected);
                }
            )*
        }
    }

    macro_rules! test_present_value_Err {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let ((discount_rate,cash_flows), expected) = $inputs;
                    assert_eq!(present_value(discount_rate,cash_flows).unwrap_err(), expected);
                }
            )*
        }
    }

    macro_rules! test_round {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $inputs;
                    assert_eq!(round(input), expected);
                }
            )*
        }
    }

    test_present_value! {
        general_inputs1:((0.13, vec![10.0, 20.70, -293.0, 297.0]),4.69),
        general_inputs2:((0.07, vec![-109129.39, 30923.23, 15098.93, 29734.0, 39.0]),-42739.63),
        general_inputs3:((0.07, vec![109129.39, 30923.23, 15098.93, 29734.0, 39.0]), 175519.15),
        zero_input:((0.0, vec![109129.39, 30923.23, 15098.93, 29734.0, 39.0]),  184924.55),

    }

    test_present_value_Err! {
        negative_discount_rate:((-1.0, vec![10.0, 20.70, -293.0, 297.0]), PresentValueError::NegetiveDiscount),
        empty_cash_flow:((1.0, vec![]), PresentValueError::EmptyCashFlow),

    }
    test_round! {
            test1:(0.55434,  0.55),
            test2:(10.453,  10.45),
            test3:(1111_f64,  1111_f64),
    }
}
