/*
Given a string s, find the length of the longest substring that does not contain repeated characters
Example 1.
Input: s = "abcabcbb"
Output: 3
Explanation: Since the longest substring without repeated characters is "abc", its length is 3.

Example 2.
Input: s = "bbbbb"
Output: 1
Explanation: Since the longest substring without repeating characters is "b", its length is 1.

Example 3.
Input: s = "pwwkew"
Output: 3
Explanation: Since the longest substring without repeated characters is "wke", its length is 3.
Note that your answer must be the length of the substring; "pwke" is a subsequence, not a substring.
*/

pub fn length_of_longest_substring(s: String) -> i32 {
    if !s.is_ascii() {
        panic!("{}", "Please enter ascii-compliant characters");
    }
    let s = s.as_bytes();
    let mut ans = 0;
    let mut left = 0;
    let mut window = [false; 128];
    for (right, &c) in s.iter().enumerate() {
        let c = c as usize;
        while window[c] {
            // After adding c, there will be duplicate elements in the window
            window[s[left] as usize] = false;
            left += 1;
        }
        window[c] = true;
        ans = ans.max(right - left + 1); // Update window length maximum
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn example_one() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3_i32);
    }

    #[test]
    fn example_two() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1_i32);
    }

    #[test]
    fn example_three() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3_i32);
    }
}
