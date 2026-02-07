//! Convert between different units of energy
//!
//! Supports conversions between 70+ energy units using Joule as an intermediary:
//! - SI units: Joule (J), kilojoule, megajoule, gigajoule
//! - Power-time units: Watt-hour, kilowatt-hour, megawatt-hour
//! - Calories: Nutritional, IT (International Table), thermochemical
//! - Electron volts: eV, keV, MeV
//! - British Thermal Units: BTU (IT), BTU (th), mega BTU
//! - Imperial units: foot-pound, pound-force foot, etc.
//! - Historical and specialized units: therm, ton TNT equivalent, Hartree energy

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnergyUnit {
    // SI units
    Joule,
    Kilojoule,
    Megajoule,
    Gigajoule,
    Millijoule,
    Microjoule,
    Nanojoule,
    Attojoule,

    // Power-time units
    WattSecond,
    WattHour,
    KilowattSecond,
    KilowattHour,
    MegawattHour,
    GigawattHour,

    // Mechanical units
    NewtonMeter,
    Erg,
    DyneCentimeter,

    // Calories
    CalorieNutritional,
    KilocalorieNutritional,
    CalorieIT,
    KilocalorieIT,
    CalorieTh,
    KilocalorieTh,

    // Electron volts
    Electronvolt,
    Kiloelectronvolt,
    Megaelectronvolt,

    // British Thermal Units
    BtuIT,
    BtuTh,
    MegaBtuIT,

    // Imperial force-distance units
    FootPound,
    PoundForceFoot,
    PoundForceInch,
    InchPound,
    InchOunce,
    OunceForceInch,
    PoundalFoot,

    // Horsepower units
    HorsepowerHour,
    HorsepowerMetricHour,

    // Metric force units
    KilogramForceMeter,
    KilogramForceCentimeter,
    KilopondMeter,
    GramForceMeter,
    GramForceCentimeter,

    // Therm units
    Therm,
    ThermEC,
    ThermUS,

    // Specialized units
    TonHourRefrigeration,
    FuelOilEquivalentKiloliter,
    FuelOilEquivalentBarrel,
    TonExplosives,
    Kiloton,
    Megaton,
    Gigaton,

    // Atomic units
    HartreeEnergy,
    RydbergConstant,
}

