//! Conversions between rectangular (Cartesian) and polar coordinate systems.
//!
//! This module provides utilities for converting rectangular coordinates
//! into polar coordinates with angles expressed in degrees.
//!
//! More information: <https://en.wikipedia.org/wiki/Polar_coordinate_system>

/// Convert rectangular (Cartesian) coordinates to polar coordinates.
///
/// The returned tuple contains:
/// - magnitude (r)
/// - angle (θ) in degrees
///
/// Both values are rounded to 2 decimal places.
///
/// # Formula
/// - r = sqrt(x² + y²)
/// - θ = atan2(y, x) converted to degrees
pub fn rectangular_to_polar(real: f64, imag: f64) -> (f64, f64) {
    let magnitude = (real.powi(2) + imag.powi(2)).sqrt();
    let angle = imag.atan2(real).to_degrees();

    (
        round_to_two_decimals(magnitude),
        round_to_two_decimals(angle),
    )
}

fn round_to_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangular_to_polar() {
        assert_eq!(rectangular_to_polar(5.0, -5.0), (7.07, -45.0));
        assert_eq!(rectangular_to_polar(-1.0, 1.0), (1.41, 135.0));
        assert_eq!(rectangular_to_polar(-1.0, -1.0), (1.41, -135.0));
        assert_eq!(rectangular_to_polar(1e-10, 1e-10), (0.0, 45.0));
        assert_eq!(rectangular_to_polar(-1e-10, 1e-10), (0.0, 135.0));
        assert_eq!(rectangular_to_polar(9.75, 5.93), (11.41, 31.31));
        assert_eq!(rectangular_to_polar(10000.0, 99999.0), (100497.76, 84.29));
    }
}
