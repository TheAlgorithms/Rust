/*
Inspired by Leetcode 312 : https://leetcode.com/problems/burst-balloons/

You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number on it represented by an array nums. You are asked to burst all the balloons.

If you burst the ith balloon, you will get nums[i - 1] * nums[i] * nums[i + 1] coins. If i - 1 or i + 1 goes out of bounds of the array, then treat it as if there is a balloon with a 1 painted on it.

Return the maximum coins you can collect by bursting the balloons wisely.

Example 1:

Input: nums = [3,1,5,8]
Output: 167
Explanation:
nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
Example 2:

Input: nums = [1,5]
Output: 10
 

Constraints:

n == nums.length
1 <= n <= 300
0 <= nums[i] <= 100

 */

 use std::cmp::max;

struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f = vec![vec![0; n]; n];
        
        // f(i,j) = if we only play in the range [i,j], what is the maximum score?
        for len in 0..n {
            for i in 0..n - len {
                let j = i + len;
                for x in i..=j {
                    // In the range [i,j], if we burst balloon x last, what is the maximum score?
                    let mut score = nums[x];
                    if i > 0 {
                        score *= nums[i - 1];
                    }
                    if j < n - 1 {
                        score *= nums[j + 1];
                    }
                    if x > i {
                        score += f[i][x - 1];
                    }
                    if x < j {
                        score += f[x + 1][j];
                    }
                    f[i][j] = max(f[i][j], score);
                }
            }
        }
        f[0][n - 1]
    }
}

fn main() {
    let test_cases = vec![
        vec![3, 1, 5, 8],  // Example from the problem
        vec![1, 5],        // Small case
        vec![1, 2, 3],     // Incremental values
        vec![3, 1, 5],     // Another small case
        vec![7, 9, 8, 0, 7], // Mixed values
        vec![9, 76, 64, 21], // Larger values
    ];

    for (i, nums) in test_cases.iter().enumerate() {
        println!("Test case {}: nums = {:?}, max coins = {}", i + 1, nums, Solution::max_coins(nums.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_coins() {
        let test_cases = vec![
            (vec![3, 1, 5, 8], 167),
            (vec![1, 5], 10),
            (vec![1, 2, 3], 12),
            (vec![3, 1, 5], 35),
            (vec![7, 9, 8, 0, 7], 665),
            (vec![9, 76, 64, 21], 123408),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(Solution::max_coins(nums), expected);
        }
    }
}