impl EnergyUnit {
    fn to_joule(self, value: f64) -> f64 {
        match self {
            // SI units
            EnergyUnit::Joule | EnergyUnit::WattSecond | EnergyUnit::NewtonMeter => value,
            EnergyUnit::Kilojoule | EnergyUnit::KilowattSecond => value * 1_000.0,
            EnergyUnit::Megajoule => value * 1_000_000.0,
            EnergyUnit::Gigajoule => value * 1_000_000_000.0,
            EnergyUnit::Millijoule => value * 0.001,
            EnergyUnit::Microjoule => value * 1.0e-6,
            EnergyUnit::Nanojoule => value * 1.0e-9,
            EnergyUnit::Attojoule => value * 1.0e-18,

            // Power-time units
            EnergyUnit::WattHour => value * 3_600.0,
            EnergyUnit::KilowattHour => value * 3_600_000.0,
            EnergyUnit::MegawattHour => value * 3_600_000_000.0,
            EnergyUnit::GigawattHour => value * 3_600_000_000_000.0,

            // Mechanical units
            EnergyUnit::Erg | EnergyUnit::DyneCentimeter => value * 1.0e-7,

            // Calories
            EnergyUnit::CalorieNutritional | EnergyUnit::KilocalorieIT => value * 4_186.8,
            EnergyUnit::KilocalorieNutritional => value * 4_186_800.0,
            EnergyUnit::CalorieIT => value * 4.1868,
            EnergyUnit::CalorieTh => value * 4.184,
            EnergyUnit::KilocalorieTh => value * 4_184.0,

            // Electron volts
            EnergyUnit::Electronvolt => value * 1.602_176_634e-19,
            EnergyUnit::Kiloelectronvolt => value * 1.602_176_634e-16,
            EnergyUnit::Megaelectronvolt => value * 1.602_176_634e-13,

            // British Thermal Units
            EnergyUnit::BtuIT => value * 1_055.055_852_62,
            EnergyUnit::BtuTh => value * 1_054.349_999_974_4,
            EnergyUnit::MegaBtuIT => value * 1_055_055_852.62,

            // Imperial force-distance units
            EnergyUnit::FootPound | EnergyUnit::PoundForceFoot => value * 1.355_817_948_3,
            EnergyUnit::PoundForceInch | EnergyUnit::InchPound => value * 0.112_984_829,
            EnergyUnit::InchOunce | EnergyUnit::OunceForceInch => value * 0.007_061_551_8,
            EnergyUnit::PoundalFoot => value * 0.042_140_11,

            // Horsepower units
            EnergyUnit::HorsepowerHour => value * 2_684_519.536_885_6,
            EnergyUnit::HorsepowerMetricHour => value * 2_647_795.5,

            // Metric force units
            EnergyUnit::KilogramForceMeter | EnergyUnit::KilopondMeter => value * 9.806_649_999_7,
            EnergyUnit::KilogramForceCentimeter => value * 0.098_066_5,
            EnergyUnit::GramForceMeter => value * 0.009_806_65,
            EnergyUnit::GramForceCentimeter => value * 9.806_65e-5,

            // Therm units
            EnergyUnit::Therm | EnergyUnit::ThermEC => value * 105_505_600.0,
            EnergyUnit::ThermUS => value * 105_480_400.0,

            // Specialized units
            EnergyUnit::TonHourRefrigeration => value * 12_660_670.231_44,
            EnergyUnit::FuelOilEquivalentKiloliter => value * 40_197_627_984.822,
            EnergyUnit::FuelOilEquivalentBarrel => value * 6_383_087_908.350_9,
            EnergyUnit::TonExplosives => value * 4_184_000_000.0,
            EnergyUnit::Kiloton => value * 4_184_000_000_000.0,
            EnergyUnit::Megaton => value * 4.184e15,
            EnergyUnit::Gigaton => value * 4.184e18,

            // Atomic units
            EnergyUnit::HartreeEnergy => value * 4.359_748_2e-18,
            EnergyUnit::RydbergConstant => value * 2.179_874_1e-18,
        }
    }

