//! Coin Change Problem
//!
//! # Algorithm
//!
//! The vector coins holds all types of coins.
//! The input argument n represents the value which has to be put together with the coins array.
//! The function returns the amount of combinations that are possible.

/// Solves the coin change problem
/// 
/// Returns the amount of combinations a price can be composed with a differently weighted coins.
/// 
/// See [coin change problem](https://en.wikipedia.org/wiki/Change-making_problem) for the theoretical background.
/// 
/// # Arguments
/// 
/// `n` - the price as usize
/// `coins` - the different coins as Vector of usize
/// 
/// # Returns
/// 
/// `combinations` - the amount of combinations as usize
/// 
/// # Panic
///
/// This function won't panic
///
/// # Examples
/// 
/// let combinations = coin_problem(12, &mut vec![1, 5, 10]);
/// 
pub fn coin_problem(n: usize, coins: &mut Vec<usize>) -> usize {
    // create the combinations vector
    let mut combinations = vec![0; n + 1];

    // set first combination to 1 because there is one way to make 0 with 0 coins
    combinations[0] = 1;

    // iterate over every coin in the vector
    for i in 0..coins.len() {
        // compare each index value of combinations with the coin value
        for j in 0..combinations.len() {
            if coins[i] <= j {
                // update the combinations array
                combinations[j] = combinations[j] + combinations[(j - coins[i])];
            }
        }
    }

    // return the amount of combinations at the position of n
    combinations[n]
}

#[cfg(test)]
mod test {
    use super::coin_problem;

    #[test]
    fn test_coin_problem() {
        assert_eq!(4, coin_problem(12, &mut vec![1, 5, 10]));
        assert_eq!(0, coin_problem(3, &mut vec![2, 5, 10]));
        assert_eq!(6, coin_problem(5, &mut vec![1, 2, 3, 4]));
    }
}
