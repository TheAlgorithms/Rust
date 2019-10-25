/// Coin Change Problem
/// The Coin Change Problem is a classic dynamic programming in which
/// you are given a target value and a list of possible coin values,
/// and you are tasked with finding the number of possible ways to use
/// the coins to generate the target value amount.
/// You can find the problem at https://www.hackerrank.com/challenges/coin-change/problem
/// You can use either iterative dynamic programming or recursive
/// dynamic programming.

/// coin_change_iterative(total, coins) will solve the problem using an
/// iterative approach and will return the number at the end.
pub fn coin_change_iterative(total: usize, coins: &Vec<usize>) -> usize {
    let mut coins = coins.clone();
    if coins.len() == 0 || coins[0] != 0 {
        coins.insert(0, 0);
    }

    let mut memo: Vec<Vec<usize>> = vec![vec![0; total + 1]; coins.len()];
    memo[0][0] = 1;
    for i in 1..coins.len() {
        let current_coin = coins[i];
        for j in 0..total + 1 {
            if j == 0 {
                memo[i][j] = 1;
            } else if j == current_coin {
                memo[i][j] = 1 + memo[i - 1][j];
            } else if j > current_coin {
                memo[i][j] = memo[i - 1][j] + memo[i][j - current_coin];
            } else {
                memo[i][j] = memo[i - 1][j];
            }
        }
    }

    memo[coins.len() - 1][total]
}

type Memo = Vec<Vec<Option<usize>>>;
/// solve_recursive(total, coins, memo, coin) is a helper method to
/// solve the Coin Change Problem in a recursive approach.
fn solve_recursive(
    total: usize,
    coins: &Vec<usize>,
    memo: &mut Memo,
    coin: usize,
    indent: usize,
) -> usize {
    if total == 0 {
        return 1;
    }

    if memo[coin][total].is_some() {
        return memo[coin][total].clone().unwrap();
    }

    let mut current_answer: usize = 0;
    for i in (0..coin + 1).rev() {
        if total >= coins[i] {
            current_answer += solve_recursive(total - coins[i], coins, memo, i, indent + 4);
        }
    }

    memo[coin][total] = Some(current_answer);
    current_answer
}

/// coin_change_recursive(total, coins) will solve the problem using the
/// recursive approach by using a helper function.
pub fn coin_change_recursive(total: usize, coins: &Vec<usize>) -> usize {
    let mut memo: Memo = vec![vec![None; total + 1]; coins.len()];
    solve_recursive(total, coins, &mut memo, coins.len() - 1, 0)
}

#[cfg(test)]
mod tests {
    use super::coin_change_iterative;
    use super::coin_change_recursive;

    fn test_coin_change_function(test_function: &dyn Fn(usize, &Vec<usize>) -> usize) {
        assert_eq!(5, test_function(5, &vec![1, 2, 3]));
        assert_eq!(4, test_function(4, &vec![1, 2, 3]));
        assert_eq!(3, test_function(3, &vec![1, 2, 3]));
        assert_eq!(2, test_function(2, &vec![1, 2, 3]));
        assert_eq!(1, test_function(1, &vec![1, 2, 3]));
        assert_eq!(1, test_function(0, &vec![1, 2, 3]));
        assert_eq!(3, test_function(12, &vec![3, 5, 6, 10]));
        assert_eq!(0, test_function(2, &vec![3, 5, 6, 10]));
        assert_eq!(0, test_function(4, &vec![3, 5, 6, 10]));
        assert_eq!(63, test_function(52, &vec![1, 5, 10, 25, 26]));
        assert_eq!(20, test_function(32, &vec![1, 5, 10, 25, 26]));
        assert_eq!(0, coin_change_iterative(5, &vec![]));
        assert_eq!(1, coin_change_iterative(0, &vec![]));
    }

    #[test]
    fn test_coin_change_iterative() {
        test_coin_change_function(&coin_change_iterative);
    }

    #[test]
    fn test_coin_change_recursive() {
        test_coin_change_function(&coin_change_recursive);
    }
}