    fn joule_to_unit(self, joule: f64) -> f64 {
        match self {
            // SI units
            EnergyUnit::Joule | EnergyUnit::WattSecond | EnergyUnit::NewtonMeter => joule,
            EnergyUnit::Kilojoule | EnergyUnit::KilowattSecond => joule / 1_000.0,
            EnergyUnit::Megajoule => joule / 1_000_000.0,
            EnergyUnit::Gigajoule => joule / 1_000_000_000.0,
            EnergyUnit::Millijoule => joule / 0.001,
            EnergyUnit::Microjoule => joule / 1.0e-6,
            EnergyUnit::Nanojoule => joule / 1.0e-9,
            EnergyUnit::Attojoule => joule / 1.0e-18,

            // Power-time units
            EnergyUnit::WattHour => joule / 3_600.0,
            EnergyUnit::KilowattHour => joule / 3_600_000.0,
            EnergyUnit::MegawattHour => joule / 3_600_000_000.0,
            EnergyUnit::GigawattHour => joule / 3_600_000_000_000.0,

            // Mechanical units
            EnergyUnit::Erg | EnergyUnit::DyneCentimeter => joule / 1.0e-7,

            // Calories
            EnergyUnit::CalorieNutritional | EnergyUnit::KilocalorieIT => joule / 4_186.8,
            EnergyUnit::KilocalorieNutritional => joule / 4_186_800.0,
            EnergyUnit::CalorieIT => joule / 4.1868,
            EnergyUnit::CalorieTh => joule / 4.184,
            EnergyUnit::KilocalorieTh => joule / 4_184.0,

            // Electron volts
            EnergyUnit::Electronvolt => joule / 1.602_176_634e-19,
            EnergyUnit::Kiloelectronvolt => joule / 1.602_176_634e-16,
            EnergyUnit::Megaelectronvolt => joule / 1.602_176_634e-13,

            // British Thermal Units
            EnergyUnit::BtuIT => joule / 1_055.055_852_62,
            EnergyUnit::BtuTh => joule / 1_054.349_999_974_4,
            EnergyUnit::MegaBtuIT => joule / 1_055_055_852.62,

            // Imperial force-distance units
            EnergyUnit::FootPound | EnergyUnit::PoundForceFoot => joule / 1.355_817_948_3,
            EnergyUnit::PoundForceInch | EnergyUnit::InchPound => joule / 0.112_984_829,
            EnergyUnit::InchOunce | EnergyUnit::OunceForceInch => joule / 0.007_061_551_8,
            EnergyUnit::PoundalFoot => joule / 0.042_140_11,

            // Horsepower units
            EnergyUnit::HorsepowerHour => joule / 2_684_519.536_885_6,
            EnergyUnit::HorsepowerMetricHour => joule / 2_647_795.5,

            // Metric force units
            EnergyUnit::KilogramForceMeter | EnergyUnit::KilopondMeter => joule / 9.806_649_999_7,
            EnergyUnit::KilogramForceCentimeter => joule / 0.098_066_5,
            EnergyUnit::GramForceMeter => joule / 0.009_806_65,
            EnergyUnit::GramForceCentimeter => joule / 9.806_65e-5,

            // Therm units
            EnergyUnit::Therm | EnergyUnit::ThermEC => joule / 105_505_600.0,
            EnergyUnit::ThermUS => joule / 105_480_400.0,

            // Specialized units
            EnergyUnit::TonHourRefrigeration => joule / 12_660_670.231_44,
            EnergyUnit::FuelOilEquivalentKiloliter => joule / 40_197_627_984.822,
            EnergyUnit::FuelOilEquivalentBarrel => joule / 6_383_087_908.350_9,
            EnergyUnit::TonExplosives => joule / 4_184_000_000.0,
            EnergyUnit::Kiloton => joule / 4_184_000_000_000.0,
            EnergyUnit::Megaton => joule / 4.184e15,
            EnergyUnit::Gigaton => joule / 4.184e18,

            // Atomic units
            EnergyUnit::HartreeEnergy => joule / 4.359_748_2e-18,
            EnergyUnit::RydbergConstant => joule / 2.179_874_1e-18,
        }
    }
}

