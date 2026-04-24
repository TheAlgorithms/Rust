//! Job Sequencing
//!
//! Given a set of jobs, each with a deadline and profit, schedule jobs to
//! maximise total profit. Each job takes exactly one unit of time and must
//! be completed on or before its deadline. Only one job can run at a time.
//!
//! # Algorithm (greedy)
//! 1. Sort jobs by profit in descending order.
//! 2. For each job (highest profit first), find the latest free time-slot
//!    that is ≤ the job's deadline and assign the job there.
//! 3. Return the sequence of scheduled jobs and the total profit earned.
//!
//! # Complexity
//! - Time:  O(n²) — for each of the n jobs we scan backwards through up to n slots.
//! - Space: O(n)  — slot array proportional to the maximum deadline.
//!
//! # References
//! - Cormen et al., *Introduction to Algorithms*, 4th ed., §16.5
//! - <https://en.wikipedia.org/wiki/Job-shop_scheduling>

/// A single job described by a name, a deadline (1-indexed, in time units),
/// and the profit earned if the job is completed on time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub name: String,
    pub deadline: usize,
    pub profit: u64,
}

impl Job {
    /// Constructs a new [`Job`].
    ///
    /// # Panics
    /// Panics if `deadline` is zero, because every job must be completable
    /// in at least the first time-slot.
    pub fn new(name: impl Into<String>, deadline: usize, profit: u64) -> Self {
        assert!(deadline >= 1, "deadline must be at least 1");
        Self {
            name: name.into(),
            deadline,
            profit,
        }
    }
}

/// Result returned by [`schedule_jobs`].
#[derive(Debug, PartialEq, Eq)]
pub struct ScheduleResult {
    /// Names of the scheduled jobs in slot order (slot 1 first).
    pub job_sequence: Vec<String>,
    /// Total profit from the scheduled jobs.
    pub total_profit: u64,
}

