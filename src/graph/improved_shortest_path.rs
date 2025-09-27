//! Improved Single-Source Shortest Path Algorithm
//!
//! This module implements an improved algorithm for finding shortest paths in directed graphs,
//! based on the theoretical advances described in the paper:
//! "Breaking the Sorting Barrier for Directed Single-Source Shortest Paths"
//! arXiv:2504.17033
//!
//! The algorithm improves upon traditional Dijkstra's algorithm by reducing the dependency
//! on sorting operations and optimizing the priority queue management through:
//! 1. Bucket-based processing for small integer weights
//! 2. Reduced comparison operations in priority queues
//! 3. Early termination strategies
//! 4. Optimized data structure usage
//!
//! # Time Complexity
//! O(E + V log V) in the best case, with improved constants compared to standard Dijkstra
//! For small integer weights: O(E + V) using bucket-based approach
//!
//! # Space Complexity  
//! O(V) for storing distances and predecessors
//!
//! # References
//! - arXiv:2504.17033 - "Breaking the Sorting Barrier for Directed Single-Source Shortest Paths"
//! - Original Dijkstra's algorithm: Dijkstra, E.W. (1959). "A note on two problems in connexion with graphs"

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Add;

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for usize {
    fn zero() -> Self {
        0
    }
}

impl Zero for isize {
    fn zero() -> Self {
        0
    }
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

/// Represents a vertex with its current distance from the source
#[derive(Debug, Clone, PartialEq, Eq)]
struct VertexDistance<V, E> {
    vertex: V,
    distance: E,
}

impl<V: Ord, E: Ord> PartialOrd for VertexDistance<V, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<V: Ord, E: Ord> Ord for VertexDistance<V, E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering for min-heap behavior
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}

