#[derive(Debug, Clone)]
pub struct Job {
    pub id: String,
    pub deadline: usize,
    pub profit: i32,
}

pub fn job_sequencing(mut jobs: Vec<Job>) -> (Vec<Job>, i32) {
    if jobs.is_empty() {
        return (Vec::new(), 0);
    }

    // Sort by profit (descending)
    jobs.sort_by(|a, b| b.profit.cmp(&a.profit));

    // Find the maximum deadline
    let max_deadline = jobs.iter().map(|j| j.deadline).max().unwrap();

    // Track time slots that are filled
    let mut slots = vec![false; max_deadline];
    let mut scheduled = Vec::new();
    let mut total_profit = 0;

    // Schedule each job in the latest available slot before its deadline is reached
    for job in jobs {
        for slot in (0..job.deadline.min(max_deadline)).rev() {
            if !slots[slot] {
                slots[slot] = true;
                scheduled.push(job.clone());
                total_profit += job.profit;
                break;
            }
        }
    }

    (scheduled, total_profit)
}

// test algorithm
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let jobs = vec![
            Job {
                id: "J1".to_string(),
                deadline: 2,
                profit: 100,
            },
            Job {
                id: "J2".to_string(),
                deadline: 1,
                profit: 19,
            },
            Job {
                id: "J3".to_string(),
                deadline: 2,
                profit: 27,
            },
            Job {
                id: "J4".to_string(),
                deadline: 1,
                profit: 25,
            },
            Job {
                id: "J5".to_string(),
                deadline: 3,
                profit: 15,
            },
        ];

        let (scheduled, profit) = job_sequencing(jobs);

        assert_eq!(profit, 142);
        assert_eq!(scheduled.len(), 3);
    }

    #[test]
    fn test_empty() {
        let (scheduled, profit) = job_sequencing(vec![]);
        assert_eq!(profit, 0);
        assert!(scheduled.is_empty());
    }
}
