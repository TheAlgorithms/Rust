/// Ternary search algorithm for finding maximum of unimodal function
pub fn ternary_search_max(
    f: fn(f32) -> f32,
    mut start: f32,
    mut end: f32,
    absolute_precision: f32,
) -> f32 {
    while (start - end).abs() >= absolute_precision {
        let mid1 = start + (end - start) / 3.0;
        let mid2 = end - (end - start) / 3.0;

        let r1 = f(mid1);
        let r2 = f(mid2);

        if r1 < r2 {
            start = mid1;
        } else if r1 > r2 {
            end = mid2;
        } else {
            start = mid1;
            end = mid2;
        }
    }
    f(start)
}

/// Ternary search algorithm for finding minimum of unimodal function
pub fn ternary_search_min(
    f: fn(f32) -> f32,
    mut start: f32,
    mut end: f32,
    absolute_precision: f32,
) -> f32 {
    while (start - end).abs() >= absolute_precision {
        let mid1 = start + (end - start) / 3.0;
        let mid2 = end - (end - start) / 3.0;

        let r1 = f(mid1);
        let r2 = f(mid2);

        if r1 < r2 {
            end = mid2;
        } else if r1 > r2 {
            start = mid1;
        } else {
            start = mid1;
            end = mid2;
        }
    }
    f(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max_value() {
        let expected = 4.0;
        let f = |x: f32| -x * x - 2.0 * x + 3.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.0000001;

        let result = ternary_search_max(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value() {
        let expected = 2.0;
        let f = |x: f32| x * x - 2.0 * x + 3.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.0000001;

        let result = ternary_search_min(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_max_value_2() {
        let expected = 7.25;
        let f = |x: f32| -x.powi(2) + 3.0 * x + 5.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.000001;

        let result = ternary_search_max(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value_2() {
        let expected = 2.75;
        let f = |x: f32| x.powi(2) + 3.0 * x + 5.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.000001;

        let result = ternary_search_min(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }
}
