//! # Gas Station Problem
//!
//! ## Problem Statement
//!
//! There are n gas stations along a circular route, where the amount of gas
//! at the ith station is `gas[i]`. You have a car with an unlimited gas tank
//! and it costs `cost[i]` of gas to travel from the ith station to its next
//! (i + 1)th station. You begin the journey with an empty tank at one of the
//! gas stations.
//!
//! Given two integer arrays `gas` and `cost`, return the starting gas station's
//! index if you can travel around the circuit once in the clockwise direction;
//! otherwise, return -1. If there exists a solution, it is guaranteed to be unique.
//!
//! ## Algorithm
//!
//! The solution uses a greedy approach:
//! 1. First, check whether the total gas is enough to complete the journey.
//!    If the sum of all gas is less than the sum of all costs, return -1.
//! 2. If there is enough gas total, there must be a valid starting index.
//! 3. Greedily calculate the net gain (gas - cost) at each station.
//! 4. If the net gain ever goes below 0 while iterating through the stations,
//!    the current starting point is invalid. Start checking from the next station.
//!
//! ## Complexity
//!
//! - Time complexity: O(n) where n is the number of gas stations
//! - Space complexity: O(1)
//!
//! ## References
//!
//! - [LeetCode Problem](https://leetcode.com/problems/gas-station/)

/// Represents a gas station with available gas and cost to travel to the next station
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GasStation {
    /// Amount of gas available at this station
    pub gas: i32,
    /// Cost of gas required to travel to the next station
    pub cost: i32,
}

impl GasStation {
    /// Creates a new gas station
    ///
    /// # Arguments
    ///
    /// * `gas` - Amount of gas available at this station
    /// * `cost` - Cost to travel to the next station
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::greedy::GasStation;
    /// let station = GasStation::new(5, 3);
    /// assert_eq!(station.gas, 5);
    /// assert_eq!(station.cost, 3);
    /// ```
    pub fn new(gas: i32, cost: i32) -> Self {
        Self { gas, cost }
    }

    /// Returns the net gain (gas - cost) for this station
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::greedy::GasStation;
    /// let station = GasStation::new(5, 3);
    /// assert_eq!(station.net_gain(), 2);
    /// ```
    pub fn net_gain(&self) -> i32 {
        self.gas - self.cost
    }
}

/// Creates a vector of gas stations from parallel arrays of gas quantities and costs
///
/// # Arguments
///
/// * `gas` - Array of gas quantities at each station
/// * `cost` - Array of costs to travel to the next station
///
/// # Panics
///
/// Panics if the lengths of `gas` and `cost` arrays don't match
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::greedy::{create_gas_stations, GasStation};
/// let stations = create_gas_stations(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]);
/// assert_eq!(stations.len(), 5);
/// assert_eq!(stations[0], GasStation::new(1, 3));
/// assert_eq!(stations[4], GasStation::new(5, 2));
/// ```
pub fn create_gas_stations(gas: &[i32], cost: &[i32]) -> Vec<GasStation> {
    assert_eq!(
        gas.len(),
        cost.len(),
        "gas and cost arrays must have the same length"
    );
    gas.iter()
        .zip(cost.iter())
        .map(|(&g, &c)| GasStation::new(g, c))
        .collect()
}

