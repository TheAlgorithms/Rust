/// Author : https://github.com/ali77gh\
/// References:\
/// RGB:  https://en.wikipedia.org/wiki/RGB_color_model\
/// CMYK: https://en.wikipedia.org/wiki/CMYK_color_model\

/// This function Converts RGB to CMYK format
///
/// ### Params
/// * `r` - red
/// * `g` - green
/// * `b` - blue
///
/// ### Returns
/// (C, M, Y, K)
pub fn rgb_to_cmyk(rgb: (u8, u8, u8)) -> (u8, u8, u8, u8) {
    // Safety: no need to check if input is positive and less than 255 because it's u8

    // change scale from [0,255] to [0,1]
    let (r, g, b) = (r as f64 / 255f64, g as f64 / 255f64, b as f64 / 255f64);

    match 1f64 - r.max(g).max(b) {
        1f64 => (0, 0, 0, 100), // pure black
        k => (
            (100f64 * (1f64 - r - k) / (1f64 - k)) as u8, // c
            (100f64 * (1f64 - g - k) / (1f64 - k)) as u8, // m
            (100f64 * (1f64 - b - k) / (1f64 - k)) as u8, // y
            (100f64 * k) as u8,                           // k
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_to_cmyk_test() {
        // white
        assert_eq!(rgb_to_cmyk(255, 255, 255), (0, 0, 0, 0));

        // gray
        assert_eq!(rgb_to_cmyk(128, 128, 128), (0, 0, 0, 49));

        // black
        assert_eq!(rgb_to_cmyk(0, 0, 0), (0, 0, 0, 100));

        // red
        assert_eq!(rgb_to_cmyk(255, 0, 0), (0, 100, 100, 0));

        // green
        assert_eq!(rgb_to_cmyk(0, 255, 0), (100, 0, 100, 0));

        // blue
        assert_eq!(rgb_to_cmyk(0, 0, 255), (100, 100, 0, 0));
    }
}