/// Schedules jobs to maximise total profit under deadline constraints.
///
/// Returns the optimal [`ScheduleResult`] — the scheduled job sequence
/// (in time-slot order) and the corresponding total profit.
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::greedy::{Job, schedule_jobs};
///
/// let jobs = vec![
///     Job::new("A", 2, 100),
///     Job::new("B", 1, 19),
///     Job::new("C", 2, 27),
///     Job::new("D", 1, 25),
///     Job::new("E", 3, 15),
/// ];
///
/// let result = schedule_jobs(jobs);
/// assert_eq!(result.total_profit, 142);
/// assert_eq!(result.job_sequence, vec!["C", "A", "E"]);
/// ```
pub fn schedule_jobs(mut jobs: Vec<Job>) -> ScheduleResult {
    if jobs.is_empty() {
        return ScheduleResult {
            job_sequence: vec![],
            total_profit: 0,
        };
    }

    // Step 1 – sort jobs by profit, highest first.
    jobs.sort_unstable_by(|a, b| b.profit.cmp(&a.profit));

    // Step 2 – allocate one slot per time-unit up to the maximum deadline.
    let max_deadline = jobs.iter().map(|j| j.deadline).max().unwrap_or(0);
    // slots[i] holds the name of the job assigned to time-slot (i + 1),
    // or None if the slot is still free.
    let mut slots: Vec<Option<String>> = vec![None; max_deadline];

    let mut total_profit: u64 = 0;

    for job in &jobs {
        // Find the latest free slot at or before this job's deadline.
        // Slots are 1-indexed in the problem but 0-indexed in our Vec.
        let deadline_idx = job.deadline; // exclusive upper bound for the range
        if let Some(slot) = (0..deadline_idx).rev().find(|&s| slots[s].is_none()) {
            slots[slot] = Some(job.name.clone());
            total_profit += job.profit;
        }
        // If no free slot is found the job is skipped (greedy choice).
    }

    // Step 3 – collect scheduled jobs in slot (time) order, skipping empty slots.
    let job_sequence = slots.into_iter().flatten().collect();

    ScheduleResult {
        job_sequence,
        total_profit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Helper
    // -----------------------------------------------------------------------

    fn make_result(jobs: &[&str], profit: u64) -> ScheduleResult {
        ScheduleResult {
            job_sequence: jobs.iter().map(|&s| s.to_string()).collect(),
            total_profit: profit,
        }
    }

    // -----------------------------------------------------------------------
    // Basic correctness
    // -----------------------------------------------------------------------

    /// Classic textbook example from Cormen et al. §16.5.
    #[test]
    fn test_classic_example() {
        let jobs = vec![
            Job::new("A", 2, 100),
            Job::new("B", 1, 19),
            Job::new("C", 2, 27),
            Job::new("D", 1, 25),
            Job::new("E", 3, 15),
        ];
        // Optimal: A in slot 2, C in slot 1 (or same profit arrangement),
        // and E in slot 3 → total = 100 + 27 + 15 = 142.
        let result = schedule_jobs(jobs);
        assert_eq!(result.total_profit, 142);
        assert_eq!(result.job_sequence, vec!["C", "A", "E"]);
    }

    /// All jobs have the same deadline (1) — only the most profitable fits.
    #[test]
    fn test_all_same_deadline() {
        let jobs = vec![
            Job::new("X", 1, 50),
            Job::new("Y", 1, 80),
            Job::new("Z", 1, 30),
        ];
        let result = schedule_jobs(jobs);
        assert_eq!(result, make_result(&["Y"], 80));
    }

    /// Every job can be scheduled (all deadlines are distinct and large enough).
    #[test]
    fn test_all_jobs_scheduled() {
        let jobs = vec![
            Job::new("P", 3, 10),
            Job::new("Q", 2, 20),
            Job::new("R", 1, 30),
        ];
        // R (profit 30) → slot 1, Q (profit 20) → slot 2, P (profit 10) → slot 3.
        let result = schedule_jobs(jobs);
        assert_eq!(result, make_result(&["R", "Q", "P"], 60));
    }

    // -----------------------------------------------------------------------
    // Edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_input() {
        let result = schedule_jobs(vec![]);
        assert_eq!(result, make_result(&[], 0));
    }

    #[test]
    fn test_single_job() {
        let result = schedule_jobs(vec![Job::new("Solo", 1, 42)]);
        assert_eq!(result, make_result(&["Solo"], 42));
    }

    /// Jobs with equal profit: the algorithm must still produce a valid
    /// (though not necessarily unique) schedule with the correct total profit.
    #[test]
    fn test_equal_profits() {
        let jobs = vec![
            Job::new("A", 1, 10),
            Job::new("B", 2, 10),
            Job::new("C", 3, 10),
        ];
        let result = schedule_jobs(jobs);
        // All three should be scheduled since their deadlines are distinct.
        assert_eq!(result.total_profit, 30);
        assert_eq!(result.job_sequence.len(), 3);
    }

    /// A large deadline value — verifies that the slot array is sized correctly
    /// and that jobs close to the deadline are still placed properly.
    #[test]
    fn test_large_deadline() {
        let jobs = vec![Job::new("Big", 100, 500), Job::new("Small", 1, 1)];
        let result = schedule_jobs(jobs);
        // Both jobs should be scheduled.
        assert_eq!(result.total_profit, 501);
        assert_eq!(result.job_sequence.len(), 2);
        // "Small" is in slot 1, "Big" somewhere ≤ 100.
        assert_eq!(result.job_sequence[0], "Small");
    }

    /// Zero-profit jobs should still be scheduled if a slot is available,
    /// because the greedy criterion is profit and zero is a valid profit.
    #[test]
    fn test_zero_profit_job() {
        let jobs = vec![Job::new("Free", 2, 0), Job::new("Paid", 1, 5)];
        let result = schedule_jobs(jobs);
        assert_eq!(result.total_profit, 5);
        // "Free" should still occupy slot 2 (no conflict).
        assert_eq!(result.job_sequence.len(), 2);
    }

    /// Verify that the returned sequence is in ascending slot order.
    #[test]
    fn test_output_in_slot_order() {
        let jobs = vec![
            Job::new("Late", 3, 5),
            Job::new("Mid", 2, 10),
            Job::new("Early", 1, 15),
        ];
        let result = schedule_jobs(jobs);
        assert_eq!(result.job_sequence, vec!["Early", "Mid", "Late"]);
        assert_eq!(result.total_profit, 30);
    }

    /// More jobs than slots — ensure that only as many jobs as there are
    /// time-slots can be scheduled.
    #[test]
    fn test_more_jobs_than_slots() {
        // 5 jobs, max deadline 2 → at most 2 can be scheduled.
        let jobs = vec![
            Job::new("A", 1, 40),
            Job::new("B", 2, 30),
            Job::new("C", 1, 20),
            Job::new("D", 2, 15),
            Job::new("E", 1, 10),
        ];
        let result = schedule_jobs(jobs);
        assert_eq!(result.total_profit, 70); // A (40) + B (30)
        assert_eq!(result.job_sequence.len(), 2);
    }
}
