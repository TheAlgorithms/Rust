use super::strongly_connected_components::StronglyConnectedComponents as SCCs;

pub type Condition = (i64, i64);
type Graph = Vec<Vec<usize>>;

#[inline]
fn variable(var: i64) -> usize {
    if var < 0 {
        (((-var) << 1) + 1) as usize
    } else {
        (var << 1) as usize
    }
}

/// Returns an assignment that satisfies all the constraints, or a variable that makes such an assignment impossible.\
/// Variables should be numbered from 1 to `n`, and a negative number `-m` corresponds to the negated variable `m`.\
/// For more information about this problem, please visit: <https://en.wikipedia.org/wiki/2-satisfiability>
pub fn solve_two_satisfiability(
    expression: &[Condition],
    num_variables: usize,
) -> Result<Vec<bool>, i64> {
    let num_verts = (num_variables + 1) << 1;
    let mut result = Vec::new();
    let mut sccs = SCCs::new(num_verts);
    let mut adj = Graph::new();
    adj.resize(num_verts, vec![]);
    expression.iter().for_each(|cond| {
        let v1 = variable(cond.0);
        let v2 = variable(cond.1);
        adj[v1 ^ 1].push(v2);
        adj[v2 ^ 1].push(v1);
    });
    sccs.find_components(&adj);
    result.resize(num_variables + 1, false);
    for var in (2..num_verts).step_by(2) {
        if sccs.component[var] == sccs.component[var ^ 1] {
            return Err((var >> 1) as i64);
        }
        // if a variable isn't
        if sccs.component[var] < sccs.component[var ^ 1] {
            result[var >> 1] = true;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    fn check_answer(expression: &[Condition], answers: &[bool]) -> bool {
        let mut ok = true;
        for &(c1, c2) in expression {
            let mut cv = false;
            if c1 < 0 {
                cv |= !answers[-c1 as usize];
            } else {
                cv |= answers[c1 as usize];
            }
            if c2 < 0 {
                cv |= !answers[-c2 as usize];
            } else {
                cv |= answers[c2 as usize];
            }
            ok &= cv;
        }
        ok
    }
    #[test]
    fn basic_test() {
        let conds = vec![(1, 1), (2, 2)];
        let res = solve_two_satisfiability(&conds, 2);
        assert!(res.is_ok());
        assert!(check_answer(&conds, &res.unwrap()));

        let conds = vec![(1, 2), (-2, -2)];
        let res = solve_two_satisfiability(&conds, 2);
        assert!(res.is_ok());
        assert!(check_answer(&conds, &res.unwrap()));

        let conds = vec![];
        let res = solve_two_satisfiability(&conds, 2);
        assert!(res.is_ok());
        assert!(check_answer(&conds, &res.unwrap()));

        let conds = vec![(-1, -1), (-2, -2), (1, 2)];
        let res = solve_two_satisfiability(&conds, 2);
        assert!(res.is_err());
    }

    #[test]
    #[ignore]
    fn big_test() {
        // We should spawn a new thread and set its stack size to something
        // big (256MB in this case), because doing DFS (for finding SCCs) is
        // a stack-intensive operation. 256MB should be enough for 3e5
        // variables though.
        let builder = thread::Builder::new().stack_size(256 * 1024 * 1024);
        let handler = builder
            .spawn(|| {
                let num_conds = 3e5 as i64;
                let mut conds = vec![];
                for i in 1..num_conds {
                    conds.push((i, -(i + 1)));
                }
                conds.push((num_conds, num_conds));
                let res = solve_two_satisfiability(&conds, num_conds as usize);
                assert!(res.is_ok());
                assert!(check_answer(&conds, &res.unwrap()));
            })
            .unwrap();
        handler.join().unwrap();
    }
}
