//! Ant Colony Optimization (ACO) algorithm for solving the Travelling Salesman Problem (TSP).
//!
//! The Travelling Salesman Problem asks: "Given a list of cities and the distances between
//! each pair of cities, what is the shortest possible route that visits each city exactly
//! once and returns to the origin city?"
//!
//! The ACO algorithm uses artificial ants that build solutions iteratively. Each ant constructs
//! a tour by probabilistically choosing the next city based on pheromone trails and heuristic
//! information (distance). After all ants complete their tours, pheromone trails are updated,
//! with stronger pheromones deposited on shorter routes. Over multiple iterations, this process
//! converges toward finding good solutions to the TSP.
//!
//! # References
//! - [Ant Colony Optimization Algorithms](https://en.wikipedia.org/wiki/Ant_colony_optimization_algorithms)
//! - [Travelling Salesman Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem)

use rand::RngExt;
use std::collections::HashSet;

/// Represents a 2D city with coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
struct City {
    x: f64,
    y: f64,
}

impl City {
    /// Calculate Euclidean distance to another city
    fn distance_to(&self, other: &City) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Ant Colony Optimization solver for the Travelling Salesman Problem
struct AntColonyOptimization {
    cities: Vec<City>,
    pheromones: Vec<Vec<f64>>,
    num_ants: usize,
    num_iterations: usize,
    evaporation_rate: f64,
    pheromone_influence: f64,
    distance_influence: f64,
    pheromone_constant: f64,
}

impl AntColonyOptimization {
    /// Create a new ACO solver with the given cities and parameters
    fn new(
        cities: Vec<City>,
        num_ants: usize,
        num_iterations: usize,
        evaporation_rate: f64,
        pheromone_influence: f64,
        distance_influence: f64,
        pheromone_constant: f64,
    ) -> Self {
        let n = cities.len();
        let pheromones = vec![vec![1.0; n]; n];
        Self {
            cities,
            pheromones,
            num_ants,
            num_iterations,
            evaporation_rate,
            pheromone_influence,
            distance_influence,
            pheromone_constant,
        }
    }

    /// Run the ACO algorithm and return the best solution found
    fn solve(&mut self) -> Option<(Vec<usize>, f64)> {
        if self.cities.is_empty() {
            return None;
        }

        let mut best_route: Vec<usize> = Vec::new();
        let mut best_distance = f64::INFINITY;

        for _ in 0..self.num_iterations {
            let routes = self.construct_solutions();

            for route in &routes {
                let distance = self.calculate_route_distance(route);
                if distance < best_distance {
                    best_distance = distance;
                    best_route.clone_from(route);
                }
            }

            self.update_pheromones(&routes);
        }

        if best_route.is_empty() {
            None
        } else {
            Some((best_route, best_distance))
        }
    }

    /// Construct solutions for all ants in one iteration
    fn construct_solutions(&self) -> Vec<Vec<usize>> {
        (0..self.num_ants)
            .map(|_| self.construct_ant_solution())
            .collect()
    }

    /// Construct a solution for a single ant
    fn construct_ant_solution(&self) -> Vec<usize> {
        let n = self.cities.len();
        let mut route = Vec::with_capacity(n + 1);
        let mut unvisited: HashSet<usize> = (0..n).collect();

        // Start at city 0
        let mut current = 0;
        route.push(current);
        unvisited.remove(&current);

        // Visit remaining cities
        while !unvisited.is_empty() {
            current = self.select_next_city(current, &unvisited);
            route.push(current);
            unvisited.remove(&current);
        }

        // Return to starting city
        route.push(0);
        route
    }

    /// Select the next city to visit based on pheromone and distance
    fn select_next_city(&self, current: usize, unvisited: &HashSet<usize>) -> usize {
        let probabilities: Vec<(usize, f64)> = unvisited
            .iter()
            .map(|&city| {
                let pheromone = self.pheromones[current][city];
                let distance = self.cities[current].distance_to(&self.cities[city]);
                let heuristic = 1.0 / distance;

                let probability = pheromone.powf(self.pheromone_influence)
                    * heuristic.powf(self.distance_influence);

                (city, probability)
            })
            .collect();

        // Roulette wheel selection
        let total: f64 = probabilities.iter().map(|(_, p)| p).sum();
        let mut rng = rand::rng();
        let mut random_value = rng.random::<f64>() * total;

        for (city, prob) in probabilities {
            random_value -= prob;
            if random_value <= 0.0 {
                return city;
            }
        }

        // Fallback to last city if rounding errors occur
        *unvisited.iter().next().unwrap()
    }

    /// Calculate the total distance of a route
    fn calculate_route_distance(&self, route: &[usize]) -> f64 {
        route
            .windows(2)
            .map(|pair| self.cities[pair[0]].distance_to(&self.cities[pair[1]]))
            .sum()
    }

