//! Conversion of weight units.
//!
//! This module provides conversion between various weight units including:
//! - Metric: Gigatonne (Gt), Megatonne (Mt), Metric Ton (t), Kilogram (kg), Gram (g),
//!   Milligram (mg), Microgram (μg), Nanogram (ng), Picogram (pg)
//! - Imperial/US: Long Ton, Short Ton, Hundredweight (cwt), Quarter (qtr), Stone (st),
//!   Pound (lb), Ounce (oz), Dram (dr), Grain (gr)
//! - Troy: Troy Pound (lb t), Troy Ounce (oz t), Pennyweight (dwt)
//! - Other: Carat (ct), Atomic Mass Unit (amu)
//!
//! # References
//! - [Kilogram](https://en.wikipedia.org/wiki/Kilogram)
//! - [Gram](https://en.wikipedia.org/wiki/Gram)
//! - [Milligram](https://en.wikipedia.org/wiki/Milligram)
//! - [Microgram](https://en.wikipedia.org/wiki/Microgram)
//! - [Nanogram](https://en.wikipedia.org/wiki/Orders_of_magnitude_(mass))
//! - [Picogram](https://en.wikipedia.org/wiki/Orders_of_magnitude_(mass))
//! - [Tonne](https://en.wikipedia.org/wiki/Tonne)
//! - [Gigatonne](https://en.wikipedia.org/wiki/Tonne#Derived_units)
//! - [Megatonne](https://en.wikipedia.org/wiki/Tonne#Derived_units)
//! - [Long Ton](https://en.wikipedia.org/wiki/Long_ton)
//! - [Short Ton](https://en.wikipedia.org/wiki/Short_ton)
//! - [Pound](https://en.wikipedia.org/wiki/Pound_(mass))
//! - [Ounce](https://en.wikipedia.org/wiki/Ounce)
//! - [Stone](https://en.wikipedia.org/wiki/Stone_(unit))
//! - [Quarter](https://en.wikipedia.org/wiki/Quarter_(unit))
//! - [Hundredweight](https://en.wikipedia.org/wiki/Hundredweight)
//! - [Grain](https://en.wikipedia.org/wiki/Grain_(unit))
//! - [Dram](https://en.wikipedia.org/wiki/Dram_(unit))
//! - [Troy Pound](https://en.wikipedia.org/wiki/Troy_weight)
//! - [Troy Ounce](https://en.wikipedia.org/wiki/Troy_weight)
//! - [Pennyweight](https://en.wikipedia.org/wiki/Pennyweight)
//! - [Carat](https://en.wikipedia.org/wiki/Carat_(mass))
//! - [Dalton (Atomic Mass Unit)](https://en.wikipedia.org/wiki/Dalton_(unit))

use std::fmt;
use std::str::FromStr;

/// Trait for types that can be converted into a WeightUnit
pub trait IntoWeightUnit {
    fn into_weight_unit(self) -> Result<WeightUnit, String>;
}

impl IntoWeightUnit for WeightUnit {
    fn into_weight_unit(self) -> Result<WeightUnit, String> {
        Ok(self)
    }
}

impl IntoWeightUnit for &str {
    fn into_weight_unit(self) -> Result<WeightUnit, String> {
        WeightUnit::from_str(self)
    }
}

impl IntoWeightUnit for String {
    fn into_weight_unit(self) -> Result<WeightUnit, String> {
        WeightUnit::from_str(&self)
    }
}

/// Supported weight units
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeightUnit {
    // Large metric units
    Gigatonne,
    Megatonne,
    MetricTon,

    // Standard metric units
    Kilogram,
    Gram,
    Milligram,
    Microgram,
    Nanogram,
    Picogram,

    // Imperial/US tons and large units
    LongTon,
    ShortTon,
    Hundredweight,
    Quarter,

    // Imperial/US common units
    Stone,
    Pound,
    Ounce,
    Dram,
    Grain,

    // Troy weight system
    TroyPound,
    TroyOunce,
    Pennyweight,

    // Other units
    Carat,
    AtomicMassUnit,
}

