type NestedVec = Vec<Vec<usize>>;

pub fn ullmann(graph: &NestedVec, pattern: &NestedVec) -> NestedVec {
    let mut matches: NestedVec = vec![];
    let mut candidates = initial_candidates(graph, pattern);

    update_candidates(graph, pattern, &mut candidates);
    search(graph, pattern, &mut matches, &mut candidates, 0);

    matches
}

fn search(
    graph: &NestedVec,
    pattern: &NestedVec,
    matches: &mut NestedVec,
    candidates: &mut NestedVec,
    depth: usize,
) {
    if depth == pattern.len() {
        // found an isomorphism
        matches.push(candidates.iter().map(|c| c[0]).collect::<Vec<_>>())
    } else {
        for v_g in &candidates[depth] {
            // check if v_G has matched a previous candidate
            if depth == 0
                || candidates[..=depth - 1]
                    .iter()
                    .find(|x| x[0] == *v_g)
                    .is_none()
            {
                let mut new_candidates = candidates.clone();
                new_candidates[depth] = vec![*v_g];
                if update_candidates(&graph, &pattern, &mut new_candidates) {
                    search(&graph, &pattern, matches, &mut new_candidates, depth + 1);
                }
            }
        }
    }
}

fn initial_candidates(graph: &NestedVec, pattern: &NestedVec) -> NestedVec {
    let mut candidates = vec![];
    for u_p in 0..pattern.len() {
        let degree_u_p = pattern[u_p].len();
        let c = (0..graph.len())
            .filter(|u_g| graph[*u_g].len() >= degree_u_p)
            .collect();
        candidates.insert(u_p, c)
    }
    candidates
}

fn update_candidates(graph: &NestedVec, pattern: &NestedVec, candidates: &mut NestedVec) -> bool {
    let mut candidates_update = true;

    while candidates_update {
        candidates_update = false;
        // for each vertex u_P in the pattern
        for u_p in 0..pattern.len() {
            // for each neighbor of u_P (v_P)
            for v_p in &pattern[u_p] {
                let mut u_g_new: Vec<usize> = vec![];
                // for each candidate of u_P (u_G)
                for u_g in &candidates[u_p] {
                    let mut found_matching_edge = false;
                    for v_g in &candidates[*v_p] {
                        if graph[*u_g].binary_search(v_g).is_ok() {
                            found_matching_edge = true;
                            break;
                        }
                    }
                    // if there is a matching edge, keep u_G as candidate for u_P
                    if found_matching_edge {
                        u_g_new.push(*u_g);
                    } else {
                        // if there is no matching edge, u_G needs to be removed
                        candidates_update = true;
                    }
                }
                // if no candidate is left for u_P, there is no match
                if u_g_new.is_empty() {
                    return false;
                }
                candidates[u_p] = u_g_new;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let g = vec![
            // (0)-->(1), (0)-->(2)
            vec![1, 2],
            // (1)-->(3)
            vec![3],
            // (2)-->(3)
            vec![3],
            // (3)-->(4)
            vec![4],
            // (4)
            vec![],
        ];

        let q = vec![
            // (0)-->(1)
            vec![1],
            // (1)-->(2)
            vec![2],
            // (2)-->(3)
            vec![3],
            // (4)
            vec![],
        ];

        let result = self::ullmann(&g, &q);

        assert_eq!(result, vec![vec![0, 1, 3, 4], vec![0, 2, 3, 4]])
    }
}
