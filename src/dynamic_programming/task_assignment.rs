// Task Assignment Problem using Bitmasking and DP in Rust
// Time Complexity: O(2^M * N) where M is number of people and N is number of tasks
// Space Complexity: O(2^M * N) for the DP table

use std::collections::HashMap;

/// Solves the task assignment problem where each person can do only certain tasks,
/// each person can do only one task, and each task is performed by only one person.
/// Uses bitmasking and dynamic programming to count total number of valid assignments.
///
/// # Arguments
/// * `task_performed` - A vector of vectors where each inner vector contains tasks
///                      that a person can perform (1-indexed task numbers)
/// * `total_tasks` - The total number of tasks (N)
///
/// # Returns
/// * The total number of valid task assignments
pub fn count_task_assignments(task_performed: Vec<Vec<usize>>, total_tasks: usize) -> i64 {
    let num_people = task_performed.len();
    let dp_size = 1 << num_people;

    // Initialize DP table with -1 (uncomputed)
    let mut dp = vec![vec![-1; total_tasks + 2]; dp_size];

    let mut task_map = HashMap::new();
    let final_mask = (1 << num_people) - 1;

    // Build the task -> people mapping
    for (person, tasks) in task_performed.iter().enumerate() {
        for &task in tasks {
            task_map.entry(task).or_insert_with(Vec::new).push(person);
        }
    }

    // Recursive DP function
    fn count_ways_until(
        dp: &mut Vec<Vec<i64>>,
        task_map: &HashMap<usize, Vec<usize>>,
        final_mask: usize,
        total_tasks: usize,
        mask: usize,
        task_no: usize,
    ) -> i64 {
        // Base case: all people have been assigned tasks
        if mask == final_mask {
            return 1;
        }

        // Base case: no more tasks available but not all people assigned
        if task_no > total_tasks {
            return 0;
        }

        // Return cached result if already computed
        if dp[mask][task_no] != -1 {
            return dp[mask][task_no];
        }

        // Option 1: Skip the current task
        let mut total_ways =
            count_ways_until(dp, task_map, final_mask, total_tasks, mask, task_no + 1);

        // Option 2: Assign current task to a capable person who isn't busy
        if let Some(people) = task_map.get(&task_no) {
            for &person in people {
                // Check if this person is already assigned a task
                if mask & (1 << person) != 0 {
                    continue;
                }

                // Assign task to this person and recurse
                total_ways += count_ways_until(
                    dp,
                    task_map,
                    final_mask,
                    total_tasks,
                    mask | (1 << person),
                    task_no + 1,
                );
            }
        }

        // Cache the result
        dp[mask][task_no] = total_ways;
        total_ways
    }

    // Start recursion with no people assigned and first task
    count_ways_until(&mut dp, &task_map, final_mask, total_tasks, 0, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Macro to generate multiple test cases for the task assignment function
    macro_rules! task_assignment_tests {
        ($($name:ident: $input:expr => $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (task_performed, total_tasks) = $input;
                    assert_eq!(count_task_assignments(task_performed, total_tasks), $expected);
                }
            )*
        };
    }

    task_assignment_tests! {
        test_case_1: (vec![vec![1, 3, 4], vec![1, 2, 5], vec![3, 4]], 5) => 10,
        test_case_2: (vec![vec![1, 2], vec![1, 2]], 2) => 2,
        test_case_3: (vec![vec![1], vec![2], vec![3]], 3) => 1,
        test_case_4: (vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]], 3) => 6,
        test_case_5: (vec![vec![1], vec![1]], 1) => 0,

        // Edge test case
        test_case_single_person: (vec![vec![1, 2, 3]], 3) => 3,
    }
}
