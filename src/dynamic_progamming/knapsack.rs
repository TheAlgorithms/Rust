use std::cmp;

pub fn knapsack(weight: &i32, weights: &[i32], values: &[i32]) -> i32 {
    if weights.len() != values.len() {
        panic!("Weights and values must be of same size");
    }
    let weight = *weight as usize;
    let len = weights.len();

    let mut dp = vec![vec![0; weight + 1]; len + 1];

    for i in 0..(len + 1) {
        for wt in 0..(weight + 1) {
            if i == 0 || wt == 0 {
                dp[i][wt] = 0;
                continue;
            }

            // Try to take this item
            if wt as i32 >= weights[i - 1] {
                dp[i][wt] = values[i - 1] + dp[i - 1][wt - weights[i - 1] as usize];
            }

            // Skip this item
            dp[i][wt] = cmp::max(dp[i][wt], dp[i - 1][wt]);
        }
    }
    println!("{:?}", dp);
    dp[len][weight]
}

#[cfg(test)]
mod tests {
    use dynamic_progamming;
    #[test]
    fn knapsack() {
        let values = vec![60, 100, 120];
        let weights = vec![10, 20, 30];

        let sol = dynamic_progamming::knapsack::knapsack(&50, &weights, &values);
        assert_eq!(sol, 220);
    }
}
