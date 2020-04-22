use std::cmp::max;
pub struct Item {
    cost: u32,
    weight: u32,
}

pub fn knapsack(items: Vec<Item>, max_weight: u32) -> u32 {
    let mut dp = vec![vec![0; max_weight as usize + 1]; items.len() + 1];
    for (i, item) in items.iter().enumerate() {
        for w in 0..=max_weight as usize {
            if w < items[i].weight as usize {
                dp[i + 1][w] = dp[i][w];
            } else {
                dp[i + 1][w] = max(dp[i][w], dp[i][w - item.weight as usize] + item.cost)
            }
        }
    }

    dp[items.len()][max_weight as usize]
}

#[cfg(test)]
mod tests {
    use super::knapsack;
    use super::Item;
    #[test]
    fn test_knapsack() {
        assert_eq!(knapsack(vec![], 10), 0);
        assert_eq!(
            knapsack(
                vec![
                    Item {
                        cost: 70,
                        weight: 31
                    },
                    Item {
                        cost: 20,
                        weight: 10
                    },
                    Item {
                        cost: 39,
                        weight: 20
                    },
                    Item {
                        cost: 37,
                        weight: 19
                    },
                    Item { cost: 7, weight: 4 },
                    Item { cost: 5, weight: 3 },
                    Item {
                        cost: 10,
                        weight: 6
                    },
                ],
                50
            ),
            107
        );
        assert_eq!(
            knapsack(
                vec![
                    Item {
                        cost: 442,
                        weight: 41
                    },
                    Item {
                        cost: 525,
                        weight: 50
                    },
                    Item {
                        cost: 511,
                        weight: 49
                    },
                    Item {
                        cost: 593,
                        weight: 59
                    },
                    Item {
                        cost: 546,
                        weight: 55
                    },
                    Item {
                        cost: 564,
                        weight: 57
                    },
                    Item {
                        cost: 617,
                        weight: 60
                    },
                ],
                170
            ),
            1735
        )
    }
}
