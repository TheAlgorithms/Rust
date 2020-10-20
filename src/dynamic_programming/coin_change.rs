/// Coin change via Dynamic Programming

/// coin_change(amount, coins) returns how many ways there are make a
/// change of the given amount using the given coins.
pub fn coin_change(amount: usize, coins: &[usize]) -> usize {
    // sub_problems[target] is the number of ways to make `target`
    // with the coins iterated on so far
    let mut sub_problems;

    // without any coin, there is exactly one way to make no change
    // and no way to make any other change
    sub_problems = vec![0; amount + 1];
    sub_problems[0] = 1;

    for coin in coins {
        if *coin == 0 {
            continue;
        }
        // update the sub_problems with ways to make the target amount
        // with coin
        for target in *coin..=amount {
            sub_problems[target] += sub_problems[target - coin]
        }
    }

    sub_problems[amount]
}

#[cfg(test)]
mod tests {
    use super::coin_change;

    fn single_test_coin_change(expected: usize, amount: usize, mut coins: Vec<usize>) {
        assert_eq!(expected, coin_change(amount, &coins));
        // works as well when the order of coins changes

        coins.reverse();
        assert_eq!(expected, coin_change(amount, &coins));

        for _ in 0..coins.len() {
            coins.rotate_left(1);
            assert_eq!(expected, coin_change(amount, &coins));
        }
    }

    #[test]
    fn test_coin_change() {
        // simple cases
        single_test_coin_change(0, 2, vec![3]);
        single_test_coin_change(1, 0, vec![0]);
        single_test_coin_change(1, 0, vec![1, 2, 3]);
        single_test_coin_change(1, 5, vec![5]);
        single_test_coin_change(0, 6, vec![5]);
        single_test_coin_change(1, 4, vec![2]);

        // recursive case
        single_test_coin_change(4, 4, vec![1, 2, 3]);
        single_test_coin_change(8, 7, vec![1, 2, 3]);
        single_test_coin_change(5, 10, vec![2, 3, 5, 6]);
    }
}
