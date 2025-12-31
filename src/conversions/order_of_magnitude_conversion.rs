//! Length Unit Conversion
//!
//! This module provides conversion between metric length units ranging from
//! meters to yottametres (10^24 meters).
//!
//! Available units: Meter, Kilometer, Megametre, Gigametre, Terametre,
//! Petametre, Exametre, Zettametre, Yottametre
//!
//! # References
//!
//! - [Meter - Wikipedia](https://en.wikipedia.org/wiki/Meter)
//! - [Kilometer - Wikipedia](https://en.wikipedia.org/wiki/Kilometer)
//! - [Orders of Magnitude (Length) - Wikipedia](https://en.wikipedia.org/wiki/Orders_of_magnitude_(length))

use std::fmt;
use std::str::FromStr;

/// Represents different metric length units.
///
/// Each variant corresponds to a specific power of 10 relative to the base unit (meter).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricLengthUnit {
    /// Meter (m) - base unit, 10^0 meters
    Meter,
    /// Kilometer (km) - 10^3 meters
    Kilometer,
    /// Megametre (Mm) - 10^6 meters
    Megametre,
    /// Gigametre (Gm) - 10^9 meters
    Gigametre,
    /// Terametre (Tm) - 10^12 meters
    Terametre,
    /// Petametre (Pm) - 10^15 meters
    Petametre,
    /// Exametre (Em) - 10^18 meters
    Exametre,
    /// Zettametre (Zm) - 10^21 meters
    Zettametre,
    /// Yottametre (Ym) - 10^24 meters
    Yottametre,
}

impl MetricLengthUnit {
    /// Returns the exponent (power of 10) for this unit relative to meters.
    ///
    /// # Example
    ///
    /// ```
    /// use the_algorithms_rust::conversions::MetricLengthUnit;
    ///
    /// assert_eq!(MetricLengthUnit::Meter.exponent(), 0);
    /// assert_eq!(MetricLengthUnit::Kilometer.exponent(), 3);
    /// assert_eq!(MetricLengthUnit::Megametre.exponent(), 6);
    /// ```
    pub fn exponent(&self) -> i32 {
        match self {
            MetricLengthUnit::Meter => 0,
            MetricLengthUnit::Kilometer => 3,
            MetricLengthUnit::Megametre => 6,
            MetricLengthUnit::Gigametre => 9,
            MetricLengthUnit::Terametre => 12,
            MetricLengthUnit::Petametre => 15,
            MetricLengthUnit::Exametre => 18,
            MetricLengthUnit::Zettametre => 21,
            MetricLengthUnit::Yottametre => 24,
        }
    }

    /// Returns the standard abbreviation for this unit.
    ///
    /// # Example
    ///
    /// ```
    /// use the_algorithms_rust::conversions::MetricLengthUnit;
    ///
    /// assert_eq!(MetricLengthUnit::Meter.symbol(), "m");
    /// assert_eq!(MetricLengthUnit::Kilometer.symbol(), "km");
    /// ```
    pub fn symbol(&self) -> &'static str {
        match self {
            MetricLengthUnit::Meter => "m",
            MetricLengthUnit::Kilometer => "km",
            MetricLengthUnit::Megametre => "Mm",
            MetricLengthUnit::Gigametre => "Gm",
            MetricLengthUnit::Terametre => "Tm",
            MetricLengthUnit::Petametre => "Pm",
            MetricLengthUnit::Exametre => "Em",
            MetricLengthUnit::Zettametre => "Zm",
            MetricLengthUnit::Yottametre => "Ym",
        }
    }
}

impl FromStr for MetricLengthUnit {
    type Err = String;

    /// Parses a unit from a string (case-insensitive, handles plurals).
    ///
    /// Accepts both full names (e.g., "meter", "meters") and symbols (e.g., "m", "km").
    ///
    /// # Example
    ///
    /// ```
    /// use the_algorithms_rust::conversions::MetricLengthUnit;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(MetricLengthUnit::from_str("meter").unwrap(), MetricLengthUnit::Meter);
    /// assert_eq!(MetricLengthUnit::from_str("km").unwrap(), MetricLengthUnit::Kilometer);
    /// assert_eq!(MetricLengthUnit::from_str("METERS").unwrap(), MetricLengthUnit::Meter);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Sanitize: lowercase and remove trailing 's'
        let sanitized = s.to_lowercase().trim_end_matches('s').to_string();

        match sanitized.as_str() {
            "meter" | "m" => Ok(MetricLengthUnit::Meter),
            "kilometer" | "km" => Ok(MetricLengthUnit::Kilometer),
            "megametre" | "megameter" | "mm" => Ok(MetricLengthUnit::Megametre),
            "gigametre" | "gigameter" | "gm" => Ok(MetricLengthUnit::Gigametre),
            "terametre" | "terameter" | "tm" => Ok(MetricLengthUnit::Terametre),
            "petametre" | "petameter" | "pm" => Ok(MetricLengthUnit::Petametre),
            "exametre" | "exameter" | "em" => Ok(MetricLengthUnit::Exametre),
            "zettametre" | "zettameter" | "zm" => Ok(MetricLengthUnit::Zettametre),
            "yottametre" | "yottameter" | "ym" => Ok(MetricLengthUnit::Yottametre),
            _ => Err(format!(
                "Invalid unit: '{s}'. Valid units are: m, km, Mm, Gm, Tm, Pm, Em, Zm, Ym"
            )),
        }
    }
}

