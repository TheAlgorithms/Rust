//! Convert between different units of temperature
//!
//! Supports conversions between 8 temperature scales:
//! - Kelvin (K) - SI base unit, absolute scale
//! - Celsius (°C) - Standard metric scale
//! - Fahrenheit (°F) - Imperial scale
//! - Rankine (°R) - Absolute Fahrenheit scale
//! - Delisle (°De) - Historical inverted scale (higher values = colder)
//! - Newton (°N) - Historical scale by Isaac Newton
//! - Réaumur (°Ré) - Historical European scale
//! - Rømer (°Rø) - Historical Danish scale

// =============================================================================
// CELSIUS CONVERSIONS
// =============================================================================

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

pub fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

pub fn celsius_to_rankine(celsius: f64) -> f64 {
    (celsius + 273.15) * 9.0 / 5.0
}

pub fn celsius_to_delisle(celsius: f64) -> f64 {
    (100.0 - celsius) * 3.0 / 2.0
}

pub fn celsius_to_newton(celsius: f64) -> f64 {
    celsius * 33.0 / 100.0
}

pub fn celsius_to_reaumur(celsius: f64) -> f64 {
    celsius * 4.0 / 5.0
}

pub fn celsius_to_romer(celsius: f64) -> f64 {
    celsius * 21.0 / 40.0 + 7.5
}

// =============================================================================
// FAHRENHEIT CONVERSIONS
// =============================================================================

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit + 459.67) * 5.0 / 9.0
}

pub fn fahrenheit_to_rankine(fahrenheit: f64) -> f64 {
    fahrenheit + 459.67
}

pub fn fahrenheit_to_delisle(fahrenheit: f64) -> f64 {
    (212.0 - fahrenheit) * 5.0 / 6.0
}

pub fn fahrenheit_to_newton(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 11.0 / 60.0
}

pub fn fahrenheit_to_reaumur(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 4.0 / 9.0
}

pub fn fahrenheit_to_romer(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 7.0 / 24.0 + 7.5
}

// =============================================================================
// KELVIN CONVERSIONS
// =============================================================================

pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

pub fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    kelvin * 9.0 / 5.0 - 459.67
}

pub fn kelvin_to_rankine(kelvin: f64) -> f64 {
    kelvin * 9.0 / 5.0
}

pub fn kelvin_to_delisle(kelvin: f64) -> f64 {
    (373.15 - kelvin) * 3.0 / 2.0
}

pub fn kelvin_to_newton(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 33.0 / 100.0
}

pub fn kelvin_to_reaumur(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 4.0 / 5.0
}

pub fn kelvin_to_romer(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 21.0 / 40.0 + 7.5
}

// =============================================================================
// RANKINE CONVERSIONS
// =============================================================================

pub fn rankine_to_celsius(rankine: f64) -> f64 {
    (rankine - 491.67) * 5.0 / 9.0
}

pub fn rankine_to_fahrenheit(rankine: f64) -> f64 {
    rankine - 459.67
}

pub fn rankine_to_kelvin(rankine: f64) -> f64 {
    rankine * 5.0 / 9.0
}

pub fn rankine_to_delisle(rankine: f64) -> f64 {
    (671.67 - rankine) * 5.0 / 6.0
}

pub fn rankine_to_newton(rankine: f64) -> f64 {
    (rankine - 491.67) * 11.0 / 60.0
}

pub fn rankine_to_reaumur(rankine: f64) -> f64 {
    (rankine - 491.67) * 4.0 / 9.0
}

pub fn rankine_to_romer(rankine: f64) -> f64 {
    (rankine - 491.67) * 7.0 / 24.0 + 7.5
}

// =============================================================================
// DELISLE CONVERSIONS
// =============================================================================

pub fn delisle_to_celsius(delisle: f64) -> f64 {
    100.0 - delisle * 2.0 / 3.0
}

pub fn delisle_to_fahrenheit(delisle: f64) -> f64 {
    212.0 - delisle * 6.0 / 5.0
}

pub fn delisle_to_kelvin(delisle: f64) -> f64 {
    373.15 - delisle * 2.0 / 3.0
}

pub fn delisle_to_rankine(delisle: f64) -> f64 {
    671.67 - delisle * 6.0 / 5.0
}

pub fn delisle_to_newton(delisle: f64) -> f64 {
    33.0 - delisle * 11.0 / 50.0
}

pub fn delisle_to_reaumur(delisle: f64) -> f64 {
    80.0 - delisle * 8.0 / 15.0
}

pub fn delisle_to_romer(delisle: f64) -> f64 {
    60.0 - delisle * 7.0 / 20.0
}

// =============================================================================
// NEWTON CONVERSIONS
// =============================================================================

pub fn newton_to_celsius(newton: f64) -> f64 {
    newton * 100.0 / 33.0
}

pub fn newton_to_fahrenheit(newton: f64) -> f64 {
    newton * 60.0 / 11.0 + 32.0
}

pub fn newton_to_kelvin(newton: f64) -> f64 {
    newton * 100.0 / 33.0 + 273.15
}

