//! Module for converting between RGB and HSV color representations
//!
//! The RGB color model is an additive color model in which red, green, and blue light
//! are added together in various ways to reproduce a broad array of colors. The name
//! of the model comes from the initials of the three additive primary colors, red,
//! green, and blue. Meanwhile, the HSV representation models how colors appear under
//! light. In it, colors are represented using three components: hue, saturation and
//! (brightness-)value.
//!
//! References:
//! - https://en.wikipedia.org/wiki/RGB_color_model
//! - https://en.wikipedia.org/wiki/HSL_and_HSV
//! - https://www.rapidtables.com/convert/color/hsv-to-rgb.html

/// Errors that can occur during color conversion
#[derive(Debug, PartialEq)]
pub enum ColorError {
    /// Hue value is out of valid range [0, 360]
    InvalidHue(f64),
    /// Saturation value is out of valid range [0, 1]
    InvalidSaturation(f64),
    /// Value component is out of valid range [0, 1]
    InvalidValue(f64),
}

impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorError::InvalidHue(val) => write!(f, "hue should be between 0 and 360, got {val}"),
            ColorError::InvalidSaturation(val) => {
                write!(f, "saturation should be between 0 and 1, got {val}")
            }
            ColorError::InvalidValue(val) => {
                write!(f, "value should be between 0 and 1, got {val}")
            }
        }
    }
}

impl std::error::Error for ColorError {}

/// RGB color representation with red, green, and blue components (0-255)
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    /// Create a new RGB color
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }
}

/// HSV color representation with hue (0-360), saturation (0-1), and value (0-1)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}

impl Hsv {
    /// Create a new HSV color with validation
    pub fn new(hue: f64, saturation: f64, value: f64) -> Result<Self, ColorError> {
        if !(0.0..=360.0).contains(&hue) {
            return Err(ColorError::InvalidHue(hue));
        }
        if !(0.0..=1.0).contains(&saturation) {
            return Err(ColorError::InvalidSaturation(saturation));
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(ColorError::InvalidValue(value));
        }
        Ok(Hsv {
            hue,
            saturation,
            value,
        })
    }

    /// Check if two HSV colors are approximately equal
    ///
    /// Uses tolerance values:
    /// - Hue: 0.2 degrees
    /// - Saturation: 0.002
    /// - Value: 0.002
    pub fn approximately_equal(&self, other: &Hsv) -> bool {
        let hue_diff = (self.hue - other.hue).abs();
        let sat_diff = (self.saturation - other.saturation).abs();
        let val_diff = (self.value - other.value).abs();

        hue_diff < 0.2 && sat_diff < 0.002 && val_diff < 0.002
    }
}

/// Convert HSV color representation to RGB
///
/// Converts from HSV (Hue, Saturation, Value) to RGB (Red, Green, Blue).
///
/// # Arguments
///
/// * `hue` - Hue value in degrees (0-360)
/// * `saturation` - Saturation value (0-1)
/// * `value` - Value/brightness (0-1)
///
/// # Returns
///
/// * `Ok(Rgb)` - RGB color with components in range 0-255
/// * `Err(ColorError)` - If any input is out of valid range
pub fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> Result<Rgb, ColorError> {
    if !(0.0..=360.0).contains(&hue) {
        return Err(ColorError::InvalidHue(hue));
    }
    if !(0.0..=1.0).contains(&saturation) {
        return Err(ColorError::InvalidSaturation(saturation));
    }
    if !(0.0..=1.0).contains(&value) {
        return Err(ColorError::InvalidValue(value));
    }

    let chroma = value * saturation;
    let hue_section = hue / 60.0;
    let second_largest_component = chroma * (1.0 - ((hue_section % 2.0) - 1.0).abs());
    let match_value = value - chroma;

    let (red, green, blue) = if (0.0..=1.0).contains(&hue_section) {
        (
            chroma + match_value,
            second_largest_component + match_value,
            match_value,
        )
    } else if (1.0..=2.0).contains(&hue_section) {
        (
            second_largest_component + match_value,
            chroma + match_value,
            match_value,
        )
    } else if (2.0..=3.0).contains(&hue_section) {
        (
            match_value,
            chroma + match_value,
            second_largest_component + match_value,
        )
    } else if (3.0..=4.0).contains(&hue_section) {
        (
            match_value,
            second_largest_component + match_value,
            chroma + match_value,
        )
    } else if (4.0..=5.0).contains(&hue_section) {
        (
            second_largest_component + match_value,
            match_value,
            chroma + match_value,
        )
    } else {
        (
            chroma + match_value,
            match_value,
            second_largest_component + match_value,
        )
    };

    Ok(Rgb {
        red: (red * 255.0).round() as u8,
        green: (green * 255.0).round() as u8,
        blue: (blue * 255.0).round() as u8,
    })
}

