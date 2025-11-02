use std::f64;

#[derive(Clone, Debug)]
pub struct YinResult {
    sample_rate: f64,
    best_lag: usize,
    cmndf: Vec<f64>,
}

impl YinResult {
    pub fn get_frequency(&self) -> f64 {
        self.sample_rate / self.best_lag as f64
    }

    pub fn get_frequency_with_interpolation(&self) -> f64 {
        let best_lag_with_interpolation = parabolic_interpolation(self.best_lag, &self.cmndf);
        self.sample_rate / best_lag_with_interpolation
    }
}

fn parabolic_interpolation(lag: usize, cmndf: &[f64]) -> f64 {
    let x0 = lag.saturating_sub(1); // max(0, lag-1)
    let x2 = usize::min(cmndf.len() - 1, lag + 1);
    let s0 = cmndf[x0];
    let s1 = cmndf[lag];
    let s2 = cmndf[x2];
    let denom = s0 - 2.0 * s1 + s2;
    if denom == 0.0 {
        return lag as f64;
    }
    let delta = (s0 - s2) / (2.0 * denom);
    lag as f64 + delta
}

#[derive(Clone, Debug)]
pub struct Yin {
    threshold: f64,
    min_lag: usize,
    max_lag: usize,
    sample_rate: f64,
}

impl Yin {
    pub fn init(
        threshold: f64,
        min_expected_frequency: f64,
        max_expected_frequency: f64,
        sample_rate: f64,
    ) -> Yin {
        let min_lag = (sample_rate / max_expected_frequency) as usize;
        let max_lag = (sample_rate / min_expected_frequency) as usize;
        Yin {
            threshold,
            min_lag,
            max_lag,
            sample_rate,
        }
    }