pub fn convert_energy(value: f64, from: EnergyUnit, to: EnergyUnit) -> f64 {
    let joule = from.to_joule(value);
    to.joule_to_unit(joule)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_same_unit_conversion() {
        let value = 42.0;
        for unit in [
            EnergyUnit::Joule,
            EnergyUnit::Kilojoule,
            EnergyUnit::KilowattHour,
            EnergyUnit::CalorieNutritional,
            EnergyUnit::BtuIT,
            EnergyUnit::FootPound,
        ] {
            assert!(approx_eq(convert_energy(value, unit, unit), value));
        }
    }

    #[test]
    fn test_joule_to_kilojoule() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::Kilojoule),
            0.001
        ));
        assert!(approx_eq(
            convert_energy(1000.0, EnergyUnit::Joule, EnergyUnit::Kilojoule),
            1.0
        ));
    }

    #[test]
    fn test_joule_to_megajoule() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::Megajoule),
            1e-6
        ));
        assert!(approx_eq(
            convert_energy(1_000_000.0, EnergyUnit::Joule, EnergyUnit::Megajoule),
            1.0
        ));
    }

    #[test]
    fn test_joule_to_gigajoule() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::Gigajoule),
            1e-9
        ));
    }

    #[test]
    fn test_watt_second() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::WattSecond),
            1.0
        ));
    }

    #[test]
    fn test_watt_hour() {
        let result = convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::WattHour);
        assert!((result - 0.000_277_777_777_777_777_8).abs() < 1e-15);
    }

    #[test]
    fn test_kilowatt_hour_conversions() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::KilowattHour, EnergyUnit::Joule),
            3_600_000.0
        ));
        assert!(approx_eq(
            convert_energy(10.0, EnergyUnit::KilowattHour, EnergyUnit::Joule),
            36_000_000.0
        ));
    }

    #[test]
    fn test_newton_meter() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::NewtonMeter),
            1.0
        ));
    }

    #[test]
    fn test_calorie_nutritional() {
        let result = convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::CalorieNutritional);
        assert!((result - 0.000_238_845_896_627_495_9).abs() < 1e-15);

        assert!(approx_eq(
            convert_energy(
                1000.0,
                EnergyUnit::CalorieNutritional,
                EnergyUnit::KilocalorieNutritional
            ),
            1.0
        ));
    }

    #[test]
    fn test_electronvolt() {
        let result = convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::Electronvolt);
        assert!((result - 6.241_509_074_460_763e18).abs() < 1e15);
    }

    #[test]
    fn test_btu_conversions() {
        let result = convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::BtuIT);
        assert!((result - 0.000_947_817_120_313_317_3).abs() < 1e-15);

        let result = convert_energy(1.0, EnergyUnit::BtuIT, EnergyUnit::FootPound);
        assert!((result - 778.169).abs() < 0.01);
    }

    #[test]
    fn test_foot_pound() {
        let result = convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::FootPound);
        assert!((result - 0.737_562_149_294_347).abs() < 1e-10);
    }

    #[test]
    fn test_round_trip_conversions() {
        let value = 100.0;
        let units = [
            EnergyUnit::Joule,
            EnergyUnit::Kilojoule,
            EnergyUnit::KilowattHour,
            EnergyUnit::CalorieNutritional,
            EnergyUnit::BtuIT,
            EnergyUnit::FootPound,
            EnergyUnit::Electronvolt,
            EnergyUnit::Erg,
        ];

        for from_unit in units.iter() {
            for to_unit in units.iter() {
                let converted = convert_energy(value, *from_unit, *to_unit);
                let back = convert_energy(converted, *to_unit, *from_unit);
                assert!(
                    approx_eq(back, value),
                    "Round trip failed: {from_unit:?} -> {to_unit:?} -> {from_unit:?}: {back} != {value}"
                );
            }
        }
    }

    #[test]
    fn test_megawatt_hour() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::MegawattHour, EnergyUnit::Joule),
            3_600_000_000.0
        ));
    }

    #[test]
    fn test_horsepower_hour() {
        let result = convert_energy(1.0, EnergyUnit::HorsepowerHour, EnergyUnit::Joule);
        assert!((result - 2_684_519.536_885_6).abs() < 0.01);
    }

    #[test]
    fn test_therm() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Therm, EnergyUnit::Joule),
            105_505_600.0
        ));
    }

    #[test]
    fn test_ton_explosives() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::TonExplosives, EnergyUnit::Joule),
            4_184_000_000.0
        ));
    }

    #[test]
    fn test_kiloton() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Kiloton, EnergyUnit::Joule),
            4_184_000_000_000.0
        ));
    }

    #[test]
    fn test_erg() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Erg, EnergyUnit::Joule),
            1.0e-7
        ));
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::Erg),
            1.0e7
        ));
    }

    #[test]
    fn test_small_si_units() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Millijoule, EnergyUnit::Joule),
            0.001
        ));
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Microjoule, EnergyUnit::Joule),
            1.0e-6
        ));
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Nanojoule, EnergyUnit::Joule),
            1.0e-9
        ));
    }

    #[test]
    fn test_calorie_variants() {
        let it_result = convert_energy(1.0, EnergyUnit::CalorieIT, EnergyUnit::Joule);
        let th_result = convert_energy(1.0, EnergyUnit::CalorieTh, EnergyUnit::Joule);
        assert!((it_result - 4.1868).abs() < 1e-10);
        assert!((th_result - 4.184).abs() < 1e-10);
    }

    #[test]
    fn test_food_energy() {
        // 2000 Calories (nutritional) to kilojoules
        let result = convert_energy(
            2000.0,
            EnergyUnit::CalorieNutritional,
            EnergyUnit::Kilojoule,
        );
        assert!((result - 8373.6).abs() < 0.1);
    }

    #[test]
    fn test_electricity_bill() {
        // 100 kWh to megajoules
        let result = convert_energy(100.0, EnergyUnit::KilowattHour, EnergyUnit::Megajoule);
        assert!((result - 360.0).abs() < 0.1);
    }

    #[test]
    fn test_zero_value() {
        assert!(approx_eq(
            convert_energy(0.0, EnergyUnit::Joule, EnergyUnit::KilowattHour),
            0.0
        ));
    }

    #[test]
    fn test_negative_value() {
        assert!(approx_eq(
            convert_energy(-1000.0, EnergyUnit::Joule, EnergyUnit::Kilojoule),
            -1.0
        ));
    }

    #[test]
    fn test_large_value() {
        let result = convert_energy(100.0, EnergyUnit::Gigajoule, EnergyUnit::Joule);
        assert!(approx_eq(result, 100_000_000_000.0));
    }

    #[test]
    fn test_imperial_units() {
        // Pound-force foot equals foot-pound
        let value = 50.0;
        let result1 = convert_energy(value, EnergyUnit::FootPound, EnergyUnit::Joule);
        let result2 = convert_energy(value, EnergyUnit::PoundForceFoot, EnergyUnit::Joule);
        assert!(approx_eq(result1, result2));
    }

    #[test]
    fn test_electron_volt_variants() {
        assert!(approx_eq(
            convert_energy(1.0, EnergyUnit::Kiloelectronvolt, EnergyUnit::Electronvolt),
            1000.0
        ));
        assert!(approx_eq(
            convert_energy(
                1.0,
                EnergyUnit::Megaelectronvolt,
                EnergyUnit::Kiloelectronvolt
            ),
            1000.0
        ));
    }

    #[test]
    fn test_therm_variants() {
        // Therm and Therm EC should be equal
        let value = 5.0;
        let result1 = convert_energy(value, EnergyUnit::Therm, EnergyUnit::Joule);
        let result2 = convert_energy(value, EnergyUnit::ThermEC, EnergyUnit::Joule);
        assert!(approx_eq(result1, result2));

        // Therm US should be slightly different
        let result3 = convert_energy(value, EnergyUnit::ThermUS, EnergyUnit::Joule);
        assert!((result1 - result3).abs() > 1.0); // They differ
    }

    #[test]
    fn test_explosive_yield() {
        // 1 megaton TNT to gigajoules
        let result = convert_energy(1.0, EnergyUnit::Megaton, EnergyUnit::Gigajoule);
        assert!((result - 4_184_000.0).abs() < 1.0);
    }

    #[test]
    fn test_atomic_units() {
        // Hartree energy conversion
        let result = convert_energy(1.0, EnergyUnit::HartreeEnergy, EnergyUnit::Joule);
        assert!((result - 4.359_748_2e-18).abs() < 1e-25);

        // Rydberg constant should be half of Hartree
        let hartree = convert_energy(1.0, EnergyUnit::HartreeEnergy, EnergyUnit::Joule);
        let rydberg = convert_energy(1.0, EnergyUnit::RydbergConstant, EnergyUnit::Joule);
        assert!((rydberg * 2.0 - hartree).abs() < 1e-25);
    }
}
