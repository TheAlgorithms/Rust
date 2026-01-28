//! Conversion of pressure units.
//!
//! This module provides conversion between various pressure units including:
//! Pascal (Pa, kPa, MPa, GPa), Bar (bar, mbar), Atmosphere (atm, at, ata),
//! Torr (Torr, mTorr), PSI (psi, ksi), Barad (Ba), Pièze (pz),
//! and manometric units (mmHg, cmHg, inHg, mmH2O, cmH2O, inH2O, msw, fsw).
//!
//! # References
//! - [Units of Pressure](https://msestudent.com/what-are-the-units-of-pressure/)

use std::fmt;
use std::str::FromStr;

/// Trait for types that can be converted into a PressureUnit
pub trait IntoPressureUnit {
    fn into_pressure_unit(self) -> Result<PressureUnit, String>;
}

impl IntoPressureUnit for PressureUnit {
    fn into_pressure_unit(self) -> Result<PressureUnit, String> {
        Ok(self)
    }
}

impl IntoPressureUnit for &str {
    fn into_pressure_unit(self) -> Result<PressureUnit, String> {
        PressureUnit::from_str(self)
    }
}

impl IntoPressureUnit for String {
    fn into_pressure_unit(self) -> Result<PressureUnit, String> {
        PressureUnit::from_str(&self)
    }
}

/// Supported pressure units
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureUnit {
    // SI units (Pascal-based)
    Pascal,
    Kilopascal,
    Megapascal,
    Gigapascal,
    Hectopascal,

    // Atmosphere units
    Atmosphere,
    TechnicalAtmosphere,
    TotalAtmosphere,

    // Torr units
    Torr,
    Millitorr,

    // Bar units
    Bar,
    Millibar,

    // Imperial units
    Psi,
    Ksi,
    OunceForcePerSquareInch,

    // Other metric units
    Barad,
    Pieze,

    // Manometric units
    MillimeterMercury,
    CentimeterMercury,
    InchMercury,
    MillimeterWater,
    CentimeterWater,
    InchWater,
    MeterSeawater,
    FootSeawater,
}

impl fmt::Display for PressureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pascal => "Pa",
            Self::Kilopascal => "kPa",
            Self::Megapascal => "MPa",
            Self::Gigapascal => "GPa",
            Self::Hectopascal => "hPa",
            Self::Atmosphere => "atm",
            Self::TechnicalAtmosphere => "at",
            Self::TotalAtmosphere => "ata",
            Self::Torr => "Torr",
            Self::Millitorr => "mTorr",
            Self::Bar => "bar",
            Self::Millibar => "mbar",
            Self::Psi => "psi",
            Self::Ksi => "ksi",
            Self::OunceForcePerSquareInch => "ozf/in²",
            Self::Barad => "Ba",
            Self::Pieze => "pz",
            Self::MillimeterMercury => "mmHg",
            Self::CentimeterMercury => "cmHg",
            Self::InchMercury => "inHg",
            Self::MillimeterWater => "mmH₂O",
            Self::CentimeterWater => "cmH₂O",
            Self::InchWater => "inH₂O",
            Self::MeterSeawater => "msw",
            Self::FootSeawater => "fsw",
        };
        write!(f, "{s}")
    }
}

impl PressureUnit {
    /// Get the conversion factor to convert this unit to pascals
    fn to_pascal_factor(self) -> f64 {
        match self {
            // SI units (Pascal-based)
            Self::Pascal => 1.0,
            Self::Kilopascal | Self::Pieze => 1_000.0,
            Self::Megapascal => 1_000_000.0,
            Self::Gigapascal => 1_000_000_000.0,
            Self::Hectopascal | Self::Millibar => 100.0,

            // Atmosphere units
            Self::Atmosphere | Self::TotalAtmosphere => 101_325.0,
            Self::TechnicalAtmosphere => 98_070.0,

            // Torr units (1 atm = 760 Torr exactly)
            Self::Torr | Self::MillimeterMercury => 101_325.0 / 760.0,
            Self::Millitorr => 101_325.0 / 760_000.0,

            // Bar units
            Self::Bar => 100_000.0,

            // Imperial units
            Self::Psi => 6_894.757_293_168,
            Self::Ksi => 6_894_757.293_168,
            Self::OunceForcePerSquareInch => 430.922_330_823,

            // Other metric units
            Self::Barad => 0.1,

            // Manometric units
            Self::CentimeterMercury => 101_325.0 / 76.0,
            Self::InchMercury => 3_386.389,
            Self::MillimeterWater => 9.806_65,
            Self::CentimeterWater => 98.0665,
            Self::InchWater => 249.088_908_333,
            Self::MeterSeawater => 10_000.0,
            Self::FootSeawater => 3_048.0,
        }
    }