impl fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            // Large metric units
            Self::Gigatonne => "Gt",
            Self::Megatonne => "Mt",
            Self::MetricTon => "t",

            // Standard metric units
            Self::Kilogram => "kg",
            Self::Gram => "g",
            Self::Milligram => "mg",
            Self::Microgram => "μg",
            Self::Nanogram => "ng",
            Self::Picogram => "pg",

            // Imperial/US tons and large units
            Self::LongTon => "long ton",
            Self::ShortTon => "short ton",
            Self::Hundredweight => "cwt",
            Self::Quarter => "qtr",

            // Imperial/US common units
            Self::Stone => "st",
            Self::Pound => "lb",
            Self::Ounce => "oz",
            Self::Dram => "dr",
            Self::Grain => "gr",

            // Troy weight system
            Self::TroyPound => "lb t",
            Self::TroyOunce => "oz t",
            Self::Pennyweight => "dwt",

            // Other units
            Self::Carat => "ct",
            Self::AtomicMassUnit => "amu",
        };
        write!(f, "{s}")
    }
}

impl WeightUnit {
    /// Get the conversion factor to convert this unit to kilograms
    fn to_kilogram_factor(self) -> f64 {
        match self {
            // Large metric units
            Self::Gigatonne => 1e12,
            Self::Megatonne => 1e9,
            Self::MetricTon => 1_000.0,

            // Standard metric units (based on 1 kg = 1000 g)
            Self::Kilogram => 1.0,
            Self::Gram => 1e-3,
            Self::Milligram => 1e-6,
            Self::Microgram => 1e-9,
            Self::Nanogram => 1e-12,
            Self::Picogram => 1e-15,

            // Imperial/US tons and large units
            // Using precise values: 1 lb = 0.45359237 kg exactly (international avoirdupois pound)
            Self::LongTon => 1_016.046_908_8, // 2240 lb × 0.45359237 kg/lb
            Self::ShortTon => 907.184_74,     // 2000 lb × 0.45359237 kg/lb
            Self::Hundredweight => 50.802_345_44, // 112 lb × 0.45359237 kg/lb
            Self::Quarter => 12.700_586_36,   // 28 lb × 0.45359237 kg/lb

            // Imperial/US common units (based on 1 lb = 0.45359237 kg exactly)
            Self::Stone => 6.350_293_18,      // 14 lb × 0.45359237 kg/lb
            Self::Pound => 0.453_592_37,      // Exactly defined
            Self::Ounce => 0.028_349_523_125, // 1/16 lb
            Self::Dram => 0.001_771_845_195_312_5, // 1/256 lb
            Self::Grain => 0.000_064_798_91,  // 1/7000 lb

            // Troy weight system (1 troy lb = 0.3732417216 kg exactly)
            Self::TroyPound => 0.373_241_721_6, // Exactly defined
            Self::TroyOunce => 0.031_103_476_8, // 1/12 troy lb
            Self::Pennyweight => 0.001_555_173_84, // 1/240 troy lb

            // Other units
            Self::Carat => 0.000_2,                       // Exactly 200 mg
            Self::AtomicMassUnit => 1.660_539_066_60e-27, // 2019 CODATA value
        }
    }

    /// Get all supported units as strings
    pub fn supported_units() -> Vec<&'static str> {
        vec![
            "gigatonne",
            "megatonne",
            "metric-ton",
            "kilogram",
            "gram",
            "milligram",
            "microgram",
            "nanogram",
            "picogram",
            "long-ton",
            "short-ton",
            "hundredweight",
            "quarter",
            "stone",
            "pound",
            "ounce",
            "dram",
            "grain",
            "troy-pound",
            "troy-ounce",
            "pennyweight",
            "carat",
            "atomic-mass-unit",
        ]
    }
}