/// Finds the starting gas station index to complete the circular journey
///
/// Returns the index of the gas station from which to start the journey
/// in order to complete a full circuit. Returns -1 if it's impossible to
/// complete the journey.
///
/// # Arguments
///
/// * `stations` - Slice of gas stations along the circular route
///
/// # Returns
///
/// * Index of the starting station (0-indexed) if a solution exists
/// * -1 if no solution exists
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::greedy::{can_complete_journey, create_gas_stations};
/// // Case 1: Solution exists starting at index 3
/// let stations = create_gas_stations(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]);
/// assert_eq!(can_complete_journey(&stations), 3);
///
/// // Case 2: No solution exists
/// let stations = create_gas_stations(&[2, 3, 4], &[3, 4, 3]);
/// assert_eq!(can_complete_journey(&stations), -1);
///
/// // Case 3: Start at index 0
/// let stations = create_gas_stations(&[5, 1, 2, 3, 4], &[4, 4, 1, 5, 1]);
/// assert_eq!(can_complete_journey(&stations), 4);
/// ```
pub fn can_complete_journey(stations: &[GasStation]) -> i32 {
    // Calculate total gas and total cost
    let total_gas: i32 = stations.iter().map(|s| s.gas).sum();
    let total_cost: i32 = stations.iter().map(|s| s.cost).sum();

    // If total gas is less than total cost, impossible to complete journey
    if total_gas < total_cost {
        return -1;
    }

    // Since we have enough gas, a solution must exist
    // Use greedy approach to find the starting station
    let mut start = 0;
    let mut net = 0;

    for (i, station) in stations.iter().enumerate() {
        net += station.net_gain();

        // If net becomes negative, we can't reach here from current start
        // So try starting from the next station
        if net < 0 {
            start = i + 1;
            net = 0;
        }
    }

    start as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_station_creation() {
        let station = GasStation::new(10, 5);
        assert_eq!(station.gas, 10);
        assert_eq!(station.cost, 5);
        assert_eq!(station.net_gain(), 5);
    }

    #[test]
    fn test_create_gas_stations() {
        let stations = create_gas_stations(&[1, 2, 3], &[2, 1, 3]);
        assert_eq!(stations.len(), 3);
        assert_eq!(stations[0], GasStation::new(1, 2));
        assert_eq!(stations[1], GasStation::new(2, 1));
        assert_eq!(stations[2], GasStation::new(3, 3));
    }

    #[test]
    #[should_panic(expected = "gas and cost arrays must have the same length")]
    fn test_create_gas_stations_mismatched_lengths() {
        create_gas_stations(&[1, 2], &[1]);
    }

    #[test]
    fn test_can_complete_journey_solution_exists() {
        let stations = create_gas_stations(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]);
        assert_eq!(can_complete_journey(&stations), 3);
    }

    #[test]
    fn test_can_complete_journey_no_solution() {
        let stations = create_gas_stations(&[2, 3, 4], &[3, 4, 3]);
        assert_eq!(can_complete_journey(&stations), -1);
    }

    #[test]
    fn test_can_complete_journey_start_at_zero() {
        let stations = create_gas_stations(&[3, 1, 1], &[1, 2, 2]);
        assert_eq!(can_complete_journey(&stations), 0);
    }

    #[test]
    fn test_can_complete_journey_single_station() {
        let stations = create_gas_stations(&[5], &[3]);
        assert_eq!(can_complete_journey(&stations), 0);
    }

    #[test]
    fn test_can_complete_journey_single_station_insufficient() {
        let stations = create_gas_stations(&[2], &[3]);
        assert_eq!(can_complete_journey(&stations), -1);
    }

    #[test]
    fn test_can_complete_journey_two_stations() {
        let stations = create_gas_stations(&[1, 2], &[2, 1]);
        assert_eq!(can_complete_journey(&stations), 1);
    }

    #[test]
    fn test_can_complete_journey_all_equal() {
        let stations = create_gas_stations(&[2, 2, 2, 2], &[2, 2, 2, 2]);
        assert_eq!(can_complete_journey(&stations), 0);
    }

    #[test]
    fn test_can_complete_journey_large_numbers() {
        let stations = create_gas_stations(&[1000, 500, 300], &[600, 400, 300]);
        assert_eq!(can_complete_journey(&stations), 0);
    }

    #[test]
    fn test_can_complete_journey_negative_net_at_start() {
        let stations = create_gas_stations(&[1, 5, 3], &[3, 2, 4]);
        assert_eq!(can_complete_journey(&stations), 1);
    }
}