pub fn newton_to_rankine(newton: f64) -> f64 {
    newton * 60.0 / 11.0 + 491.67
}

pub fn newton_to_delisle(newton: f64) -> f64 {
    (33.0 - newton) * 50.0 / 11.0
}

pub fn newton_to_reaumur(newton: f64) -> f64 {
    newton * 80.0 / 33.0
}

pub fn newton_to_romer(newton: f64) -> f64 {
    newton * 35.0 / 22.0 + 7.5
}

// =============================================================================
// RÉAUMUR CONVERSIONS
// =============================================================================

pub fn reaumur_to_celsius(reaumur: f64) -> f64 {
    reaumur * 5.0 / 4.0
}

pub fn reaumur_to_fahrenheit(reaumur: f64) -> f64 {
    reaumur * 9.0 / 4.0 + 32.0
}

pub fn reaumur_to_kelvin(reaumur: f64) -> f64 {
    reaumur * 5.0 / 4.0 + 273.15
}

pub fn reaumur_to_rankine(reaumur: f64) -> f64 {
    reaumur * 9.0 / 4.0 + 491.67
}

pub fn reaumur_to_delisle(reaumur: f64) -> f64 {
    (80.0 - reaumur) * 15.0 / 8.0
}

pub fn reaumur_to_newton(reaumur: f64) -> f64 {
    reaumur * 33.0 / 80.0
}

pub fn reaumur_to_romer(reaumur: f64) -> f64 {
    reaumur * 21.0 / 32.0 + 7.5
}

// =============================================================================
// RØMER CONVERSIONS
// =============================================================================

pub fn romer_to_celsius(romer: f64) -> f64 {
    (romer - 7.5) * 40.0 / 21.0
}

pub fn romer_to_fahrenheit(romer: f64) -> f64 {
    (romer - 7.5) * 24.0 / 7.0 + 32.0
}

pub fn romer_to_kelvin(romer: f64) -> f64 {
    (romer - 7.5) * 40.0 / 21.0 + 273.15
}

pub fn romer_to_rankine(romer: f64) -> f64 {
    (romer - 7.5) * 24.0 / 7.0 + 491.67
}

pub fn romer_to_delisle(romer: f64) -> f64 {
    (60.0 - romer) * 20.0 / 7.0
}

pub fn romer_to_newton(romer: f64) -> f64 {
    (romer - 7.5) * 22.0 / 35.0
}