impl FromStr for WeightUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit = match s.to_lowercase().as_str() {
            // Large metric units
            "gigatonne" | "gt" | "gigaton" => Self::Gigatonne,
            "megatonne" | "mt" | "megaton" => Self::Megatonne,
            "metric-ton" | "metric_ton" | "tonne" | "t" | "ton" => Self::MetricTon,

            // Standard metric units
            "kilogram" | "kg" | "kilo" => Self::Kilogram,
            "gram" | "g" | "gm" => Self::Gram,
            "milligram" | "mg" => Self::Milligram,
            "microgram" | "μg" | "ug" | "mcg" => Self::Microgram,
            "nanogram" | "ng" => Self::Nanogram,
            "picogram" | "pg" => Self::Picogram,

            // Imperial/US tons and large units
            "long-ton" | "long_ton" | "imperial_ton" | "uk_ton" => Self::LongTon,
            "short-ton" | "short_ton" | "us_ton" => Self::ShortTon,
            "hundredweight" | "cwt" => Self::Hundredweight,
            "quarter" | "qtr" => Self::Quarter,

            // Imperial/US common units
            "stone" | "st" => Self::Stone,
            "pound" | "lb" | "lbs" => Self::Pound,
            "ounce" | "oz" => Self::Ounce,
            "dram" | "drachm" | "dr" => Self::Dram,
            "grain" | "gr" => Self::Grain,

            // Troy weight system
            "troy-pound" | "troy_pound" | "lb_t" | "lbt" => Self::TroyPound,
            "troy-ounce" | "troy_ounce" | "oz_t" | "ozt" => Self::TroyOunce,
            "pennyweight" | "dwt" | "pwt" => Self::Pennyweight,

            // Other units
            "carat" | "carrat" | "ct" => Self::Carat,
            "atomic-mass-unit" | "atomic_mass_unit" | "amu" | "dalton" | "da" => {
                Self::AtomicMassUnit
            }
            _ => return Err(format!("Unknown weight unit: {s}")),
        };
        Ok(unit)
    }
}

