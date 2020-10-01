/* 
 * A simple Breadth First Search (BSF)
 * implementation in Rust
 */

use std::collections::VecDeque;

pub fn bfs(from: u32, to: u32, v: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut frontier:   VecDeque<u32>   = VecDeque::new();
    let mut path:       Vec<u32>        = Vec::new();
    let mut visited:    Vec<u32>        = Vec::new();

    visited.resize(v.len(), 0xffff);

    frontier.push_front(from);
    visited[from as usize] = from;

    /* Construct field for tracer */
    while !frontier.is_empty() {
        let p = frontier.pop_front();

        // stop expanding if reached target point
        if p.unwrap() == to {
            break;
        }

        let nbrs = &v[p.unwrap() as usize];

        for n in nbrs {
            if visited[*n as usize] == 0xffff {
                visited[*n as usize] = p.unwrap();
                frontier.push_back(*n);
            }
        }
    }

    /* Follow the White rabbit */
    let mut p = to;

    path.push(p);

    while p != from {
        p = visited[p as usize];
        path.push(p);
    }

    path.reverse();

    return path;
}

pub fn gen_field_graph(n: u32) -> Vec<Vec<u32>> {
    let mut v: Vec<Vec<u32>> = Vec::new();

    for y in 0..n {
        for x in 0..n {
            let mut row: Vec<u32> = Vec::new();
            let pos = x + y * n;

            if pos % n > 0 {
                row.push(pos - 1);  // west
            }

            if pos < n * (n - 1) {
                row.push(pos + n);  // south
            }

            if pos % n < (n - 1) {
                row.push(pos + 1);  // east
            }

            if pos >= n {
                row.push(pos - n);  // north
            }

            v.push(row);
        }
    }

    return v;
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_gen_field_graph_00() {
        let n = 4;

        let eth = vec![
            vec![     4,  1,   ],
            vec![ 0,  5,  2    ],
            vec![ 1,  6,  3    ],
            vec![ 2,  7        ],
            vec![     8,  5,  0],
            vec![ 4,  9,  6,  1],
            vec![ 5, 10,  7,  2],
            vec![ 6, 11,      3],
            vec![    12,  9,  4],
            vec![ 8, 13, 10,  5],
            vec![ 9, 14, 11,  6],
            vec![10, 15,      7],
            vec![        13,  8],
            vec![12,     14,  9],
            vec![13,     15, 10],
            vec![14,         11]
        ];

        let res = gen_field_graph(n);

        assert_eq!(eth.len() == res.len(), false);

        for i in 0..eth.len() {
            for j in 0..eth[i].len() {
                assert_eq!((eth[i])[j], (res[i])[j]);
            }
        }
    }
}


fn main() {

    let start_point = 10;
    let end_point   =  3;

    let g       = gen_field_graph(4);
    let path    = bfs(start_point, end_point, &g);

    //-------
    println!("{:?} ", path);
}