pub fn romer_to_reaumur(romer: f64) -> f64 {
    (romer - 7.5) * 32.0 / 21.0
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_celsius_conversions() {
        assert!(approx_eq(celsius_to_fahrenheit(0.0), 32.0));
        assert!(approx_eq(celsius_to_fahrenheit(100.0), 212.0));
        assert!(approx_eq(celsius_to_kelvin(0.0), 273.15));
        assert!(approx_eq(celsius_to_rankine(0.0), 491.67));
        assert!(approx_eq(celsius_to_delisle(0.0), 150.0));
        assert!(approx_eq(celsius_to_newton(0.0), 0.0));
        assert!(approx_eq(celsius_to_reaumur(0.0), 0.0));
        assert!(approx_eq(celsius_to_romer(0.0), 7.5));
    }

    #[test]
    fn test_fahrenheit_conversions() {
        assert!(approx_eq(fahrenheit_to_celsius(32.0), 0.0));
        assert!(approx_eq(fahrenheit_to_celsius(212.0), 100.0));
        assert!(approx_eq(fahrenheit_to_kelvin(32.0), 273.15));
        assert!(approx_eq(fahrenheit_to_rankine(32.0), 491.67));
        assert!(approx_eq(fahrenheit_to_delisle(32.0), 150.0));
        assert!(approx_eq(fahrenheit_to_newton(32.0), 0.0));
        assert!(approx_eq(fahrenheit_to_reaumur(32.0), 0.0));
        assert!(approx_eq(fahrenheit_to_romer(32.0), 7.5));
    }

    #[test]
    fn test_kelvin_conversions() {
        assert!(approx_eq(kelvin_to_celsius(273.15), 0.0));
        assert!(approx_eq(kelvin_to_fahrenheit(273.15), 32.0));
        assert!(approx_eq(kelvin_to_rankine(273.15), 491.67));
        assert!(approx_eq(kelvin_to_delisle(273.15), 150.0));
        assert!(approx_eq(kelvin_to_newton(273.15), 0.0));
        assert!(approx_eq(kelvin_to_reaumur(273.15), 0.0));
        assert!(approx_eq(kelvin_to_romer(273.15), 7.5));
    }

    #[test]
    fn test_rankine_conversions() {
        assert!(approx_eq(rankine_to_celsius(491.67), 0.0));
        assert!(approx_eq(rankine_to_fahrenheit(491.67), 32.0));
        assert!(approx_eq(rankine_to_kelvin(491.67), 273.15));
        assert!(approx_eq(rankine_to_delisle(491.67), 150.0));
        assert!(approx_eq(rankine_to_newton(491.67), 0.0));
        assert!(approx_eq(rankine_to_reaumur(491.67), 0.0));
        assert!(approx_eq(rankine_to_romer(491.67), 7.5));
    }

    #[test]
    fn test_delisle_conversions() {
        assert!(approx_eq(delisle_to_celsius(150.0), 0.0));
        assert!(approx_eq(delisle_to_fahrenheit(150.0), 32.0));
        assert!(approx_eq(delisle_to_kelvin(150.0), 273.15));
        assert!(approx_eq(delisle_to_rankine(150.0), 491.67));
        assert!(approx_eq(delisle_to_newton(150.0), 0.0));
        assert!(approx_eq(delisle_to_reaumur(150.0), 0.0));
        assert!(approx_eq(delisle_to_romer(150.0), 7.5));
    }

    #[test]
    fn test_newton_conversions() {
        assert!(approx_eq(newton_to_celsius(0.0), 0.0));
        assert!(approx_eq(newton_to_fahrenheit(0.0), 32.0));
        assert!(approx_eq(newton_to_kelvin(0.0), 273.15));
        assert!(approx_eq(newton_to_rankine(0.0), 491.67));
        assert!(approx_eq(newton_to_delisle(0.0), 150.0));
        assert!(approx_eq(newton_to_reaumur(0.0), 0.0));
        assert!(approx_eq(newton_to_romer(0.0), 7.5));
    }

    #[test]
    fn test_reaumur_conversions() {
        assert!(approx_eq(reaumur_to_celsius(0.0), 0.0));
        assert!(approx_eq(reaumur_to_fahrenheit(0.0), 32.0));
        assert!(approx_eq(reaumur_to_kelvin(0.0), 273.15));
        assert!(approx_eq(reaumur_to_rankine(0.0), 491.67));
        assert!(approx_eq(reaumur_to_delisle(0.0), 150.0));
        assert!(approx_eq(reaumur_to_newton(0.0), 0.0));
        assert!(approx_eq(reaumur_to_romer(0.0), 7.5));
    }

    #[test]
    fn test_romer_conversions() {
        assert!(approx_eq(romer_to_celsius(7.5), 0.0));
        assert!(approx_eq(romer_to_fahrenheit(7.5), 32.0));
        assert!(approx_eq(romer_to_kelvin(7.5), 273.15));
        assert!(approx_eq(romer_to_rankine(7.5), 491.67));
        assert!(approx_eq(romer_to_delisle(7.5), 150.0));
        assert!(approx_eq(romer_to_newton(7.5), 0.0));
        assert!(approx_eq(romer_to_reaumur(7.5), 0.0));
    }

    #[test]
    fn test_round_trip_conversions() {
        let temp_c = 25.0;

        let temp_f = celsius_to_fahrenheit(temp_c);
        assert!(approx_eq(fahrenheit_to_celsius(temp_f), temp_c));

        let temp_k = celsius_to_kelvin(temp_c);
        assert!(approx_eq(kelvin_to_celsius(temp_k), temp_c));

        let temp_r = celsius_to_rankine(temp_c);
        assert!(approx_eq(rankine_to_celsius(temp_r), temp_c));

        let temp_de = celsius_to_delisle(temp_c);
        assert!(approx_eq(delisle_to_celsius(temp_de), temp_c));

        let temp_n = celsius_to_newton(temp_c);
        assert!(approx_eq(newton_to_celsius(temp_n), temp_c));

        let temp_re = celsius_to_reaumur(temp_c);
        assert!(approx_eq(reaumur_to_celsius(temp_re), temp_c));

        let temp_ro = celsius_to_romer(temp_c);
        assert!(approx_eq(romer_to_celsius(temp_ro), temp_c));
    }

    #[test]
    fn test_special_temperatures() {
        // Absolute zero
        assert!(approx_eq(kelvin_to_celsius(0.0), -273.15));
        assert!(approx_eq(kelvin_to_fahrenheit(0.0), -459.67));

        // Water freezing point
        assert!(approx_eq(celsius_to_fahrenheit(0.0), 32.0));
        assert!(approx_eq(celsius_to_kelvin(0.0), 273.15));

        // Water boiling point
        assert!(approx_eq(celsius_to_fahrenheit(100.0), 212.0));
        assert!(approx_eq(celsius_to_kelvin(100.0), 373.15));

        // Celsius equals Fahrenheit
        assert!(approx_eq(celsius_to_fahrenheit(-40.0), -40.0));
    }

    #[test]
    fn test_historical_scales() {
        // Delisle (inverted scale)
        assert!(approx_eq(celsius_to_delisle(100.0), 0.0));
        assert!(approx_eq(delisle_to_celsius(0.0), 100.0));

        // Newton scale
        assert!(approx_eq(celsius_to_newton(100.0), 33.0));
        assert!(approx_eq(newton_to_celsius(33.0), 100.0));

        // Rømer scale
        assert!(approx_eq(celsius_to_romer(100.0), 60.0));
        assert!(approx_eq(romer_to_celsius(60.0), 100.0));
    }
}
