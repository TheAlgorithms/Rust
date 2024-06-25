//graph-colouring uses backtracking algorithm
//it colors the graph in such a way that no two adjacent vertices are of same color
//if two adjacent vertices are found at same color it aims to solve that using backtracking

use std::fmt::Write;
// Checks whether no two adjacent vertices are of the same color
fn is_safe(g: &[Vec<i32>], color: &[i32], v: usize, c: i32) -> bool {
    for (i, &val) in g[v].iter().enumerate() {
        if val == 1 && color[i] == c {
            return false;
        }
    }
    true
}

// Displays the color assigned to each vertex
fn display(color: &[i32], solution_number: &mut i32, output: &mut String) {
    writeln!(output, "Solution {solution_number}:").expect("Failed to write to output");

    println!("{output}");
    *solution_number += 1;
    for (i, &c) in color.iter().enumerate() {
        writeln!(output, "Vertex {} -> Color {}", i + 1, c).expect("Failed to write to output");

        println!("{output}");
    }
    output.push('\n');
}

// Solves the graph coloring problem using backtracking
pub fn graph_coloring(
    g: &[Vec<i32>],
    color: &mut [i32],
    v: usize,
    m: i32,
    solution_number: &mut i32,
    output: &mut String,
) -> bool {
    if v == g.len() {
        display(color, solution_number, output);
        return true;
    }
    let mut res = false;
    for i in 1..=m {
        if is_safe(g, color, v, i) {
            color[v] = i;
            res = graph_coloring(g, color, v + 1, m, solution_number, output) || res;
            color[v] = 0;
        }
    }
    res
}

// Test function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_coloring_check_3() {
        let g = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 1],
            vec![1, 0, 1, 0],
        ];
        let mut color = vec![0; 4];
        let mut solution_number = 1;
        let mut output = String::new();
        let expected_output = "Solution 1:
Vertex 1 -> Color 1
Vertex 2 -> Color 2
Vertex 3 -> Color 3
Vertex 4 -> Color 2

Solution 2:
Vertex 1 -> Color 1
Vertex 2 -> Color 3
Vertex 3 -> Color 2
Vertex 4 -> Color 3

Solution 3:
Vertex 1 -> Color 2
Vertex 2 -> Color 1
Vertex 3 -> Color 3
Vertex 4 -> Color 1

Solution 4:
Vertex 1 -> Color 2
Vertex 2 -> Color 3
Vertex 3 -> Color 1
Vertex 4 -> Color 3

Solution 5:
Vertex 1 -> Color 3
Vertex 2 -> Color 1
Vertex 3 -> Color 2
Vertex 4 -> Color 1

Solution 6:
Vertex 1 -> Color 3
Vertex 2 -> Color 2
Vertex 3 -> Color 1
Vertex 4 -> Color 2

";

        graph_coloring(&g, &mut color, 0, 3, &mut solution_number, &mut output);
        assert_eq!(expected_output, output);
    }
    #[test]
    fn graph_coloring_check_2() {
        let g = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0],
            vec![1, 1, 0, 1],
            vec![1, 0, 1, 0],
        ];
        let mut color = vec![0; 4];
        let mut solution_number = 1;
        let mut output = String::new();
        let expected_output = "";

        graph_coloring(&g, &mut color, 0, 2, &mut solution_number, &mut output);
        assert_eq!(expected_output, output);
    }
}