    /// Get all supported units as strings
    pub fn supported_units() -> Vec<&'static str> {
        vec![
            "Pa", "kPa", "MPa", "GPa", "hPa", "atm", "at", "ata", "Torr", "mTorr", "bar", "mbar",
            "psi", "ksi", "ozf/in²", "Ba", "pz", "mmHg", "cmHg", "inHg", "mmH₂O", "cmH₂O", "inH₂O",
            "msw", "fsw",
        ]
    }
}

impl FromStr for PressureUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit = match s.to_lowercase().as_str() {
            "pa" | "pascal" => Self::Pascal,
            "kpa" | "kilopascal" => Self::Kilopascal,
            "mpa" | "megapascal" => Self::Megapascal,
            "gpa" | "gigapascal" => Self::Gigapascal,
            "hpa" | "hectopascal" => Self::Hectopascal,
            "atm" | "atmosphere" => Self::Atmosphere,
            "at" | "technical_atmosphere" | "kgf/cm2" => Self::TechnicalAtmosphere,
            "ata" | "total_atmosphere" => Self::TotalAtmosphere,
            "torr" => Self::Torr,
            "mtorr" | "millitorr" => Self::Millitorr,
            "bar" => Self::Bar,
            "mbar" | "millibar" => Self::Millibar,
            "psi" | "lb/in2" => Self::Psi,
            "ksi" => Self::Ksi,
            "ozf/in2" | "ounce_force_per_square_inch" => Self::OunceForcePerSquareInch,
            "ba" | "barad" => Self::Barad,
            "pz" | "pieze" => Self::Pieze,
            "mmhg" | "millimeter_mercury" => Self::MillimeterMercury,
            "cmhg" | "centimeter_mercury" => Self::CentimeterMercury,
            "inhg" | "inch_mercury" => Self::InchMercury,
            "mmh2o" | "millimeter_water" => Self::MillimeterWater,
            "cmh2o" | "centimeter_water" => Self::CentimeterWater,
            "inh2o" | "inch_water" => Self::InchWater,
            "msw" | "meter_seawater" => Self::MeterSeawater,
            "fsw" | "foot_seawater" => Self::FootSeawater,
            _ => return Err(format!("Unknown pressure unit: {s}")),
        };
        Ok(unit)
    }
}

