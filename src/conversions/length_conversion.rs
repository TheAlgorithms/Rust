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

/// Universal Units on Length
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

/// Private methods
/// This is a n*n problem
/// It's gonna be 56 functions or 56 cases in match statement
/// It's hard to write code for every unit to unit so I solve this problem
/// by converting input to meter and than convert it to output
impl LengthUnit {
    /// This function give you a number (let's call it n)
    /// So if you multiple a value in this unit to n you will get meters
    ///
    /// m * in-unit = meter
    fn get_unit_to_meter_multiplier(&self) -> f64 {
        match self {
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

    /// This function give you a number (let's call it n)
    /// So if you multiple a value in meters to n you will get unit
    ///
    /// m * meter = in-unit
    fn get_unit_from_meter_multiplier(&self) -> f64 {
        1.0 / self.get_unit_to_meter_multiplier()
    }
}

/// This function will convert a value in unit of [from] to value in unit of [to]
/// by first converting it to meter and than convert it to destination unit
pub fn length_conversion(input: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    input * from.get_unit_to_meter_multiplier() * to.get_unit_from_meter_multiplier()
}

#[cfg(test)]
mod length_conversion_tests {
    use super::LengthUnit::*;
    use super::*;

    #[test]
    fn meter_to_other() {
        assert_eq!(length_conversion(4f64, Meter, Millimeter), 4000.0);
        assert_eq!(length_conversion(4f64, Meter, Foot), 13.123359580052492);
        assert_eq!(length_conversion(1.0, Meter, Kilometer), 0.001);
    }

    #[test]
    fn other_to_meter() {
        assert_eq!(length_conversion(4f64, Millimeter, Meter), 0.004);
        assert_eq!(length_conversion(2.0, Foot, Meter), 0.6096);
        assert_eq!(length_conversion(1.0, Inch, Meter), 0.0254);
        assert_eq!(length_conversion(4.0, Yard, Meter), 3.6576);
        assert_eq!(length_conversion(3.0, Foot, Meter), 0.9144000000000001);
    }

    #[test]
    fn other_to_other() {
        // ---------------
        assert_eq!(length_conversion(1.0, Kilometer, Inch), 39370.07874015748);
        assert_eq!(length_conversion(3.0, Kilometer, Mile), 1.8641182099494205);
        assert_eq!(length_conversion(4.0, Foot, Yard), 1.3333333333333335);
        assert_eq!(length_conversion(2.0, Inch, Mile), 3.156573502181019e-5);
        assert_eq!(length_conversion(2.0, Centimeter, Millimeter), 20.0);
        assert_eq!(
            length_conversion(2.0, Centimeter, Yard),
            0.021872265966754158
        );
        assert_eq!(length_conversion(4.0, Yard, Kilometer), 0.0036576);
        assert_eq!(length_conversion(3.0, Foot, Inch), 36.00000000000001);
        assert_eq!(length_conversion(4.0, Mile, Kilometer), 6.43736);
        assert_eq!(length_conversion(2.0, Mile, Inch), 126719.68503937007);
        assert_eq!(length_conversion(3.0, Millimeter, Centimeter), 0.3);
        assert_eq!(
            length_conversion(3.0, Millimeter, Inch),
            0.11811023622047245
        );
    }
}