/// Convert weight from one unit to another.
///
/// This function accepts both `WeightUnit` enums and string identifiers.
///
/// # Arguments
///
/// * `value` - The numerical value to convert
/// * `from_unit` - The unit to convert from (can be a `WeightUnit` enum or a string)
/// * `to_unit` - The unit to convert to (can be a `WeightUnit` enum or a string)
///
/// # Returns
///
/// The converted value, or an error if the unit is invalid
///
/// # Examples
///
/// Using enums (type-safe):
/// ```ignore
/// let result = convert_weight(100.0, WeightUnit::Pound, WeightUnit::Kilogram);
/// ```
///
/// Using strings (convenient):
/// ```ignore
/// let result = convert_weight(100.0, "pound", "kilogram");
/// ```
pub fn convert_weight<F, T>(value: f64, from_unit: F, to_unit: T) -> Result<f64, String>
where
    F: IntoWeightUnit,
    T: IntoWeightUnit,
{
    let from = from_unit.into_weight_unit().map_err(|_| {
        format!(
            "Invalid 'from_unit' value. Supported values are:\n{}",
            WeightUnit::supported_units().join(", ")
        )
    })?;

    let to = to_unit.into_weight_unit().map_err(|_| {
        format!(
            "Invalid 'to_unit' value. Supported values are:\n{}",
            WeightUnit::supported_units().join(", ")
        )
    })?;

    // Convert to kilograms first, then to target unit
    let kilograms = value * from.to_kilogram_factor();
    Ok(kilograms / to.to_kilogram_factor())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6; // Tolerance for floating point comparisons

    fn approx_eq(a: f64, b: f64) -> bool {
        let diff = (a - b).abs();
        // Use relative comparison for large numbers, absolute for small numbers
        if a.abs() > 1e10 || b.abs() > 1e10 {
            let max = a.abs().max(b.abs());
            diff / max < EPSILON
        } else {
            diff < EPSILON
        }
    }

    #[test]
    fn test_kilogram_conversions() {
        // Test conversions from kilogram
        assert!(approx_eq(
            convert_weight(4.0, "kilogram", "kilogram").unwrap(),
            4.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "kilogram", "gram").unwrap(),
            1_000.0
        ));
        assert!(approx_eq(
            convert_weight(4.0, "kilogram", "milligram").unwrap(),
            4_000_000.0
        ));
        assert!(approx_eq(
            convert_weight(4.0, "kilogram", "metric-ton").unwrap(),
            0.004
        ));
        assert!(approx_eq(
            convert_weight(3.0, "kilogram", "long-ton").unwrap(),
            0.002_952_396_2
        ));
        assert!(approx_eq(
            convert_weight(1.0, "kilogram", "short-ton").unwrap(),
            0.001_102_311_3
        ));
        assert!(approx_eq(
            convert_weight(4.0, "kilogram", "pound").unwrap(),
            8.818_490_487
        ));
        assert!(approx_eq(
            convert_weight(5.0, "kilogram", "stone").unwrap(),
            0.787_365_222
        ));
        assert!(approx_eq(
            convert_weight(4.0, "kilogram", "ounce").unwrap(),
            141.095_847_8
        ));
        assert!(approx_eq(
            convert_weight(3.0, "kilogram", "carat").unwrap(),
            15_000.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "kilogram", "atomic-mass-unit").unwrap(),
            6.022_140_762e26
        ));
    }

    #[test]
    fn test_large_metric_conversions() {
        // Test gigatonne conversions
        assert!(approx_eq(
            convert_weight(1.0, "gigatonne", "megatonne").unwrap(),
            1_000.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "gigatonne", "metric-ton").unwrap(),
            1e9
        ));
        assert!(approx_eq(
            convert_weight(1.0, "gigatonne", "kilogram").unwrap(),
            1e12
        ));
        assert!(approx_eq(
            convert_weight(1.0, "gigatonne", "gram").unwrap(),
            1e15
        ));

        // Test megatonne conversions
        assert!(approx_eq(
            convert_weight(1.0, "megatonne", "metric-ton").unwrap(),
            1_000_000.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "megatonne", "kilogram").unwrap(),
            1e9
        ));
        assert!(approx_eq(
            convert_weight(1.0, "megatonne", "gram").unwrap(),
            1e12
        ));
    }

    #[test]
    fn test_gram_conversions() {
        // Test conversions from gram
        assert!(approx_eq(
            convert_weight(1.0, "gram", "kilogram").unwrap(),
            0.001
        ));
        assert!(approx_eq(convert_weight(3.0, "gram", "gram").unwrap(), 3.0));
        assert!(approx_eq(
            convert_weight(2.0, "gram", "milligram").unwrap(),
            2_000.0
        ));
        assert!(approx_eq(
            convert_weight(4.0, "gram", "metric-ton").unwrap(),
            4e-6
        ));
        assert!(approx_eq(
            convert_weight(3.0, "gram", "pound").unwrap(),
            0.006_613_867_8
        ));
    }

    #[test]
    fn test_milligram_conversions() {
        // Test conversions from milligram
        assert!(approx_eq(
            convert_weight(1.0, "milligram", "kilogram").unwrap(),
            1e-6
        ));
        assert!(approx_eq(
            convert_weight(2.0, "milligram", "gram").unwrap(),
            0.002
        ));
        assert!(approx_eq(
            convert_weight(3.0, "milligram", "milligram").unwrap(),
            3.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "milligram", "carat").unwrap(),
            0.005
        ));
    }

    #[test]
    fn test_small_metric_conversions() {
        // Test microgram conversions (1 μg = 0.000001 g)
        assert!(approx_eq(
            convert_weight(1.0, "microgram", "gram").unwrap(),
            1e-6
        ));
        assert!(approx_eq(
            convert_weight(1_000.0, "microgram", "milligram").unwrap(),
            1.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "microgram", "kilogram").unwrap(),
            1e-9
        ));

        // Test nanogram conversions (1 ng = 0.000000001 g)
        assert!(approx_eq(
            convert_weight(1.0, "nanogram", "gram").unwrap(),
            1e-9
        ));
        assert!(approx_eq(
            convert_weight(1_000.0, "nanogram", "microgram").unwrap(),
            1.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "nanogram", "kilogram").unwrap(),
            1e-12
        ));

        // Test picogram conversions (1 pg = 0.000000000001 g)
        assert!(approx_eq(
            convert_weight(1.0, "picogram", "gram").unwrap(),
            1e-12
        ));
        assert!(approx_eq(
            convert_weight(1_000.0, "picogram", "nanogram").unwrap(),
            1.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "picogram", "kilogram").unwrap(),
            1e-15
        ));
    }

    #[test]
    fn test_metric_ton_conversions() {
        // Test conversions from metric ton
        assert!(approx_eq(
            convert_weight(2.0, "metric-ton", "kilogram").unwrap(),
            2_000.0
        ));
        assert!(approx_eq(
            convert_weight(2.0, "metric-ton", "gram").unwrap(),
            2_000_000.0
        ));
        assert!(approx_eq(
            convert_weight(3.0, "metric-ton", "milligram").unwrap(),
            3_000_000_000.0
        ));
        assert!(approx_eq(
            convert_weight(2.0, "metric-ton", "metric-ton").unwrap(),
            2.0
        ));
        assert!(approx_eq(
            convert_weight(3.0, "metric-ton", "pound").unwrap(),
            6_613.867_865
        ));
    }

    #[test]
    fn test_long_ton_conversions() {
        // Test conversions from long ton (UK ton = 1016.0469088 kg precisely)
        assert!(approx_eq(
            convert_weight(4.0, "long-ton", "kilogram").unwrap(),
            4_064.187_635_2
        ));
        assert!(approx_eq(
            convert_weight(4.0, "long-ton", "gram").unwrap(),
            4_064_187.635_2
        ));
        assert!(approx_eq(
            convert_weight(4.0, "long-ton", "metric-ton").unwrap(),
            4.064_187_635_2
        ));
        assert!(approx_eq(
            convert_weight(3.0, "long-ton", "long-ton").unwrap(),
            3.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "long-ton", "short-ton").unwrap(),
            1.12
        ));
    }

    #[test]
    fn test_imperial_large_units() {
        // Test hundredweight (112 lb = 50.80234544 kg precisely)
        assert!(approx_eq(
            convert_weight(1.0, "hundredweight", "kilogram").unwrap(),
            50.802_345_44
        ));
        assert!(approx_eq(
            convert_weight(1.0, "hundredweight", "gram").unwrap(),
            50_802.345_44
        ));
        assert!(approx_eq(
            convert_weight(1.0, "hundredweight", "pound").unwrap(),
            112.0
        ));
        assert!(approx_eq(
            convert_weight(20.0, "hundredweight", "long-ton").unwrap(),
            1.0
        ));

        // Test quarter (28 lb = 12.70058636 kg precisely)
        assert!(approx_eq(
            convert_weight(1.0, "quarter", "kilogram").unwrap(),
            12.700_586_36
        ));
        assert!(approx_eq(
            convert_weight(1.0, "quarter", "gram").unwrap(),
            12_700.586_36
        ));
        assert!(approx_eq(
            convert_weight(1.0, "quarter", "pound").unwrap(),
            28.0
        ));
        assert!(approx_eq(
            convert_weight(4.0, "quarter", "hundredweight").unwrap(),
            1.0
        ));
    }

    #[test]
    fn test_short_ton_conversions() {
        // Test conversions from short ton (2000 lb = 907.18474 kg precisely)
        assert!(approx_eq(
            convert_weight(3.0, "short-ton", "kilogram").unwrap(),
            2_721.554_22
        ));
        assert!(approx_eq(
            convert_weight(3.0, "short-ton", "gram").unwrap(),
            2_721_554.22
        ));
        assert!(approx_eq(
            convert_weight(1.0, "short-ton", "milligram").unwrap(),
            907_184_740.0
        ));
        assert!(approx_eq(
            convert_weight(4.0, "short-ton", "metric-ton").unwrap(),
            3.628_738_96
        ));
        assert!(approx_eq(
            convert_weight(2.0, "short-ton", "pound").unwrap(),
            4_000.0
        ));
    }

    #[test]
    fn test_pound_conversions() {
        // Test conversions from pound (0.45359237 kg exactly)
        assert!(approx_eq(
            convert_weight(4.0, "pound", "kilogram").unwrap(),
            1.814_369_48
        ));
        assert!(approx_eq(
            convert_weight(2.0, "pound", "gram").unwrap(),
            907.184_74
        ));
        assert!(approx_eq(
            convert_weight(3.0, "pound", "milligram").unwrap(),
            1_360_777.11
        ));
        assert!(approx_eq(
            convert_weight(3.0, "pound", "pound").unwrap(),
            3.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "pound", "ounce").unwrap(),
            16.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "pound", "carat").unwrap(),
            2_267.961_85
        ));
    }

    #[test]
    fn test_stone_conversions() {
        // Test conversions from stone (14 lb = 6.35029318 kg precisely)
        assert!(approx_eq(
            convert_weight(5.0, "stone", "kilogram").unwrap(),
            31.751_465_9
        ));
        assert!(approx_eq(
            convert_weight(2.0, "stone", "gram").unwrap(),
            12_700.586_36
        ));
        assert!(approx_eq(
            convert_weight(2.0, "stone", "pound").unwrap(),
            28.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "stone", "ounce").unwrap(),
            224.0
        ));
    }

    #[test]
    fn test_ounce_conversions() {
        // Test conversions from ounce (1/16 lb = 0.028349523125 kg precisely)
        assert!(approx_eq(
            convert_weight(3.0, "ounce", "kilogram").unwrap(),
            0.085_048_569_375
        ));
        assert!(approx_eq(
            convert_weight(3.0, "ounce", "gram").unwrap(),
            85.048_569_375
        ));
        assert!(approx_eq(
            convert_weight(1.0, "ounce", "pound").unwrap(),
            0.0625
        ));
        assert!(approx_eq(
            convert_weight(2.0, "ounce", "ounce").unwrap(),
            2.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "ounce", "carat").unwrap(),
            141.747_615_625
        ));
    }

    #[test]
    fn test_small_imperial_units() {
        // Test dram (1/256 lb = 0.0017718451953125 kg precisely)
        assert!(approx_eq(
            convert_weight(1.0, "dram", "gram").unwrap(),
            1.771_845_195_312_5
        ));
        assert!(approx_eq(
            convert_weight(1.0, "dram", "kilogram").unwrap(),
            0.001_771_845_195_312_5
        ));
        assert!(approx_eq(
            convert_weight(256.0, "dram", "pound").unwrap(),
            1.0
        ));
        assert!(approx_eq(
            convert_weight(16.0, "dram", "ounce").unwrap(),
            1.0
        ));

        // Test grain (1/7000 lb = 0.00006479891 kg precisely)
        assert!(approx_eq(
            convert_weight(1.0, "grain", "gram").unwrap(),
            0.064_798_91
        ));
        assert!(approx_eq(
            convert_weight(1.0, "grain", "kilogram").unwrap(),
            0.000_064_798_91
        ));
        assert!(approx_eq(
            convert_weight(7000.0, "grain", "pound").unwrap(),
            1.0
        ));
    }

    #[test]
    fn test_carat_conversions() {
        // Test conversions from carat
        assert!(approx_eq(
            convert_weight(1.0, "carat", "kilogram").unwrap(),
            0.000_2
        ));
        assert!(approx_eq(
            convert_weight(4.0, "carat", "gram").unwrap(),
            0.8
        ));
        assert!(approx_eq(
            convert_weight(2.0, "carat", "milligram").unwrap(),
            400.0
        ));
        assert!(approx_eq(
            convert_weight(4.0, "carat", "carat").unwrap(),
            4.0
        ));
    }

    #[test]
    fn test_troy_weight_system() {
        // Test troy pound (0.3732417216 kg exactly)
        assert!(approx_eq(
            convert_weight(1.0, "troy-pound", "gram").unwrap(),
            373.241_721_6
        ));
        assert!(approx_eq(
            convert_weight(1.0, "troy-pound", "kilogram").unwrap(),
            0.373_241_721_6
        ));
        assert!(approx_eq(
            convert_weight(1.0, "troy-pound", "pound").unwrap(),
            0.822_857_143
        ));
        assert!(approx_eq(
            convert_weight(1.0, "troy-pound", "troy-ounce").unwrap(),
            12.0
        ));

        // Test troy ounce (1/12 troy lb = 0.0311034768 kg exactly)
        assert!(approx_eq(
            convert_weight(1.0, "troy-ounce", "gram").unwrap(),
            31.103_476_8
        ));
        assert!(approx_eq(
            convert_weight(1.0, "troy-ounce", "kilogram").unwrap(),
            0.031_103_476_8
        ));
        assert!(approx_eq(
            convert_weight(1.0, "troy-ounce", "ounce").unwrap(),
            1.097_142_857
        ));
        assert!(approx_eq(
            convert_weight(12.0, "troy-ounce", "troy-pound").unwrap(),
            1.0
        ));
        assert!(approx_eq(
            convert_weight(1.0, "troy-ounce", "pennyweight").unwrap(),
            20.0
        ));

        // Test pennyweight (1/240 troy lb = 0.00155517384 kg exactly)
        assert!(approx_eq(
            convert_weight(1.0, "pennyweight", "gram").unwrap(),
            1.555_173_84
        ));
        assert!(approx_eq(
            convert_weight(1.0, "pennyweight", "kilogram").unwrap(),
            0.001_555_173_84
        ));
        assert!(approx_eq(
            convert_weight(20.0, "pennyweight", "troy-ounce").unwrap(),
            1.0
        ));
        assert!(approx_eq(
            convert_weight(240.0, "pennyweight", "troy-pound").unwrap(),
            1.0
        ));
    }

    #[test]
    fn test_atomic_mass_unit_conversions() {
        // Test conversions from atomic mass unit
        assert!(approx_eq(
            convert_weight(4.0, "atomic-mass-unit", "kilogram").unwrap(),
            6.642_160_796e-27
        ));
        assert!(approx_eq(
            convert_weight(2.0, "atomic-mass-unit", "atomic-mass-unit").unwrap(),
            2.0
        ));
    }

    #[test]
    fn test_using_enums() {
        // Test using enums (type-safe)
        assert!(approx_eq(
            convert_weight(1.0, WeightUnit::Kilogram, WeightUnit::Gram).unwrap(),
            1_000.0
        ));
        assert!(approx_eq(
            convert_weight(100.0, WeightUnit::Pound, WeightUnit::Kilogram).unwrap(),
            45.359_237
        ));
    }

    #[test]
    fn test_mixed_usage() {
        // Test mixed usage (enum and string)
        assert!(approx_eq(
            convert_weight(1.0, WeightUnit::Kilogram, "pound").unwrap(),
            2.204_622_622
        ));
        assert!(approx_eq(
            convert_weight(16.0, "ounce", WeightUnit::Pound).unwrap(),
            1.0
        ));
    }

    #[test]
    fn test_invalid_units() {
        // Test invalid units
        assert!(convert_weight(4.0, "slug", "kilogram").is_err());
        assert!(convert_weight(4.0, "kilogram", "wrongUnit").is_err());
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Test roundtrip conversion
        let original = 100.0;
        let converted = convert_weight(original, "pound", "kilogram").unwrap();
        let back = convert_weight(converted, "kilogram", "pound").unwrap();
        assert!(approx_eq(original, back));
    }

    #[test]
    fn test_string_ownership() {
        // Test String (owned) conversion
        let unit_string = String::from("kilogram");
        assert_eq!(
            unit_string.into_weight_unit().unwrap(),
            WeightUnit::Kilogram
        );

        let invalid_string = String::from("invalid");
        assert!(invalid_string.into_weight_unit().is_err());
    }

    #[test]
    fn test_display_implementation() {
        // Test Display implementation for all units
        assert_eq!(format!("{}", WeightUnit::Gigatonne), "Gt");
        assert_eq!(format!("{}", WeightUnit::Megatonne), "Mt");
        assert_eq!(format!("{}", WeightUnit::MetricTon), "t");
        assert_eq!(format!("{}", WeightUnit::Kilogram), "kg");
        assert_eq!(format!("{}", WeightUnit::Gram), "g");
        assert_eq!(format!("{}", WeightUnit::Milligram), "mg");
        assert_eq!(format!("{}", WeightUnit::Microgram), "μg");
        assert_eq!(format!("{}", WeightUnit::Nanogram), "ng");
        assert_eq!(format!("{}", WeightUnit::Picogram), "pg");
        assert_eq!(format!("{}", WeightUnit::LongTon), "long ton");
        assert_eq!(format!("{}", WeightUnit::ShortTon), "short ton");
        assert_eq!(format!("{}", WeightUnit::Hundredweight), "cwt");
        assert_eq!(format!("{}", WeightUnit::Quarter), "qtr");
        assert_eq!(format!("{}", WeightUnit::Stone), "st");
        assert_eq!(format!("{}", WeightUnit::Pound), "lb");
        assert_eq!(format!("{}", WeightUnit::Ounce), "oz");
        assert_eq!(format!("{}", WeightUnit::Dram), "dr");
        assert_eq!(format!("{}", WeightUnit::Grain), "gr");
        assert_eq!(format!("{}", WeightUnit::TroyPound), "lb t");
        assert_eq!(format!("{}", WeightUnit::TroyOunce), "oz t");
        assert_eq!(format!("{}", WeightUnit::Pennyweight), "dwt");
        assert_eq!(format!("{}", WeightUnit::Carat), "ct");
        assert_eq!(format!("{}", WeightUnit::AtomicMassUnit), "amu");
    }

    #[test]
    fn test_alternative_names() {
        // Test alternative unit names
        assert!(convert_weight(1.0, "kg", "gram").is_ok());
        assert!(convert_weight(1.0, "lb", "kilogram").is_ok());
        assert!(convert_weight(1.0, "oz", "gram").is_ok());
        assert!(convert_weight(1.0, "tonne", "kilogram").is_ok());
        assert!(convert_weight(1.0, "dalton", "kilogram").is_ok());
        assert!(convert_weight(1.0, "gt", "megatonne").is_ok());
        assert!(convert_weight(1.0, "mt", "metric-ton").is_ok());
        assert!(convert_weight(1.0, "ug", "gram").is_ok());
        assert!(convert_weight(1.0, "μg", "microgram").is_ok());
        assert!(convert_weight(1.0, "cwt", "kilogram").is_ok());
        assert!(convert_weight(1.0, "qtr", "quarter").is_ok());
        assert!(convert_weight(1.0, "dr", "gram").is_ok());
        assert!(convert_weight(1.0, "gr", "grain").is_ok());
        assert!(convert_weight(1.0, "ozt", "gram").is_ok());
        assert!(convert_weight(1.0, "lbt", "troy-pound").is_ok());
        assert!(convert_weight(1.0, "dwt", "gram").is_ok());
    }
}
