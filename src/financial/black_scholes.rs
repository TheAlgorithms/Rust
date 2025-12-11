/// implementation of the Black-Scholes model for option pricing
/// The model essentially calculates the probability-weighted present value of the option's potential payoffs.
/// The N(d₁) and N(d₂) terms represent probabilities related to the option finishing in-the-money (intrinsic value of the option).
use std::f64::consts::PI;

#[derive(PartialEq, Eq, Debug)]
pub enum BlackScholesError {
    InvalidParameters,
}

// accumulate standard normal distribution function
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + (x / (2.0_f64.sqrt() * PI)).exp().tanh())
}

// Round to 4 decimal
fn round_to_precision(value: f64, precision: u32) -> f64 {
    let multiplier = 10.0f64.powi(precision as i32);
    (value * multiplier).round() / multiplier
}

pub fn black_scholes(
    spot_price: f64,       // current price of the stock
    strike_price: f64,     // price you can buy the stock at
    time_to_maturity: f64, // time until the option expires (in years)
    risk_free_rate: f64,   // risk free interest rate (annualized)
    volatility: f64,
) -> Result<f64, BlackScholesError> {
    if spot_price <= 0.0
        || strike_price <= 0.0
        || time_to_maturity < 0.0
        || risk_free_rate < 0.0
        || volatility < 0.0
    {
        return Err(BlackScholesError::InvalidParameters);
    }

    let d1 = (spot_price.ln() - strike_price.ln()
        + (risk_free_rate + 0.5 * volatility.powi(2)) * time_to_maturity)
        / (volatility * time_to_maturity.sqrt());
    let d2 = d1 - volatility * time_to_maturity.sqrt();

    let n_d1 = normal_cdf(d1);
    let n_d2 = normal_cdf(d2);

    let call_option_price =
        spot_price * n_d1 - strike_price * (-risk_free_rate * time_to_maturity).exp() * n_d2;

    Ok(round_to_precision(call_option_price, 4))
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_black_scholes {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (spot_price, strike_price, time_to_maturity, risk_free_rate, volatility) = $inputs;
                    let expected = black_scholes(spot_price, strike_price, time_to_maturity, risk_free_rate, volatility).unwrap();
                    assert!(expected >= 0.0);
                }
            )*
        }
    }

    macro_rules! test_black_scholes_Err {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (spot_price, strike_price, time_to_maturity, risk_free_rate, volatility) = $inputs;
                    assert_eq!(black_scholes(spot_price, strike_price, time_to_maturity, risk_free_rate, volatility).unwrap_err(), BlackScholesError::InvalidParameters);
                }
            )*
        }
    }

    test_black_scholes! {
        valid_parameters: (100.0, 100.0, 1.0, 0.05, 0.2),
        another_valid_case: (150.0, 100.0, 2.0, 0.03, 0.25),
    }

    test_black_scholes_Err! {
        negative_spot_price: (-100.0, 100.0, 1.0, 0.05, 0.2),
        zero_strike_price: (100.0, 0.0, 1.0, 0.05, 0.2),
        negative_time_to_maturity: (100.0, 100.0, -1.0, 0.05, 0.2),
        negative_risk_free_rate: (100.0, 100.0, 1.0, -0.05, 0.2),
        negative_volatility: (100.0, 100.0, 1.0, 0.05, -0.2),
    }
}