    pub fn yin(&self, frequencies: &[f64]) -> Result<YinResult, String> {
        let df = difference_function_values(frequencies, self.max_lag);
        let cmndf = cumulative_mean_normalized_difference_function(&df, self.max_lag);
        let best_lag = find_cmndf_argmin(&cmndf, self.min_lag, self.max_lag, self.threshold);
        match best_lag {
            _ if best_lag == 0 => Err(format!(
                "Could not find lag value which minimizes CMNDF below the given threshold {}",
                self.threshold
            )),
            _ => Ok(YinResult {
                sample_rate: self.sample_rate,
                best_lag,
                cmndf,
            }),
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn difference_function_values(frequencies: &[f64], max_lag: usize) -> Vec<f64> {
    let mut df_list = vec![0.0; max_lag + 1];
    for lag in 1..=max_lag {
        df_list[lag] = difference_function(frequencies, lag);
    }
    df_list
}

fn difference_function(f: &[f64], lag: usize) -> f64 {
    let mut sum = 0.0;
    let n = f.len();
    for i in 0..(n - lag) {
        let diff = f[i] - f[i + lag];
        sum += diff * diff;
    }
    sum
}

fn cumulative_mean_normalized_difference_function(df: &[f64], max_lag: usize) -> Vec<f64> {
    let mut cmndf = vec![0.0; max_lag + 1];
    cmndf[0] = 1.0;
    let mut sum = 0.0;
    for lag in 1..=max_lag {
        sum += df[lag];
        cmndf[lag] = lag as f64 * df[lag] / if sum == 0.0 { 1e-10 } else { sum };
    }
    cmndf
}

fn find_cmndf_argmin(cmndf: &[f64], min_lag: usize, max_lag: usize, threshold: f64) -> usize {
    let mut lag = min_lag;
    while lag <= max_lag {
        if cmndf[lag] < threshold {
            while lag < max_lag && cmndf[lag + 1] < cmndf[lag] {
                lag += 1;
            }
            return lag;
        }
        lag += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sine_wave(frequency: f64, sample_rate: f64, duration_secs: f64) -> Vec<f64> {
        let total_samples = (sample_rate * duration_secs).round() as usize;
        let two_pi_f = 2.0 * std::f64::consts::PI * frequency;

        (0..total_samples)
            .map(|n| {
                let t = n as f64 / sample_rate;
                (two_pi_f * t).sin()
            })
            .collect()
    }

    fn diff_from_actual_frequency_smaller_than_threshold(
        result_frequency: f64,
        actual_frequency: f64,
        threshold: f64,
    ) -> bool {
        let result_diff_from_actual_freq = (result_frequency - actual_frequency).abs();
        result_diff_from_actual_freq < threshold
    }

    fn interpolation_better_than_raw_result(result: YinResult, frequency: f64) -> bool {
        let result_frequency = result.get_frequency();
        let refined_frequency = result.get_frequency_with_interpolation();
        let result_diff = (result_frequency - frequency).abs();
        let refined_diff = (refined_frequency - frequency).abs();
        refined_diff < result_diff
    }

    #[test]
    fn test_simple_sine() {
        let sample_rate = 1000.0;
        let frequency = 12.0;
        let seconds = 10.0;
        let signal = generate_sine_wave(frequency, sample_rate, seconds);

        let min_expected_frequency = 10.0;
        let max_expected_frequency = 100.0;

        let yin = Yin::init(
            0.1,
            min_expected_frequency,
            max_expected_frequency,
            sample_rate,
        );

        let result = yin.yin(signal.as_slice());
        assert!(result.is_ok());
        let yin_result = result.unwrap();

        assert!(diff_from_actual_frequency_smaller_than_threshold(
            yin_result.get_frequency(),
            frequency,
            1.0
        ));
        assert!(diff_from_actual_frequency_smaller_than_threshold(
            yin_result.get_frequency_with_interpolation(),
            frequency,
            1.0,
        ));

        assert!(interpolation_better_than_raw_result(yin_result, frequency));
    }

    #[test]
    fn test_sine_frequency_range() {
        let sample_rate = 5000.0;
        for freq in 30..50 {
            let frequency = freq as f64;
            let seconds = 2.0;
            let signal = generate_sine_wave(frequency, sample_rate, seconds);

            let min_expected_frequency = 5.0;
            let max_expected_frequency = 100.0;

            let yin = Yin::init(
                0.1,
                min_expected_frequency,
                max_expected_frequency,
                sample_rate,
            );
            let result = yin.yin(signal.as_slice());
            assert!(result.is_ok());
            let yin_result = result.unwrap();

            if (sample_rate as i32 % freq) == 0 {
                assert_eq!(yin_result.get_frequency(), frequency);
            } else {
                assert!(diff_from_actual_frequency_smaller_than_threshold(
                    yin_result.get_frequency(),
                    frequency,
                    1.0
                ));
                assert!(diff_from_actual_frequency_smaller_than_threshold(
                    yin_result.get_frequency_with_interpolation(),
                    frequency,
                    1.0,
                ));

                assert!(interpolation_better_than_raw_result(yin_result, frequency));
            }
        }
    }

    #[test]
    fn test_harmonic_sines() {
        let sample_rate = 44100.0;
        let seconds = 2.0;
        let frequency_1 = 50.0; // Minimal/Fundamental frequency - this is what YIN should find
        let signal_1 = generate_sine_wave(frequency_1, sample_rate, seconds);
        let frequency_2 = 150.0;
        let signal_2 = generate_sine_wave(frequency_2, sample_rate, seconds);
        let frequency_3 = 300.0;
        let signal_3 = generate_sine_wave(frequency_3, sample_rate, seconds);

        let min_expected_frequency = 10.0;
        let max_expected_frequency = 500.0;

        let yin = Yin::init(
            0.1,
            min_expected_frequency,
            max_expected_frequency,
            sample_rate,
        );

        let total_samples = (sample_rate * seconds).round() as usize;
        let combined_signal: Vec<f64> = (0..total_samples)
            .map(|n| signal_1[n] + signal_2[n] + signal_3[n])
            .collect();

        let result = yin.yin(&combined_signal);
        assert!(result.is_ok());
        let yin_result = result.unwrap();

        assert!(diff_from_actual_frequency_smaller_than_threshold(
            yin_result.get_frequency(),
            frequency_1,
            1.0
        ));
    }

    #[test]
    fn test_unharmonic_sines() {
        let sample_rate = 44100.0;
        let seconds = 2.0;
        let frequency_1 = 50.0;
        let signal_1 = generate_sine_wave(frequency_1, sample_rate, seconds);
        let frequency_2 = 66.0;
        let signal_2 = generate_sine_wave(frequency_2, sample_rate, seconds);
        let frequency_3 = 300.0;
        let signal_3 = generate_sine_wave(frequency_3, sample_rate, seconds);

        let min_expected_frequency = 10.0;
        let max_expected_frequency = 500.0;

        let yin = Yin::init(
            0.1,
            min_expected_frequency,
            max_expected_frequency,
            sample_rate,
        );

        let total_samples = (sample_rate * seconds).round() as usize;
        let combined_signal: Vec<f64> = (0..total_samples)
            .map(|n| signal_1[n] + signal_2[n] + signal_3[n])
            .collect();

        let result = yin.yin(&combined_signal);
        assert!(result.is_ok());
        let yin_result = result.unwrap();

        let expected_frequency = (frequency_1 - frequency_2).abs();
        assert!(diff_from_actual_frequency_smaller_than_threshold(
            yin_result.get_frequency(),
            expected_frequency,
            1.0
        ));
        assert!(interpolation_better_than_raw_result(
            yin_result,
            expected_frequency
        ));
    }

    #[test]
    fn test_err() {
        let sample_rate = 2500.0;
        let seconds = 2.0;
        let frequency = 440.0;

        // Can't find frequency 440 between 500 and 700
        let min_expected_frequency = 500.0;
        let max_expected_frequency = 700.0;
        let yin = Yin::init(
            0.1,
            min_expected_frequency,
            max_expected_frequency,
            sample_rate,
        );

        let signal = generate_sine_wave(frequency, sample_rate, seconds);
        let result = yin.yin(&signal);
        assert!(result.is_err());

        let yin_with_suitable_frequency_range = Yin::init(
            0.1,
            min_expected_frequency - 100.0,
            max_expected_frequency,
            sample_rate,
        );
        let result = yin_with_suitable_frequency_range.yin(&signal);
        assert!(result.is_ok());
    }
}
