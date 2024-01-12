// Simple RGB to CMYK conversion. Returns percentages of CMYK paint.
// https://www.programmingalgorithms.com/algorithm/rgb-to-cmyk/

// Note: this is a very popular algorithm that converts colors linearly and gives
// only approximate results. Actual preparation for printing requires advanced color
// conversion considering the color profiles and parameters of the target device.

pub fn rgb_to_cmyk(r_input: u8, g_input: u8, b_input: u8) -> (i32, i32, i32, i32) {
    // Convert RGB values to the range 0..1
    let r = r_input as f32 / 255.0;
    let g = g_input as f32 / 255.0;
    let b = b_input as f32 / 255.0;

    let k = 1.0 - r.max(g).max(b);

    if k == 1.0 {
        return (0, 0, 0, 100); // pure black
    }

    let c = ((1.0 - r - k) / (1.0 - k) * 100.0).round() as i32;
    let m = ((1.0 - g - k) / (1.0 - k) * 100.0).round() as i32;
    let y = ((1.0 - b - k) / (1.0 - k) * 100.0).round() as i32;
    let k = (k * 100.0).round() as i32;

    (c, m, y, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_cmyk() {
        // White
        assert_eq!(rgb_to_cmyk(255, 255, 255), (0, 0, 0, 0));

        // Gray
        assert_eq!(rgb_to_cmyk(128, 128, 128), (0, 0, 0, 50));

        // Black
        assert_eq!(rgb_to_cmyk(0, 0, 0), (0, 0, 0, 100));

        // Primary colors
        assert_eq!(rgb_to_cmyk(255, 0, 0), (0, 100, 100, 0)); // Red
        assert_eq!(rgb_to_cmyk(0, 255, 0), (100, 0, 100, 0)); // Green
        assert_eq!(rgb_to_cmyk(0, 0, 255), (100, 100, 0, 0)); // Blue

        // Secondary colors
        assert_eq!(rgb_to_cmyk(255, 255, 0), (0, 0, 100, 0)); // Yellow
        assert_eq!(rgb_to_cmyk(0, 255, 255), (100, 0, 0, 0)); // Cyan
        assert_eq!(rgb_to_cmyk(255, 0, 255), (0, 100, 0, 0)); // Magenta

        // Additional colors
        assert_eq!(rgb_to_cmyk(102, 204, 0), (50, 0, 100, 20)); // Light green
        assert_eq!(rgb_to_cmyk(191, 64, 191), (0, 66, 0, 25));  // Purple
    }
}
