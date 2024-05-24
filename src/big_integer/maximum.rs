/// Compute the multiplication set of the three largest numbers in an array
pub fn maximum(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    (nums[nums.len() - 1] * nums[nums.len() - 2] * nums[nums.len() - 3])
        .max(nums[0] * nums[nums.len() - 1] * nums[nums.len() - 2])
        .max(nums[0] * nums[1] * nums[nums.len() - 1])
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
        positive0: ([1,2,3], 6),
        positive1: ([1,2,3,4], 24),
        negative0: ([-1,-2,-3], -6),
        negative1: ([-1,-2,-3,-4,-5], -6),
        positive_and_negative0: ([-1,-2,-3,4,5], 30),
        positive_and_negative1: ([-1,2,-3,-4,5], 60),
        positive_and_negative2: ([-1,2,-3,4,5], 40),
    }
}