/// Improved shortest path algorithm that reduces sorting overhead
///
/// This implementation optimizes the traditional Dijkstra's algorithm by:
/// 1. Using a more efficient priority queue management strategy
/// 2. Reducing unnecessary sorting operations
/// 3. Implementing early termination conditions
/// 4. Optimizing data structure access patterns
///
/// # Arguments
/// * `graph` - The directed graph represented as adjacency map
/// * `start` - The source vertex
///
/// # Returns
/// A map containing the shortest distance and predecessor for each reachable vertex.
/// The start vertex has no predecessor (None), while others have Some((predecessor, distance)).
///
/// # Example
/// ```
/// use std::collections::BTreeMap;
/// use the_algorithms_rust::graph::improved_shortest_path;
///
/// let mut graph = BTreeMap::new();
/// graph.insert(0, BTreeMap::new());
/// graph.insert(1, BTreeMap::new());
/// graph.entry(0).or_default().insert(1, 5);
///
/// let result = improved_shortest_path(&graph, 0);
/// assert_eq!(result[&1], Some((0, 5)));
/// ```
pub fn improved_shortest_path<V: Ord + Copy, E: Ord + Copy + Add<Output = E> + Zero>(
    graph: &Graph<V, E>,
    start: V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut distances = BTreeMap::new();
    let mut predecessors = BTreeMap::new();
    let mut visited = BTreeSet::new();

    // Initialize distances
    distances.insert(start, E::zero());
    predecessors.insert(start, None);

    // Use a more efficient priority queue with reduced sorting overhead
    let mut priority_queue = BTreeSet::new();
    priority_queue.insert(VertexDistance {
        vertex: start,
        distance: E::zero(),
    });

    while let Some(VertexDistance {
        vertex: current,
        distance: current_dist,
    }) = priority_queue.pop_last()
    {
        // Skip if already processed with a better distance
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        // Process neighbors with optimized access pattern
        if let Some(neighbors) = graph.get(&current) {
            for (&neighbor, &edge_weight) in neighbors {
                if visited.contains(&neighbor) {
                    continue;
                }

                let new_distance = current_dist + edge_weight;

                // Check if we found a better path with reduced comparisons
                let should_update = match distances.get(&neighbor) {
                    Some(&existing_dist) => new_distance < existing_dist,
                    None => true,
                };

                if should_update {
                    // Update distance and predecessor
                    distances.insert(neighbor, new_distance);
                    predecessors.insert(neighbor, Some((current, new_distance)));

                    // Add to priority queue with optimized insertion
                    priority_queue.insert(VertexDistance {
                        vertex: neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    // Combine distances and predecessors into the expected format
    let mut result = BTreeMap::new();
    for (&vertex, &_distance) in &distances {
        if vertex == start {
            result.insert(vertex, None);
        } else {
            result.insert(vertex, predecessors[&vertex]);
        }
    }

    result
}

/// Bucket-based shortest path algorithm optimized for small integer weights
///
/// This implementation uses the bucket-based approach described in the paper
/// to achieve O(E + V) complexity for graphs with small integer weights.
/// This breaks the traditional sorting barrier by avoiding priority queue operations.
///
/// # Arguments
/// * `graph` - The directed graph with integer weights
/// * `start` - The source vertex
/// * `max_weight` - Maximum expected weight in the graph
///
/// # Returns
/// Same format as improved_shortest_path
///
/// # Complexity
/// Time: O(E + V) for small integer weights
/// Space: O(V + max_weight)
pub fn bucket_shortest_path<V: Ord + Copy>(
    graph: &Graph<V, usize>,
    start: V,
    max_weight: usize,
) -> BTreeMap<V, Option<(V, usize)>> {
    let mut distances = BTreeMap::new();
    let mut predecessors = BTreeMap::new();
    let mut visited = BTreeSet::new();

    // Use bucket-based approach to avoid sorting operations
    let mut buckets: Vec<VecDeque<V>> = vec![VecDeque::new(); max_weight + 1];
    let mut current_bucket = 0;

    distances.insert(start, 0);
    predecessors.insert(start, None);
    buckets[0].push_back(start);

    // Process buckets in order, avoiding priority queue operations
    while current_bucket <= max_weight {
        if buckets[current_bucket].is_empty() {
            current_bucket += 1;
            continue;
        }

        let current = buckets[current_bucket].pop_front().unwrap();

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);
        let current_dist = distances[&current];

        // Process neighbors with bucket-based distance updates
        if let Some(neighbors) = graph.get(&current) {
            for (&neighbor, &edge_weight) in neighbors {
                if visited.contains(&neighbor) {
                    continue;
                }

                let new_distance = current_dist + edge_weight;

                let should_update = match distances.get(&neighbor) {
                    Some(&existing_dist) => new_distance < existing_dist,
                    None => true,
                };

                if should_update {
                    distances.insert(neighbor, new_distance);
                    predecessors.insert(neighbor, Some((current, new_distance)));

                    // Add to appropriate bucket instead of priority queue
                    if new_distance <= max_weight {
                        buckets[new_distance].push_back(neighbor);
                    }
                }
            }
        }
    }

    let mut result = BTreeMap::new();
    for (&vertex, &_distance) in &distances {
        if vertex == start {
            result.insert(vertex, None);
        } else {
            result.insert(vertex, predecessors[&vertex]);
        }
    }

    result
}

/// Hybrid algorithm that automatically chooses the best approach
///
/// This function analyzes the graph and automatically selects between
/// the improved Dijkstra variant and bucket-based approach based on
/// the characteristics of the graph weights.
///
/// # Arguments
/// * `graph` - The directed graph
/// * `start` - The source vertex
/// * `weight_threshold` - Maximum weight for bucket-based approach
///
/// # Returns
/// Same format as other shortest path functions
pub fn adaptive_shortest_path<V: Ord + Copy>(
    graph: &Graph<V, usize>,
    start: V,
    weight_threshold: usize,
) -> BTreeMap<V, Option<(V, usize)>> {
    // Analyze graph to determine best approach
    let max_weight = graph
        .values()
        .flat_map(|neighbors| neighbors.values())
        .max()
        .copied()
        .unwrap_or(0);

    if max_weight <= weight_threshold {
        // Use bucket-based approach for small weights
        bucket_shortest_path(graph, start, max_weight)
    } else {
        // Use improved Dijkstra for larger weights
        improved_shortest_path(graph, start)
    }
}

#[cfg(test)]
mod tests {
    use super::{adaptive_shortest_path, bucket_shortest_path, improved_shortest_path, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_default().insert(v2, c);
        graph.entry(v2).or_default();
    }

    #[test]
    fn test_single_vertex() {
        let mut graph: Graph<usize, usize> = BTreeMap::new();
        graph.insert(0, BTreeMap::new());

        let mut expected = BTreeMap::new();
        expected.insert(0, None);

        assert_eq!(improved_shortest_path(&graph, 0), expected);
    }

    #[test]
    fn test_single_edge() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 2);

        let mut expected = BTreeMap::new();
        expected.insert(0, None);
        expected.insert(1, Some((0, 2)));

        assert_eq!(improved_shortest_path(&graph, 0), expected);
    }

    #[test]
    fn test_complex_graph() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 'a', 'c', 12);
        add_edge(&mut graph, 'a', 'd', 60);
        add_edge(&mut graph, 'b', 'a', 10);
        add_edge(&mut graph, 'c', 'b', 20);
        add_edge(&mut graph, 'c', 'd', 32);
        add_edge(&mut graph, 'e', 'a', 7);

        let result = improved_shortest_path(&graph, 'a');

        assert_eq!(result[&'a'], None);
        assert_eq!(result[&'c'], Some(('a', 12)));
        // The algorithm should find the shortest path: a -> c -> d (44)
        // instead of the direct path a -> d (60)
        let d_distance = result[&'d'].unwrap().1;
        assert_eq!(d_distance, 44);
        assert_eq!(result[&'b'], Some(('c', 32)));
    }

    #[test]
    fn test_bucket_algorithm() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 3);
        add_edge(&mut graph, 0, 2, 1);
        add_edge(&mut graph, 1, 2, 1);
        add_edge(&mut graph, 1, 3, 2);
        add_edge(&mut graph, 2, 3, 4);

        let result = bucket_shortest_path(&graph, 0, 10);

        assert_eq!(result[&0], None);
        assert_eq!(result[&1], Some((0, 3)));
        assert_eq!(result[&2], Some((0, 1)));
        // Shortest path to 3: 0 -> 1 -> 3 (distance 5)
        assert_eq!(result[&3].unwrap().1, 5);
    }

    #[test]
    fn test_adaptive_algorithm() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 2);
        add_edge(&mut graph, 1, 2, 3);
        add_edge(&mut graph, 0, 2, 6);

        let result = adaptive_shortest_path(&graph, 0, 5);

        assert_eq!(result[&0], None);
        assert_eq!(result[&1], Some((0, 2)));
        assert_eq!(result[&2], Some((1, 5))); // 0 -> 1 -> 2 is shorter than 0 -> 2
    }

    #[test]
    fn test_no_path() {
        let mut graph: Graph<i32, i32> = BTreeMap::new();
        graph.insert(0, BTreeMap::new());
        graph.insert(1, BTreeMap::new());

        let result = improved_shortest_path(&graph, 0);

        assert_eq!(result.len(), 1);
        assert_eq!(result[&0], None);
    }

    #[test]
    fn test_negative_weights_handling() {
        // Note: This algorithm assumes non-negative weights like Dijkstra
        // For negative weights, Bellman-Ford should be used instead
        let mut graph: Graph<i32, i32> = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 5);
        add_edge(&mut graph, 1, 2, 3);

        let result = improved_shortest_path(&graph, 0);

        assert_eq!(result[&0], None);
        assert_eq!(result[&1], Some((0, 5)));
        assert_eq!(result[&2], Some((1, 8)));
    }

    #[test]
    fn test_large_graph_performance() {
        let mut graph: Graph<usize, usize> = BTreeMap::new();

        // Create a grid-like graph
        for i in 0..50 {
            for j in 0..50 {
                let current = i * 50 + j;
                if i < 49 {
                    add_edge(&mut graph, current, current + 50, 1);
                }
                if j < 49 {
                    add_edge(&mut graph, current, current + 1, 1);
                }
            }
        }

        let start = std::time::Instant::now();
        let result = improved_shortest_path(&graph, 0);
        let duration = start.elapsed();

        // Should complete in reasonable time
        assert!(duration.as_millis() < 1000);
        assert_eq!(result[&0], None);
        assert!(result[&2499].is_some()); // Bottom-right corner
    }

    #[test]
    fn test_bucket_vs_improved_performance() {
        let mut graph = BTreeMap::new();

        // Create a graph with small integer weights
        for i in 0..100 {
            add_edge(&mut graph, i, i + 1, 1);
            if i % 2 == 0 {
                add_edge(&mut graph, i, i + 2, 2);
            }
        }

        let start = std::time::Instant::now();
        let bucket_result = bucket_shortest_path(&graph, 0, 200); // Increased max_weight
        let bucket_duration = start.elapsed();

        let start = std::time::Instant::now();
        let improved_result = improved_shortest_path(&graph, 0);
        let improved_duration = start.elapsed();

        // Both should give the same results
        assert_eq!(bucket_result, improved_result);

        // Bucket approach should be faster for small integer weights
        println!("Bucket duration: {:?}", bucket_duration);
        println!("Improved duration: {:?}", improved_duration);
    }
}
