/// Author : https://github.com/ali77gh
/// Conversion of length units.
///
/// Available Units:
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Millimeter
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Centimeter
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Meter
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Kilometer
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Inch
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Foot
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Yard
/// -> Wikipedia reference: https://en.wikipedia.org/wiki/Mile

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum LengthUnit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
    Inch,
    Foot,
    Yard,
    Mile,
}

fn unit_to_meter_multiplier(from: LengthUnit) -> f64 {
    match from {
        LengthUnit::Millimeter => 0.001,
        LengthUnit::Centimeter => 0.01,
        LengthUnit::Meter => 1.0,
        LengthUnit::Kilometer => 1000.0,
        LengthUnit::Inch => 0.0254,
        LengthUnit::Foot => 0.3048,
        LengthUnit::Yard => 0.9144,
        LengthUnit::Mile => 1609.34,
    }
}

fn unit_to_meter(input: f64, from: LengthUnit) -> f64 {
    input * unit_to_meter_multiplier(from)
}

fn meter_to_unit(input: f64, to: LengthUnit) -> f64 {
    input / unit_to_meter_multiplier(to)
}

/// This function will convert a value in unit of [from] to value in unit of [to]
/// by first converting it to meter and than convert it to destination unit
pub fn length_conversion(input: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    meter_to_unit(unit_to_meter(input, from), to)
}

#[cfg(test)]
mod length_conversion_tests {
    use std::collections::HashMap;

    use super::LengthUnit::*;
    use super::*;

    #[test]
    fn zero_to_zero() {
        let units = vec![
            Millimeter, Centimeter, Meter, Kilometer, Inch, Foot, Yard, Mile,
        ];

        for u1 in units.clone() {
            for u2 in units.clone() {
                assert_eq!(length_conversion(0f64, u1, u2), 0f64);
            }
        }
    }

    #[test]
    fn length_of_one_meter() {
        let meter_in_different_units = HashMap::from([
            (Millimeter, 1000f64),
            (Centimeter, 100f64),
            (Kilometer, 0.001f64),
            (Inch, 39.37007874015748f64),
            (Foot, 3.280839895013123f64),
            (Yard, 1.0936132983377078f64),
            (Mile, 0.0006213727366498068f64),
        ]);
        for (input_unit, input_value) in &meter_in_different_units {
            for (target_unit, target_value) in &meter_in_different_units {
                assert!(
                    num_traits::abs(
                        length_conversion(*input_value, *input_unit, *target_unit) - *target_value
                    ) < 0.0000001
                );
            }
        }
    }
}
