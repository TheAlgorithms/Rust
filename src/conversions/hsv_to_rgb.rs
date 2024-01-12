// The RGB color model is an additive color model in which red, green, and blue light
// are added together in various ways to reproduce a broad array of colors. The name
// of the model comes from the initials of the three additive primary colors, red,
// green, and blue. Meanwhile, the HSV representation models how colors appear under
// light. In it, colors are represented using three components: hue, saturation and
// (brightness-)value. This file provides functions for converting colors from one
// representation to the other.

pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> [i32; 3] {
    if !(0.0..=360.0).contains(&hue)
        || !(0.0..=1.0).contains(&saturation)
        || !(0.0..=1.0).contains(&value) 
    {
        panic!("Input values out of range");
    }

    let chroma = value * saturation;
    let hue_section = hue / 60.0;
    let second_largest_component = chroma * (1.0 - (hue_section % 2.0 - 1.0).abs());
    let match_value = value - chroma;

    let (red, green, blue) = if hue_section >= 0.0 && hue_section <= 1.0 {
        (chroma, second_largest_component, 0.0)
    } else if hue_section <= 2.0 {
        (second_largest_component, chroma, 0.0)
    } else if hue_section <= 3.0 {
        (0.0, chroma, second_largest_component)
    } else if hue_section <= 4.0 {
        (0.0, second_largest_component, chroma)
    } else if hue_section <= 5.0 {
        (second_largest_component, 0.0, chroma)
    } else {
        (chroma, 0.0, second_largest_component)
    };

    [
        ((red + match_value) * 255.0).round() as i32,
        ((green + match_value) * 255.0).round() as i32,
        ((blue + match_value) * 255.0).round() as i32,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_to_rgb() {
        assert_eq!(hsv_to_rgb(0.0, 0.0, 0.0), [0, 0, 0]); // black
        assert_eq!(hsv_to_rgb(0.0, 0.0, 1.0), [255, 255, 255]); // white
        assert_eq!(hsv_to_rgb(0.0, 1.0, 1.0), [255, 0, 0]); // red
        assert_eq!(hsv_to_rgb(60.0, 1.0, 1.0), [255, 255, 0]); // yellow
        assert_eq!(hsv_to_rgb(120.0, 1.0, 1.0), [0, 255, 0]); // green
        assert_eq!(hsv_to_rgb(240.0, 1.0, 1.0), [0, 0, 255]); // blue
        assert_eq!(hsv_to_rgb(300.0, 1.0, 1.0), [255, 0, 255]); // magenta
        assert_eq!(hsv_to_rgb(180.0, 0.5, 0.5), [64, 128, 128]); // teal
        assert_eq!(hsv_to_rgb(234.0, 0.14, 0.88), [193, 196, 224]); // light blue
        assert_eq!(hsv_to_rgb(330.0, 0.75, 0.5), [128, 32, 80]); // purple
    }
}