/// Convert RGB color representation to HSV
///
/// Converts from RGB (Red, Green, Blue) to HSV (Hue, Saturation, Value).
///
/// # Arguments
///
/// * `red` - Red component (0-255)
/// * `green` - Green component (0-255)
/// * `blue` - Blue component (0-255)
///
/// # Returns
///
/// * `Ok(Hsv)` - HSV color with hue in [0, 360] and saturation/value in [0, 1]
pub fn rgb_to_hsv(red: u8, green: u8, blue: u8) -> Result<Hsv, ColorError> {
    let float_red = f64::from(red) / 255.0;
    let float_green = f64::from(green) / 255.0;
    let float_blue = f64::from(blue) / 255.0;

    let value = float_red.max(float_green).max(float_blue);
    let min_val = float_red.min(float_green).min(float_blue);
    let chroma = value - min_val;

    let saturation = if value == 0.0 { 0.0 } else { chroma / value };

    let hue = if chroma == 0.0 {
        0.0
    } else if (value - float_red).abs() < f64::EPSILON {
        60.0 * (0.0 + (float_green - float_blue) / chroma)
    } else if (value - float_green).abs() < f64::EPSILON {
        60.0 * (2.0 + (float_blue - float_red) / chroma)
    } else {
        60.0 * (4.0 + (float_red - float_green) / chroma)
    };

    let hue = (hue + 360.0) % 360.0;

    Ok(Hsv {
        hue,
        saturation,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_to_rgb_basic_colors() {
        // Black
        assert_eq!(hsv_to_rgb(0.0, 0.0, 0.0).unwrap(), Rgb::new(0, 0, 0));

        // White
        assert_eq!(hsv_to_rgb(0.0, 0.0, 1.0).unwrap(), Rgb::new(255, 255, 255));

        // Red
        assert_eq!(hsv_to_rgb(0.0, 1.0, 1.0).unwrap(), Rgb::new(255, 0, 0));

        // Yellow
        assert_eq!(hsv_to_rgb(60.0, 1.0, 1.0).unwrap(), Rgb::new(255, 255, 0));

        // Green
        assert_eq!(hsv_to_rgb(120.0, 1.0, 1.0).unwrap(), Rgb::new(0, 255, 0));

        // Blue
        assert_eq!(hsv_to_rgb(240.0, 1.0, 1.0).unwrap(), Rgb::new(0, 0, 255));

        // Magenta
        assert_eq!(hsv_to_rgb(300.0, 1.0, 1.0).unwrap(), Rgb::new(255, 0, 255));
    }

    #[test]
    fn test_hsv_to_rgb_intermediate_colors() {
        assert_eq!(hsv_to_rgb(180.0, 0.5, 0.5).unwrap(), Rgb::new(64, 128, 128));
        assert_eq!(
            hsv_to_rgb(234.0, 0.14, 0.88).unwrap(),
            Rgb::new(193, 196, 224)
        );
        assert_eq!(hsv_to_rgb(330.0, 0.75, 0.5).unwrap(), Rgb::new(128, 32, 80));
    }

    #[test]
    fn test_hsv_to_rgb_invalid_hue() {
        assert_eq!(
            hsv_to_rgb(-1.0, 0.5, 0.5),
            Err(ColorError::InvalidHue(-1.0))
        );
        assert_eq!(
            hsv_to_rgb(361.0, 0.5, 0.5),
            Err(ColorError::InvalidHue(361.0))
        );
    }

    #[test]
    fn test_hsv_to_rgb_invalid_saturation() {
        assert_eq!(
            hsv_to_rgb(180.0, -0.1, 0.5),
            Err(ColorError::InvalidSaturation(-0.1))
        );
        assert_eq!(
            hsv_to_rgb(180.0, 1.1, 0.5),
            Err(ColorError::InvalidSaturation(1.1))
        );
    }

    #[test]
    fn test_hsv_to_rgb_invalid_value() {
        assert_eq!(
            hsv_to_rgb(180.0, 0.5, -0.1),
            Err(ColorError::InvalidValue(-0.1))
        );
        assert_eq!(
            hsv_to_rgb(180.0, 0.5, 1.1),
            Err(ColorError::InvalidValue(1.1))
        );
    }

    #[test]
    fn test_rgb_to_hsv_basic_colors() {
        // Black
        let hsv = rgb_to_hsv(0, 0, 0).unwrap();
        assert!(Hsv::new(0.0, 0.0, 0.0).unwrap().approximately_equal(&hsv));

        // White
        let hsv = rgb_to_hsv(255, 255, 255).unwrap();
        assert!(Hsv::new(0.0, 0.0, 1.0).unwrap().approximately_equal(&hsv));

        // Red
        let hsv = rgb_to_hsv(255, 0, 0).unwrap();
        assert!(Hsv::new(0.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));

        // Yellow
        let hsv = rgb_to_hsv(255, 255, 0).unwrap();
        assert!(Hsv::new(60.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));

        // Green
        let hsv = rgb_to_hsv(0, 255, 0).unwrap();
        assert!(Hsv::new(120.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));

        // Blue
        let hsv = rgb_to_hsv(0, 0, 255).unwrap();
        assert!(Hsv::new(240.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));

        // Magenta
        let hsv = rgb_to_hsv(255, 0, 255).unwrap();
        assert!(Hsv::new(300.0, 1.0, 1.0).unwrap().approximately_equal(&hsv));
    }

    #[test]
    fn test_rgb_to_hsv_intermediate_colors() {
        let hsv = rgb_to_hsv(64, 128, 128).unwrap();
        assert!(Hsv::new(180.0, 0.5, 0.5).unwrap().approximately_equal(&hsv));

        let hsv = rgb_to_hsv(193, 196, 224).unwrap();
        assert!(Hsv::new(234.0, 0.14, 0.88)
            .unwrap()
            .approximately_equal(&hsv));

        let hsv = rgb_to_hsv(128, 32, 80).unwrap();
        assert!(Hsv::new(330.0, 0.75, 0.5)
            .unwrap()
            .approximately_equal(&hsv));
    }

    #[test]
    fn test_round_trip_conversion() {
        let test_cases = vec![
            (0.0, 0.0, 0.0),
            (0.0, 0.0, 1.0),
            (0.0, 1.0, 1.0),
            (60.0, 1.0, 1.0),
            (120.0, 1.0, 1.0),
            (240.0, 1.0, 1.0),
            (300.0, 1.0, 1.0),
            (180.0, 0.5, 0.5),
            (234.0, 0.14, 0.88),
            (330.0, 0.75, 0.5),
        ];

        for (hue, sat, val) in test_cases {
            let original_hsv = Hsv::new(hue, sat, val).unwrap();
            let rgb = hsv_to_rgb(hue, sat, val).unwrap();
            let converted_hsv = rgb_to_hsv(rgb.red, rgb.green, rgb.blue).unwrap();
            assert!(
                original_hsv.approximately_equal(&converted_hsv),
                "Round trip failed for HSV({hue}, {sat}, {val})"
            );
        }
    }

    #[test]
    fn test_approximately_equal_hsv() {
        let hsv1 = Hsv::new(0.0, 0.0, 0.0).unwrap();
        let hsv2 = Hsv::new(0.0, 0.0, 0.0).unwrap();
        assert!(hsv1.approximately_equal(&hsv2));

        let hsv1 = Hsv::new(180.0, 0.5, 0.3).unwrap();
        let hsv2 = Hsv::new(179.9999, 0.500001, 0.30001).unwrap();
        assert!(hsv1.approximately_equal(&hsv2));

        let hsv1 = Hsv::new(0.0, 0.0, 0.0).unwrap();
        let hsv2 = Hsv::new(1.0, 0.0, 0.0).unwrap();
        assert!(!hsv1.approximately_equal(&hsv2));

        let hsv1 = Hsv::new(180.0, 0.5, 0.3).unwrap();
        let hsv2 = Hsv::new(179.9999, 0.6, 0.30001).unwrap();
        assert!(!hsv1.approximately_equal(&hsv2));
    }

    #[test]
    fn test_hsv_new_validation() {
        assert!(Hsv::new(0.0, 0.0, 0.0).is_ok());
        assert!(Hsv::new(360.0, 1.0, 1.0).is_ok());
        assert_eq!(Hsv::new(-1.0, 0.5, 0.5), Err(ColorError::InvalidHue(-1.0)));
        assert_eq!(
            Hsv::new(361.0, 0.5, 0.5),
            Err(ColorError::InvalidHue(361.0))
        );
        assert_eq!(
            Hsv::new(180.0, -0.1, 0.5),
            Err(ColorError::InvalidSaturation(-0.1))
        );
        assert_eq!(
            Hsv::new(180.0, 1.1, 0.5),
            Err(ColorError::InvalidSaturation(1.1))
        );
        assert_eq!(
            Hsv::new(180.0, 0.5, -0.1),
            Err(ColorError::InvalidValue(-0.1))
        );
        assert_eq!(
            Hsv::new(180.0, 0.5, 1.1),
            Err(ColorError::InvalidValue(1.1))
        );
    }

    #[test]
    fn test_edge_cases() {
        // Hue = 360 should work (edge of valid range)
        assert!(hsv_to_rgb(360.0, 1.0, 1.0).is_ok());

        // Saturation and value at boundaries
        assert!(hsv_to_rgb(180.0, 0.0, 0.0).is_ok());
        assert!(hsv_to_rgb(180.0, 1.0, 1.0).is_ok());

        // All RGB values at max
        assert!(rgb_to_hsv(255, 255, 255).is_ok());

        // All RGB values at min
        assert!(rgb_to_hsv(0, 0, 0).is_ok());
    }

    #[test]
    fn test_rgb_struct() {
        let rgb = Rgb::new(100, 150, 200);
        assert_eq!(rgb.red, 100);
        assert_eq!(rgb.green, 150);
        assert_eq!(rgb.blue, 200);
    }
}