impl fmt::Display for MetricLengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// Converts a length value from one unit to another.
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `from` - The unit to convert from
/// * `to` - The unit to convert to
///
/// # Returns
///
/// The converted value in the target unit
///
/// # Example
///
/// ```
/// use the_algorithms_rust::conversions::{MetricLengthUnit, convert_metric_length};
///
/// let result = convert_metric_length(1.0, MetricLengthUnit::Meter, MetricLengthUnit::Kilometer);
/// assert_eq!(result, 0.001);
///
/// let result = convert_metric_length(1.0, MetricLengthUnit::Kilometer, MetricLengthUnit::Meter);
/// assert_eq!(result, 1000.0);
/// ```
pub fn convert_metric_length(value: f64, from: MetricLengthUnit, to: MetricLengthUnit) -> f64 {
    let from_exp = from.exponent();
    let to_exp = to.exponent();
    let exponent = from_exp - to_exp;

    value * 10_f64.powi(exponent)
}

/// Converts a length value from one unit to another using string unit names.
///
/// This function accepts both full unit names and abbreviations, and is case-insensitive.
/// It also handles plural forms (e.g., "meters" and "meter" both work).
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `from_type` - The unit to convert from (as a string)
/// * `to_type` - The unit to convert to (as a string)
///
/// # Returns
///
/// `Ok(f64)` with the converted value, or `Err(String)` if either unit is invalid
///
/// # Example
///
/// ```
/// use the_algorithms_rust::conversions::metric_length_conversion;
///
/// let result = metric_length_conversion(1.0, "meter", "kilometer").unwrap();
/// assert_eq!(result, 0.001);
///
/// let result = metric_length_conversion(1.0, "km", "m").unwrap();
/// assert_eq!(result, 1000.0);
///
/// // Case insensitive and handles plurals
/// let result = metric_length_conversion(5.0, "METERS", "kilometers").unwrap();
/// assert_eq!(result, 0.005);
///
/// // Invalid unit returns error
/// assert!(metric_length_conversion(1.0, "wrongUnit", "meter").is_err());
/// ```
pub fn metric_length_conversion(value: f64, from_type: &str, to_type: &str) -> Result<f64, String> {
    let from_unit = MetricLengthUnit::from_str(from_type).map_err(|_| {
        format!(
            "Invalid 'from_type' value: '{from_type}'.\nConversion abbreviations are: m, km, Mm, Gm, Tm, Pm, Em, Zm, Ym"
        )
    })?;

    let to_unit = MetricLengthUnit::from_str(to_type).map_err(|_| {
        format!(
            "Invalid 'to_type' value: '{to_type}'.\nConversion abbreviations are: m, km, Mm, Gm, Tm, Pm, Em, Zm, Ym"
        )
    })?;

    Ok(convert_metric_length(value, from_unit, to_unit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_exponents() {
        assert_eq!(MetricLengthUnit::Meter.exponent(), 0);
        assert_eq!(MetricLengthUnit::Kilometer.exponent(), 3);
        assert_eq!(MetricLengthUnit::Megametre.exponent(), 6);
        assert_eq!(MetricLengthUnit::Gigametre.exponent(), 9);
        assert_eq!(MetricLengthUnit::Terametre.exponent(), 12);
        assert_eq!(MetricLengthUnit::Petametre.exponent(), 15);
        assert_eq!(MetricLengthUnit::Exametre.exponent(), 18);
        assert_eq!(MetricLengthUnit::Zettametre.exponent(), 21);
        assert_eq!(MetricLengthUnit::Yottametre.exponent(), 24);
    }

    #[test]
    fn test_unit_symbols() {
        assert_eq!(MetricLengthUnit::Meter.symbol(), "m");
        assert_eq!(MetricLengthUnit::Kilometer.symbol(), "km");
        assert_eq!(MetricLengthUnit::Megametre.symbol(), "Mm");
        assert_eq!(MetricLengthUnit::Gigametre.symbol(), "Gm");
        assert_eq!(MetricLengthUnit::Terametre.symbol(), "Tm");
        assert_eq!(MetricLengthUnit::Petametre.symbol(), "Pm");
        assert_eq!(MetricLengthUnit::Exametre.symbol(), "Em");
        assert_eq!(MetricLengthUnit::Zettametre.symbol(), "Zm");
        assert_eq!(MetricLengthUnit::Yottametre.symbol(), "Ym");
    }

    #[test]
    fn test_from_str_full_names() {
        assert_eq!(
            MetricLengthUnit::from_str("meter").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("kilometer").unwrap(),
            MetricLengthUnit::Kilometer
        );
        assert_eq!(
            MetricLengthUnit::from_str("megametre").unwrap(),
            MetricLengthUnit::Megametre
        );
    }

    #[test]
    fn test_from_str_symbols() {
        assert_eq!(
            MetricLengthUnit::from_str("m").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("km").unwrap(),
            MetricLengthUnit::Kilometer
        );
        assert_eq!(
            MetricLengthUnit::from_str("Mm").unwrap(),
            MetricLengthUnit::Megametre
        );
        assert_eq!(
            MetricLengthUnit::from_str("Gm").unwrap(),
            MetricLengthUnit::Gigametre
        );
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!(
            MetricLengthUnit::from_str("METER").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("KiLoMeTeR").unwrap(),
            MetricLengthUnit::Kilometer
        );
        assert_eq!(
            MetricLengthUnit::from_str("KM").unwrap(),
            MetricLengthUnit::Kilometer
        );
    }

    #[test]
    fn test_from_str_plurals() {
        assert_eq!(
            MetricLengthUnit::from_str("meters").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("kilometers").unwrap(),
            MetricLengthUnit::Kilometer
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(MetricLengthUnit::from_str("wrongUnit").is_err());
        assert!(MetricLengthUnit::from_str("inch").is_err());
        assert!(MetricLengthUnit::from_str("").is_err());
    }

    #[test]
    fn test_convert_length_meter_to_kilometer() {
        let result =
            convert_metric_length(1.0, MetricLengthUnit::Meter, MetricLengthUnit::Kilometer);
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_convert_length_meter_to_megametre() {
        let result =
            convert_metric_length(1.0, MetricLengthUnit::Meter, MetricLengthUnit::Megametre);
        assert_eq!(result, 1e-6);
    }

    #[test]
    fn test_convert_length_gigametre_to_meter() {
        let result =
            convert_metric_length(1.0, MetricLengthUnit::Gigametre, MetricLengthUnit::Meter);
        assert_eq!(result, 1_000_000_000.0);
    }

    #[test]
    fn test_convert_length_gigametre_to_terametre() {
        let result = convert_metric_length(
            1.0,
            MetricLengthUnit::Gigametre,
            MetricLengthUnit::Terametre,
        );
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_convert_length_petametre_to_terametre() {
        let result = convert_metric_length(
            1.0,
            MetricLengthUnit::Petametre,
            MetricLengthUnit::Terametre,
        );
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_convert_length_petametre_to_exametre() {
        let result =
            convert_metric_length(1.0, MetricLengthUnit::Petametre, MetricLengthUnit::Exametre);
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_convert_length_terametre_to_zettametre() {
        let result = convert_metric_length(
            1.0,
            MetricLengthUnit::Terametre,
            MetricLengthUnit::Zettametre,
        );
        assert_eq!(result, 1e-9);
    }

    #[test]
    fn test_convert_length_yottametre_to_zettametre() {
        let result = convert_metric_length(
            1.0,
            MetricLengthUnit::Yottametre,
            MetricLengthUnit::Zettametre,
        );
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_convert_length_same_unit() {
        let result = convert_metric_length(
            42.0,
            MetricLengthUnit::Kilometer,
            MetricLengthUnit::Kilometer,
        );
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_length_conversion_str_basic() {
        let result = metric_length_conversion(1.0, "meter", "kilometer").unwrap();
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_length_conversion_str_symbols() {
        let result = metric_length_conversion(1.0, "m", "km").unwrap();
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_length_conversion_str_case_insensitive() {
        let result = metric_length_conversion(1.0, "METER", "KILOMETER").unwrap();
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_length_conversion_str_plurals() {
        let result = metric_length_conversion(5.0, "meters", "kilometers").unwrap();
        assert_eq!(result, 0.005);
    }

    #[test]
    fn test_length_conversion_str_invalid_from() {
        let result = metric_length_conversion(1.0, "wrongUnit", "meter");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid 'from_type'"));
    }

    #[test]
    fn test_length_conversion_str_invalid_to() {
        let result = metric_length_conversion(1.0, "meter", "inch");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid 'to_type'"));
    }

    #[test]
    fn test_length_conversion_str_large_values() {
        let result = metric_length_conversion(1000.0, "km", "m").unwrap();
        assert_eq!(result, 1_000_000.0);
    }

    #[test]
    fn test_length_conversion_str_small_values() {
        let result = metric_length_conversion(0.001, "m", "km").unwrap();
        assert_eq!(result, 0.000001);
    }

    #[test]
    fn test_all_conversions_reversible() {
        let units = [
            MetricLengthUnit::Meter,
            MetricLengthUnit::Kilometer,
            MetricLengthUnit::Megametre,
            MetricLengthUnit::Gigametre,
            MetricLengthUnit::Terametre,
        ];

        for &from in &units {
            for &to in &units {
                let forward = convert_metric_length(100.0, from, to);
                let backward = convert_metric_length(forward, to, from);
                assert!((backward - 100.0).abs() < 1e-9, "Conversion not reversible");
            }
        }
    }
}
