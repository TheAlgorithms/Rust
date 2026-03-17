//! # Peak Signal-to-Noise Ratio (PSNR)
//!
//! Measures the quality of a reconstructed or compressed image relative to the original.
//! A higher PSNR generally indicates better quality.
//!
//! Reference: <https://en.wikipedia.org/wiki/Peak_signal-to-noise_ratio>

const PIXEL_MAX: f64 = 255.0;

/// Computes the PSNR in decibels (dB) between an original and a compressed image.
///
/// # Arguments
/// * `original` - Pixel values of the original image (u8 slice, any channel layout)
/// * `compressed` - Pixel values of the compressed/reconstructed image (same length)
///
/// # Returns
/// * `f64::INFINITY` when the images are identical (MSE = 0)
/// * Otherwise the PSNR value in dB
///
/// # Panics
/// Panics if `original` and `compressed` have different lengths.
pub fn peak_signal_to_noise_ratio(original: &[u8], compressed: &[u8]) -> f64 {
    assert_eq!(
        original.len(),
        compressed.len(),
        "original and compressed images must have the same number of pixels"
    );

    let mse: f64 = original
        .iter()
        .zip(compressed.iter())
        .map(|(&o, &c)| {
            let diff = o as f64 - c as f64;
            diff * diff
        })
        .sum::<f64>()
        / original.len() as f64;

    if mse == 0.0 {
        return f64::INFINITY;
    }

    20.0 * (PIXEL_MAX / mse.sqrt()).log10()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_images_returns_infinity() {
        let img = vec![100u8, 150, 200, 50, 75, 25];
        assert_eq!(peak_signal_to_noise_ratio(&img, &img), f64::INFINITY);
    }

    #[test]
    fn single_pixel_off_by_one() {
        // original: [0], compressed: [1]  →  MSE = 1.0  →  PSNR = 20·log10(255) ≈ 48.13 dB
        let original = vec![0u8];
        let compressed = vec![1u8];
        let psnr = peak_signal_to_noise_ratio(&original, &compressed);
        let expected = 20.0 * 255.0_f64.log10();
        assert!((psnr - expected).abs() < 1e-6, "got {psnr}");
    }

    #[test]
    fn uniform_noise() {
        // original all-zero, compressed all-10  →  MSE = 100  →  PSNR = 20·log10(255/10) ≈ 28.13 dB
        let original = vec![0u8; 16];
        let compressed = vec![10u8; 16];
        let psnr = peak_signal_to_noise_ratio(&original, &compressed);
        let expected = 20.0 * (PIXEL_MAX / 10.0).log10();
        assert!((psnr - expected).abs() < 1e-6, "got {psnr}");
    }

    #[test]
    fn known_psnr_value() {
        // 4 pixels: diffs = [15, 15, 15, 15]  →  MSE = 225  →  PSNR ≈ 24.61 dB
        let original = vec![0u8, 0, 0, 0];
        let compressed = vec![15u8, 15, 15, 15];
        let psnr = peak_signal_to_noise_ratio(&original, &compressed);
        let expected = 20.0 * (PIXEL_MAX / 15.0).log10();
        assert!((psnr - expected).abs() < 1e-6, "got {psnr}");
    }

    #[test]
    #[should_panic(expected = "same number of pixels")]
    fn mismatched_lengths_panics() {
        let original = vec![0u8; 4];
        let compressed = vec![0u8; 8];
        peak_signal_to_noise_ratio(&original, &compressed);
    }
}
