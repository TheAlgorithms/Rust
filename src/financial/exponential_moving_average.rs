//! Calculate the exponential moving average (EMA) on the series of stock prices.
//!
//! Wikipedia Reference: <https://en.wikipedia.org/wiki/Exponential_smoothing>
//! Investopedia Reference: <https://www.investopedia.com/terms/e/ema.asp>
//!
//! Exponential moving average is used in finance to analyze changes in stock prices.
//! EMA is used in conjunction with Simple Moving Average (SMA). EMA reacts to
//! changes in value quicker than SMA, which is one of its key advantages.
//!
//! # Formula
//! ```text
//! st = alpha * xt + (1 - alpha) * st_prev
//! ```
//! Where:
//! - `st`      : Exponential moving average at timestamp t
//! - `xt`      : Stock price at timestamp t
//! - `st_prev` : Exponential moving average at timestamp t-1
//! - `alpha`   : 2 / (1 + window_size) — the smoothing factor

/// Returns an iterator of exponential moving averages over the given stock prices.
///
/// # Errors
/// Returns an error string if `window_size` is zero.
///
/// # Examples
/// ```
/// use the_algorithms_rust::financial::exponential_moving_average;
/// let prices = vec![2.0, 5.0, 3.0, 8.2, 6.0, 9.0, 10.0];
/// let result: Vec<f64> = exponential_moving_average(prices.into_iter(), 3).unwrap().collect();
/// let expected = vec![2.0, 3.5, 3.25, 5.725, 5.8625, 7.43125, 8.715625];
/// for (r, e) in result.iter().zip(expected.iter()) {
///     assert!((r - e).abs() < 1e-9);
/// }
/// ```
pub fn exponential_moving_average(
    stock_prices: impl Iterator<Item = f64>,
    window_size: usize,
) -> Result<impl Iterator<Item = f64>, &'static str> {
    if window_size == 0 {
        return Err("window_size must be > 0");
    }

    let alpha = 2.0 / (1.0 + window_size as f64);

    let iter = stock_prices
        .enumerate()
        .scan(0.0_f64, move |moving_average, (i, stock_price)| {
            *moving_average = if i <= window_size {
                // Use simple moving average until window_size is first reached
                if i == 0 {
                    stock_price
                } else {
                    (*moving_average + stock_price) * 0.5
                }
            } else {
                // Apply exponential smoothing formula
                alpha * stock_price + (1.0 - alpha) * *moving_average
            };
            Some(*moving_average)
        });

    Ok(iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_basic_ema() {
        let prices = vec![2.0, 5.0, 3.0, 8.2, 6.0, 9.0, 10.0];
        let expected = vec![2.0, 3.5, 3.25, 5.725, 5.8625, 7.43125, 8.715625];
        let result: Vec<f64> = exponential_moving_average(prices.into_iter(), 3)
            .unwrap()
            .collect();

        assert_eq!(result.len(), expected.len());
        for (r, e) in result.iter().zip(expected.iter()) {
            assert!(approx_eq(*r, *e), "Expected {e}, got {r}");
        }
    }

    #[test]
    fn test_window_size_one() {
        // With window_size=1, alpha=1.0, so EMA should equal each price after the first
        let prices = vec![1.0, 2.0, 3.0];
        let result: Vec<f64> = exponential_moving_average(prices.into_iter(), 1)
            .unwrap()
            .collect();
        // i=0: price (window boundary), i=1: SMA=(1+2)*0.5=1.5, i=2: alpha=1.0 => 3.0
        assert!(approx_eq(result[0], 1.0));
        assert!(approx_eq(result[1], 1.5));
        assert!(approx_eq(result[2], 3.0));
    }

    #[test]
    fn test_single_price() {
        let prices = vec![42.0];
        let result: Vec<f64> = exponential_moving_average(prices.into_iter(), 3)
            .unwrap()
            .collect();
        assert_eq!(result, vec![42.0]);
    }

    #[test]
    fn test_empty_prices() {
        let prices: Vec<f64> = vec![];
        let result: Vec<f64> = exponential_moving_average(prices.into_iter(), 3)
            .unwrap()
            .collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_zero_window_size_returns_error() {
        let prices = vec![1.0, 2.0, 3.0];
        assert!(exponential_moving_average(prices.into_iter(), 0).is_err());
    }
}