/// Convert pressure from one unit to another.
///
/// This function accepts both `PressureUnit` enums and string identifiers.
///
/// # Arguments
///
/// * `value` - The numerical value to convert
/// * `from_unit` - The unit to convert from (can be a `PressureUnit` enum or a string)
/// * `to_unit` - The unit to convert to (can be a `PressureUnit` enum or a string)
///
/// # Returns
///
/// The converted value, or an error if the unit is invalid
///
/// # Examples
///
/// Using enums (type-safe):
/// ```ignore
/// let result = convert_pressure(100.0, PressureUnit::Psi, PressureUnit::Kilopascal);
/// ```
///
/// Using strings (convenient):
/// ```ignore
/// let result = convert_pressure(100.0, "psi", "kpa");
/// ```
pub fn convert_pressure<F, T>(value: f64, from_unit: F, to_unit: T) -> Result<f64, String>
where
    F: IntoPressureUnit,
    T: IntoPressureUnit,
{
    let from = from_unit.into_pressure_unit().map_err(|_| {
        format!(
            "Invalid 'from_unit' value. Supported values are:\n{}",
            PressureUnit::supported_units().join(", ")
        )
    })?;

    let to = to_unit.into_pressure_unit().map_err(|_| {
        format!(
            "Invalid 'to_unit' value. Supported values are:\n{}",
            PressureUnit::supported_units().join(", ")
        )
    })?;

    // Convert to pascals first, then to target unit
    let pascals = value * from.to_pascal_factor();
    Ok(pascals / to.to_pascal_factor())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-3; // Increased tolerance for floating point comparisons

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_pressure_conversions() {
        // Test basic conversions from Python original (using strings)
        assert!(approx_eq(
            convert_pressure(4.0, "atm", "pascal").unwrap(),
            405_300.0
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "pascal", "psi").unwrap(),
            0.000_145_037_738
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "bar", "atm").unwrap(),
            0.986_923_266_716
        ));
        assert!(approx_eq(
            convert_pressure(3.0, "kilopascal", "bar").unwrap(),
            0.03
        ));
        assert!(approx_eq(
            convert_pressure(2.0, "megapascal", "psi").unwrap(),
            290.075_476
        ));
        assert!(approx_eq(
            convert_pressure(4.0, "psi", "torr").unwrap(),
            206.859_730
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "inhg", "atm").unwrap(),
            0.033_421_052
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "torr", "psi").unwrap(),
            0.019_336_775
        ));

        // Test using enums (type-safe)
        assert!(approx_eq(
            convert_pressure(1.0, PressureUnit::Atmosphere, PressureUnit::Pascal).unwrap(),
            101_325.0
        ));
        assert!(approx_eq(
            convert_pressure(100.0, PressureUnit::Psi, PressureUnit::Kilopascal).unwrap(),
            689.475_729
        ));

        // Test mixed usage (enum and string)
        assert!(approx_eq(
            convert_pressure(1.0, PressureUnit::Bar, "atm").unwrap(),
            0.986_923_266_716
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "bar", PressureUnit::Atmosphere).unwrap(),
            0.986_923_266_716
        ));

        // Test invalid units
        assert!(convert_pressure(4.0, "wrongUnit", "atm").is_err());
        assert!(convert_pressure(4.0, "atm", "wrongUnit").is_err());

        // Test atmospheric pressure conversions
        assert!(approx_eq(
            convert_pressure(1.0, "atm", "pascal").unwrap(),
            101_325.0
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "atm", "bar").unwrap(),
            1.01325
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "atm", "torr").unwrap(),
            760.0
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "atm", "psi").unwrap(),
            14.695_949
        ));

        // Test roundtrip conversion
        let original = 100.0;
        let converted = convert_pressure(original, "psi", "kpa").unwrap();
        let back = convert_pressure(converted, "kpa", "psi").unwrap();
        assert!(approx_eq(original, back));

        // Test manometric units
        assert!(approx_eq(
            convert_pressure(760.0, "mmhg", "atm").unwrap(),
            1.0
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "mmh2o", "pascal").unwrap(),
            9.80665
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "msw", "kpa").unwrap(),
            10.0
        ));
        assert!(approx_eq(
            convert_pressure(1.0, "fsw", "pascal").unwrap(),
            3_048.0
        ));

        // Test technical atmosphere
        assert!(approx_eq(
            convert_pressure(1.0, "at", "atm").unwrap(),
            0.967_841_105
        ));

        // Test ksi conversion
        assert!(approx_eq(
            convert_pressure(1.0, "ksi", "psi").unwrap(),
            1_000.0
        ));

        // Test gigapascal conversion
        assert!(approx_eq(
            convert_pressure(1.0, "gpa", "mpa").unwrap(),
            1_000.0
        ));

        // Test hectopascal equals millibar
        let hpa_to_pa = convert_pressure(1.0, "hpa", "pa").unwrap();
        let mbar_to_pa = convert_pressure(1.0, "mbar", "pa").unwrap();
        assert!(approx_eq(hpa_to_pa, mbar_to_pa));

        // Test barad conversion
        assert!(approx_eq(convert_pressure(1.0, "ba", "pa").unwrap(), 0.1));

        // Test pieze conversion
        assert!(approx_eq(convert_pressure(1.0, "pz", "kpa").unwrap(), 1.0));
    }
}
