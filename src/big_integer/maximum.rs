/// Compute the multiplication set of the three largest numbers in an array
///
// pub fn maximum(mut nums: Vec<i32>) -> i32 {
//     nums.sort();
//     (nums[nums.len() - 1] * nums[nums.len() - 2] * nums[nums.len() - 3])
//         .max(nums[0] * nums[nums.len() - 1] * nums[nums.len() - 2])
//         .max(nums[0] * nums[1] * nums[nums.len() - 1])
// }

pub fn maximum(nums: Vec<isize>) -> isize {
    let (mut s1, mut s2, mut b1, mut b2, mut b3) =
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN, isize::MIN);

    for n in nums {
        if n < s1 {
            s2 = s1;
            s1 = n;
        } else if n < s2 {
            s2 = n;
        }

        if n > b1 {
            b3 = b2;
            b2 = b1;
            b1 = n;
        } else if n > b2 {
            b3 = b2;
            b2 = n;
        } else if n > b3 {
            b3 = n;
        }
    }

    (s1 * s2 * b1).max(b2 * b3 * b1)
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_maximum {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, expected) = $inputs;
                assert_eq!(maximum(s.to_vec()), expected);
            }
        )*
        }
    }

    test_maximum! {
        three_positive_numbers: ([1, 2, 3], 6),
        four_positive_numbers: ([1, 2, 3, 4], 24),
        four_numbers_with_zero: ([-1, 2, 0, 4], 0),
        three_negative_numbers: ([-1, -2, -3], -6),
        four_negative_numbers: ([-1, -2, -3, -4], -6),
        five_negative_numbers: ([-1, -2, -3, -4, -5], -6),
        negative_numbers_with_zero: ([-1, -2, 0, -3, -4, -5], 0),
        four_negative_numbers_with_zero: ([-1, -2, 0, -3, -4], 0),
        mixed_positive_and_negative_numbers1: ([-1, -2, -3, 4, 5], 30),
        mixed_positive_and_negative_numbers2: ([-1, 2, -3, -4, 5], 60),
        mixed_positive_and_negative_numbers3: ([-1, 2, -3, 4, 5], 40),
        single_negative_number_repeated: ([-1, -1, -1, -1, -1], -1),
        all_zeros: ([0, 0, 0, 0, 0], 0),
        two_positive_one_negative: ([1, 2, -3], -6),
        two_negative_one_positive: ([-1, -2, 3], 6),
        large_numbers: ([1000, 1000, 10000], 10000000000),
        mixture_with_large_and_small_numbers: ([1, -1, 2, -2, 1000000], 2000000),
    }
}
