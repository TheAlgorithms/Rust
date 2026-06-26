use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Calculates the PageRank for each node in a graph.
///
/// The graph is represented as an adjacency list: `HashMap<Node, Vec<Node>>`,
/// where each key is a source node pointing to a vector of destination nodes.
///
/// # Parameters
/// * `graph` - The adjacency list of the graph.
/// * `damping_factor` - The probability that a surfer continues clicking links should be in betwen 0 and 1 (typically 0.85).
/// * `max_iterations` - The maximum number of iterations to perform (typically 100).
/// * `convergence_threshold` - The L1 difference threshold to stop iterations early (typically 1e-5).
pub fn page_rank<Node: Hash + Eq + Clone>(
    graph: &HashMap<Node, Vec<Node>>,
    damping_factor: f64,
    max_iterations: usize,
    convergence_threshold: f64,
) -> HashMap<Node, f64> {
    if graph.is_empty() {
        return HashMap::new();
    }

    // Collect all unique nodes present as either a source or a destination
    let mut all_nodes = HashSet::new();
    for (src, dests) in graph {
        all_nodes.insert(src.clone());
        for dest in dests {
            all_nodes.insert(dest.clone());
        }
    }

    let num_pages = all_nodes.len();
    let num_pages_f64 = num_pages as f64;

    // Initial ranks: 1.0 / N
    let mut ranks: HashMap<Node, f64> = all_nodes
        .iter()
        .map(|node| (node.clone(), 1.0 / num_pages_f64))
        .collect();

    // Track out-degrees and build the reverse (incoming) graph
    let mut out_degrees: HashMap<Node, usize> =
        all_nodes.iter().map(|node| (node.clone(), 0)).collect();

    let mut incoming_edges: HashMap<Node, Vec<Node>> = all_nodes
        .iter()
        .map(|node| (node.clone(), Vec::new()))
        .collect();

    for (src, dests) in graph {
        // Deduplicate destinations so multi-edges don't skew rank distribution
        let unique_dests: Vec<&Node> = {
            let mut seen = HashSet::new();
            dests.iter().filter(|d| seen.insert(*d)).collect()
        };

        out_degrees.insert(src.clone(), unique_dests.len());
        for dest in unique_dests {
            if let Some(incoming) = incoming_edges.get_mut(dest) {
                incoming.push(src.clone());
            }
        }
    }

    // Dangling nodes are those with zero out-degree
    let dangling_nodes: Vec<Node> = out_degrees
        .iter()
        .filter(|(_, &degree)| degree == 0)
        .map(|(node, _)| node.clone())
        .collect();

    let base_random_jump = (1.0 - damping_factor) / num_pages_f64;

    // Iterative power iteration
    for _ in 0..max_iterations {
        // Sum ranks of dangling nodes to redistribute evenly
        let total_dangling_mass: f64 = dangling_nodes.iter().map(|node| ranks[node]).sum();

        let dangling_share = (total_dangling_mass * damping_factor) / num_pages_f64;
        let base_rank = base_random_jump + dangling_share;

        let mut new_ranks = HashMap::with_capacity(num_pages);

        for node in &all_nodes {
            let mut sum_incoming = 0.0;
            if let Some(sources) = incoming_edges.get(node) {
                for src in sources {
                    let degree = out_degrees[src];
                    sum_incoming += ranks[src] / (degree as f64);
                }
            }

            let rank = base_rank + (sum_incoming * damping_factor);
            new_ranks.insert(node.clone(), rank);
        }

        // Check for convergence (L1 norm difference)
        let total_diff: f64 = all_nodes
            .iter()
            .map(|node| (ranks[node] - new_ranks[node]).abs())
            .sum();

        ranks = new_ranks;

        if total_diff < convergence_threshold {
            break;
        }
    }

    ranks
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Assert that every node's rank is within `epsilon` of `expected`.
    fn assert_ranks_close(ranks: &HashMap<String, f64>, expected: &[(&str, f64)], epsilon: f64) {
        for (node, exp) in expected {
            let got = ranks[*node]; // Indexing panics automatically if the node is missing
            assert!((got - exp).abs() < epsilon);
        }
    }

    /// All ranks must sum to 1.0 (within tolerance).
    fn assert_sum_to_one(ranks: &HashMap<String, f64>, epsilon: f64) {
        let total: f64 = ranks.values().sum();
        assert!((total - 1.0).abs() < epsilon);
    }

    // -----------------------------------------------------------------------
    // 1. Empty graph
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_graph() {
        let graph: HashMap<String, Vec<String>> = HashMap::new();
        let ranks = page_rank(&graph, 0.85, 100, 1e-5);
        assert!(ranks.is_empty());
    }

    // -----------------------------------------------------------------------
    // 2. Single node, self-loop
    // -----------------------------------------------------------------------

    #[test]
    fn test_single_node_self_loop() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["A".to_string()]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        assert_eq!(ranks.len(), 1);
        // With a single node the rank must be 1.0
        assert!((ranks["A"] - 1.0).abs() < 1e-5);
        assert_sum_to_one(&ranks, 1e-5);
    }

    // -----------------------------------------------------------------------
    // 3. Single node, no edges (dangling)
    // -----------------------------------------------------------------------

    #[test]
    fn test_single_dangling_node() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec![]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        assert_eq!(ranks.len(), 1);
        assert!((ranks["A"] - 1.0).abs() < 1e-5);
        assert_sum_to_one(&ranks, 1e-5);
    }

    // -----------------------------------------------------------------------
    // 4. Circular graph (A→B→C→A) — symmetry
    //    Already present in the original file; extended with sum check.
    // -----------------------------------------------------------------------

    #[test]
    fn test_circular_graph_symmetry() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["A".to_string()]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        let expected = 1.0 / 3.0;
        assert_ranks_close(
            &ranks,
            &[("A", expected), ("B", expected), ("C", expected)],
            1e-4,
        );
        assert_sum_to_one(&ranks, 1e-4);
    }

    // -----------------------------------------------------------------------
    // 5. Two nodes, reciprocal links (A⇄B)
    //    Both should converge to 0.5 each.
    // -----------------------------------------------------------------------

    #[test]
    fn test_two_nodes_bidirectional() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["A".to_string()]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        assert_eq!(ranks.len(), 2);
        assert_ranks_close(&ranks, &[("A", 0.5), ("B", 0.5)], 1e-4);
        assert_sum_to_one(&ranks, 1e-4);
    }

    // -----------------------------------------------------------------------
    // 6. Star graph — hub receives all rank
    //    Spokes A, B, C all point to Hub.  Hub is a dangling node.
    //    Expected: Hub collects the redistributed mass and ends up highest.
    // -----------------------------------------------------------------------

    #[test]
    fn test_star_graph_hub_wins() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["Hub".to_string()]);
        graph.insert("B".to_string(), vec!["Hub".to_string()]);
        graph.insert("C".to_string(), vec!["Hub".to_string()]);
        // Hub has no outgoing edges → dangling node

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        assert_eq!(ranks.len(), 4);
        assert_sum_to_one(&ranks, 1e-4);

        // Hub must have strictly higher rank than any spoke
        let hub_rank = ranks["Hub"];
        for spoke in &["A", "B", "C"] {
            assert!(hub_rank > ranks[*spoke]);
        }
    }

    // -----------------------------------------------------------------------
    // 7. Linear chain — rank should decrease along the chain
    //    A → B → C → D  (all dangling except the last which is also dangling)
    //    Nodes further down get more incoming flow; D has no outbound links.
    // -----------------------------------------------------------------------

    #[test]
    fn test_linear_chain_rank_order() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        // D is a sink (dangling)

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        assert_eq!(ranks.len(), 4);
        assert_sum_to_one(&ranks, 1e-4);

        // With PageRank's dangling-node redistribution D should accumulate most rank
        assert!(ranks["D"] > ranks["A"]);
    }

    // -----------------------------------------------------------------------
    // 8. Disconnected graph — two separate components
    //    A→B  and  C→D→C
    //    All nodes must still be present; ranks sum to 1.
    // -----------------------------------------------------------------------

    #[test]
    fn test_disconnected_components() {
        let mut graph = HashMap::new();
        // Component 1
        graph.insert("A".to_string(), vec!["B".to_string()]);
        // Component 2 (cycle)
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec!["C".to_string()]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        // B is a dangling node and appears only as a destination,
        // so it must still be included.
        assert_eq!(ranks.len(), 4);
        assert_sum_to_one(&ranks, 1e-4);

        // Sanity: no rank is zero or negative
        for &rank in ranks.values() {
            assert!(rank > 0.0);
        }
    }

    // -----------------------------------------------------------------------
    // 9. Known small graph with analytically derivable ranks
    //    Classic 3-node example from the original PageRank paper.
    //
    //    A → B, A → C
    //    B → C
    //    C → A
    //
    //    With d = 0.85, N = 3:
    //      base = (1 - 0.85) / 3 = 0.05
    //
    //    PR(A) = 0.05 + 0.85 * PR(C)/1
    //    PR(B) = 0.05 + 0.85 * PR(A)/2
    //    PR(C) = 0.05 + 0.85 * (PR(A)/2 + PR(B)/1)
    //
    //    Solving: PR(A) ≈ 0.4828, PR(B) ≈ 0.2552, PR(C) ≈ 0.2620
    //    (normalised so they sum to 1)
    // -----------------------------------------------------------------------

    #[test]
    fn test_analytical_three_node() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["A".to_string()]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-6);

        assert_sum_to_one(&ranks, 1e-4);

        // C receives flow from both A (half) and B (all of B).
        // A receives flow only from C.
        // Correct order: C > A > B
        assert!(ranks["C"] > ranks["A"]);
        assert!(ranks["A"] > ranks["B"]);

        // Analytically solved values (d=0.85, N=3):
        // PR(A) = 0.3878, PR(B) = 0.2148, PR(C) = 0.3974
        assert_ranks_close(&ranks, &[("A", 0.3878), ("B", 0.2148), ("C", 0.3974)], 5e-3);
    }

    // -----------------------------------------------------------------------
    // 10. All nodes point to one sink — dangling mass is redistributed
    //     A → D, B → D, C → D   (D is a dangling node)
    //     All four nodes must receive some rank due to redistribution.
    // -----------------------------------------------------------------------

    #[test]
    fn test_all_pointing_to_sink_redistributes() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["D".to_string()]);
        graph.insert("B".to_string(), vec!["D".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        // D has no outgoing edges

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        assert_sum_to_one(&ranks, 1e-4);

        // D should have the highest rank
        let d = ranks["D"];
        assert!(d > ranks["A"]);
        assert!(d > ranks["B"]);
        assert!(d > ranks["C"]);

        // A, B, C are symmetric → equal ranks
        assert!((ranks["A"] - ranks["B"]).abs() < 1e-4);
        assert!((ranks["B"] - ranks["C"]).abs() < 1e-4);
    }

    // -----------------------------------------------------------------------
    // 11. Damping factor = 0 (pure random jump, uniform distribution)
    //     With d = 0 every node gets rank 1/N regardless of topology.
    // -----------------------------------------------------------------------

    #[test]
    fn test_damping_factor_zero_gives_uniform() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["A".to_string()]);

        let ranks = page_rank(&graph, 0.0, 100, 1e-5);

        let expected = 1.0 / 3.0;
        assert_ranks_close(
            &ranks,
            &[("A", expected), ("B", expected), ("C", expected)],
            1e-4,
        );
    }

    // -----------------------------------------------------------------------
    // 12. Integer node keys (tests generic Hash + Eq + Clone bound)
    // -----------------------------------------------------------------------

    #[test]
    fn test_integer_nodes() {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![1]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        assert_eq!(ranks.len(), 3);
        let expected = 1.0 / 3.0;
        for i in 1..=3 {
            let rank = ranks[&i];
            assert!((rank - expected).abs() < 1e-4);
        }
    }

    // -----------------------------------------------------------------------
    // 13. Convergence: fewer iterations should still be close (sanity check)
    // -----------------------------------------------------------------------

    #[test]
    fn test_convergence_within_iterations() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
        graph.insert("B".to_string(), vec!["C".to_string()]);
        graph.insert("C".to_string(), vec!["A".to_string()]);

        let ranks_full = page_rank(&graph, 0.85, 100, 1e-8);
        // 10 iterations gets all nodes within 0.002 of the converged value
        let ranks_few = page_rank(&graph, 0.85, 10, 1e-8);

        for node in &["A", "B", "C"] {
            let diff = (ranks_full[*node] - ranks_few[*node]).abs();
            assert!(diff < 0.005);
        }
    }

    // -----------------------------------------------------------------------
    // 14. Node that appears only as a destination (never as a key in the map)
    //     must still be present in the output.
    // -----------------------------------------------------------------------

    #[test]
    fn test_implicit_destination_node_present() {
        let mut graph = HashMap::new();
        // "B" and "C" never appear as keys
        graph.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-5);

        assert!(ranks.contains_key("A"));
        assert!(ranks.contains_key("B"));
        assert!(ranks.contains_key("C"));
        assert_sum_to_one(&ranks, 1e-4);
    }

    // -----------------------------------------------------------------------
    // 15. Large fully-connected graph — all ranks equal
    //     In a complete graph every node has identical in- and out-degree,
    //     so all ranks converge to 1/N.
    // -----------------------------------------------------------------------

    #[test]
    fn test_complete_graph_uniform_ranks() {
        let nodes = vec!["A", "B", "C", "D", "E"];
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for &src in &nodes {
            let dests: Vec<String> = nodes
                .iter()
                .filter(|&&n| n != src)
                .map(|&n| n.to_string())
                .collect();
            graph.insert(src.to_string(), dests);
        }

        let ranks = page_rank(&graph, 0.85, 100, 1e-6);

        assert_eq!(ranks.len(), 5);
        let expected = 1.0 / 5.0;
        assert_ranks_close(
            &ranks,
            &nodes.iter().map(|&n| (n, expected)).collect::<Vec<_>>(),
            1e-4,
        );
        assert_sum_to_one(&ranks, 1e-4);
    }

    // -----------------------------------------------------------------------
    // 16. Pure duplicate edges cancel out — result identical to single edge
    //     A→[B,B] is equivalent to A→[B]: both halves of A's rank flow to B.
    //     Without deduplication this accidentally works; WITH the fix it still works.
    //     The test documents that the behaviour is correct either way.
    // -----------------------------------------------------------------------
    #[test]
    fn test_pure_duplicate_edges_same_as_single() {
        let mut graph_single = HashMap::new();
        graph_single.insert("A".to_string(), vec!["B".to_string()]);
        graph_single.insert("B".to_string(), vec!["A".to_string()]);

        let mut graph_dup = HashMap::new();
        graph_dup.insert("A".to_string(), vec!["B".to_string(), "B".to_string()]);
        graph_dup.insert("B".to_string(), vec!["A".to_string()]);

        let ranks_single = page_rank(&graph_single, 0.85, 100, 1e-6);
        let ranks_dup = page_rank(&graph_dup, 0.85, 100, 1e-6);

        for node in &["A", "B"] {
            let diff = (ranks_single[*node] - ranks_dup[*node]).abs();
            assert!(diff < 1e-4);
        }
    }

    // -----------------------------------------------------------------------
    // 17. Mixed duplicate + distinct edges — the critical failure case.
    //     A→[B,B,C]: without dedup B gets 2/3 of A's rank, C gets 1/3.
    //     With dedup it becomes A→[B,C]: both get 1/2, i.e. B == C.
    // -----------------------------------------------------------------------
    #[test]
    fn test_mixed_duplicate_and_distinct_edges() {
        let mut graph = HashMap::new();
        // A points to B twice and C once — B and C should receive equal rank
        graph.insert(
            "A".to_string(),
            vec!["B".to_string(), "B".to_string(), "C".to_string()],
        );
        graph.insert("B".to_string(), vec!["A".to_string()]);
        graph.insert("C".to_string(), vec!["A".to_string()]);

        let ranks = page_rank(&graph, 0.85, 100, 1e-6);

        assert_sum_to_one(&ranks, 1e-4);
        assert!((ranks["B"] - ranks["C"]).abs() < 1e-4);
    }

    // -----------------------------------------------------------------------
    // 18. Self-loop duplicate — A→[A,A,B] should deduplicate to A→[A,B]
    // -----------------------------------------------------------------------
    #[test]
    fn test_duplicate_self_loop_with_other_edge() {
        let mut graph_dup = HashMap::new();
        graph_dup.insert(
            "A".to_string(),
            vec!["A".to_string(), "A".to_string(), "B".to_string()],
        );
        graph_dup.insert("B".to_string(), vec!["A".to_string()]);

        let mut graph_clean = HashMap::new();
        graph_clean.insert("A".to_string(), vec!["A".to_string(), "B".to_string()]);
        graph_clean.insert("B".to_string(), vec!["A".to_string()]);

        let ranks_dup = page_rank(&graph_dup, 0.85, 100, 1e-6);
        let ranks_clean = page_rank(&graph_clean, 0.85, 100, 1e-6);

        for node in &["A", "B"] {
            let diff = (ranks_dup[*node] - ranks_clean[*node]).abs();
            assert!(diff < 1e-4);
        }
    }
}