    /// Update pheromone trails based on ant solutions
    fn update_pheromones(&mut self, routes: &[Vec<usize>]) {
        let n = self.cities.len();

        // Evaporate pheromones
        for i in 0..n {
            for j in 0..n {
                self.pheromones[i][j] *= self.evaporation_rate;
            }
        }

        // Deposit new pheromones
        for route in routes {
            let distance = self.calculate_route_distance(route);
            let deposit = self.pheromone_constant / distance;

            for pair in route.windows(2) {
                let (i, j) = (pair[0], pair[1]);
                self.pheromones[i][j] += deposit;
                self.pheromones[j][i] += deposit; // Symmetric for undirected graph
            }
        }
    }
}

/// Solve the Travelling Salesman Problem using Ant Colony Optimization.
///
/// Given a list of cities (as (x, y) coordinates), finds a near-optimal route
/// that visits each city exactly once and returns to the starting city.
///
/// # Arguments
///
/// * `cities` - Vector of (x, y) coordinate tuples representing city locations
/// * `num_ants` - Number of ants per iteration (default: 10)
/// * `num_iterations` - Number of iterations to run (default: 20)
/// * `evaporation_rate` - Pheromone evaporation rate 0.0-1.0 (default: 0.7)
/// * `alpha` - Influence of pheromone on decision making (default: 1.0)
/// * `beta` - Influence of distance on decision making (default: 5.0)
/// * `q` - Pheromone deposit constant (default: 10.0)
///
/// # Returns
///
/// `Some((route, distance))` where route is a vector of city indices and distance
/// is the total route length, or `None` if the cities list is empty.
///
/// # Example
///
/// ```
/// use the_algorithms_rust::graph::ant_colony_optimization;
///
/// let cities = vec![
///     (0.0, 0.0),
///     (0.0, 5.0),
///     (3.0, 8.0),
///     (8.0, 10.0),
/// ];
///
/// let result = ant_colony_optimization(cities, 10, 20, 0.7, 1.0, 5.0, 10.0);
/// if let Some((route, distance)) = result {
///     println!("Best route: {:?}", route);
///     println!("Distance: {}", distance);
/// }
/// ```
pub fn ant_colony_optimization(
    cities: Vec<(f64, f64)>,
    num_ants: usize,
    num_iterations: usize,
    evaporation_rate: f64,
    alpha: f64,
    beta: f64,
    q: f64,
) -> Option<(Vec<usize>, f64)> {
    if cities.is_empty() {
        return None;
    }

    let city_structs: Vec<City> = cities.into_iter().map(|(x, y)| City { x, y }).collect();

    let mut aco = AntColonyOptimization::new(
        city_structs,
        num_ants,
        num_iterations,
        evaporation_rate,
        alpha,
        beta,
        q,
    );

    aco.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_city_distance() {
        let city1 = City { x: 0.0, y: 0.0 };
        let city2 = City { x: 3.0, y: 4.0 };
        assert!((city1.distance_to(&city2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_city_distance_negative() {
        let city1 = City { x: 0.0, y: 0.0 };
        let city2 = City { x: -3.0, y: -4.0 };
        assert!((city1.distance_to(&city2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_aco_simple() {
        let cities = vec![(0.0, 0.0), (2.0, 2.0)];

        let result = ant_colony_optimization(cities, 5, 5, 0.7, 1.0, 5.0, 10.0);

        assert!(result.is_some());
        let (route, distance) = result.unwrap();

        // Expected route: [0, 1, 0]
        assert_eq!(route, vec![0, 1, 0]);

        // Expected distance: 2 * sqrt(8) ≈ 5.656854
        let expected_distance = 2.0 * (8.0_f64).sqrt();
        assert!((distance - expected_distance).abs() < 0.001);
    }

    #[test]
    fn test_aco_larger_problem() {
        let cities = vec![
            (0.0, 0.0),
            (0.0, 5.0),
            (3.0, 8.0),
            (8.0, 10.0),
            (12.0, 8.0),
            (12.0, 4.0),
            (8.0, 0.0),
            (6.0, 2.0),
        ];

        let result = ant_colony_optimization(cities.clone(), 10, 20, 0.7, 1.0, 5.0, 10.0);

        assert!(result.is_some());
        let (route, distance) = result.unwrap();

        // Verify the route visits all cities
        assert_eq!(route.len(), cities.len() + 1);
        assert_eq!(route.first(), Some(&0));
        assert_eq!(route.last(), Some(&0));

        // Verify all cities are visited exactly once (except start/end)
        let mut visited = std::collections::HashSet::new();
        for &city in &route[0..route.len() - 1] {
            assert!(visited.insert(city), "City {city} visited multiple times");
        }
        assert_eq!(visited.len(), cities.len());

        // Distance should be reasonable (not infinity)
        assert!(distance > 0.0);
        assert!(distance < f64::INFINITY);
    }

    #[test]
    fn test_aco_empty_cities() {
        let cities: Vec<(f64, f64)> = Vec::new();
        let result = ant_colony_optimization(cities, 10, 20, 0.7, 1.0, 5.0, 10.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_aco_single_city() {
        let cities = vec![(0.0, 0.0)];
        let result = ant_colony_optimization(cities, 10, 20, 0.7, 1.0, 5.0, 10.0);

        assert!(result.is_some());
        let (route, distance) = result.unwrap();
        assert_eq!(route, vec![0, 0]);
        assert!((distance - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_default_parameters() {
        let cities = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let result = ant_colony_optimization(cities, 10, 20, 0.7, 1.0, 5.0, 10.0);
        assert!(result.is_some());
    }

    #[test]
    fn test_zero_ants() {
        // Test with zero ants - should return None as no solutions are constructed
        let cities = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let result = ant_colony_optimization(cities, 0, 20, 0.7, 1.0, 5.0, 10.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_zero_iterations() {
        // Test with zero iterations - should return None as no solutions are found
        let cities = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)];
        let result = ant_colony_optimization(cities, 10, 0, 0.7, 1.0, 5.0, 10.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_extreme_parameters() {
        // Test with extreme beta value and many iterations to potentially trigger
        // the rounding fallback in select_next_city
        let cities = vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0), (4.0, 0.0)];
        // Very high beta makes distance dominate, low alpha reduces pheromone influence
        // This creates extreme probability distributions that may trigger rounding edge cases
        let result = ant_colony_optimization(cities, 50, 100, 0.5, 0.1, 100.0, 10.0);
        assert!(result.is_some());
        let (route, _) = result.unwrap();
        // Should still produce valid route
        assert_eq!(route.len(), 6); // 5 cities + return to start
    }
}
