use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

use coloring::Graph;

/// Structure to represent the program input: either file or stdin.
pub enum GraphInput {
    File(String),
    StdIn,
}

/// Reads the problem from the specified input.
///
/// Two integers should be on the first line: `n`, the number of nodes
/// of the graph, and `e`, its number of edges.
/// The following `e` lines each contain two integers: `a_i` and `b_i`
/// and describe an edge between nodes `a_i` and `b_i`.
pub fn read_graph(graph_input: &GraphInput) -> io::Result<Graph> {
    let mut buf_reader = match *graph_input {
        GraphInput::File(ref path) => Box::new(BufReader::new(File::open(path)?)),
        GraphInput::StdIn => Box::new(BufReader::new(io::stdin())) as Box<dyn BufRead>,
    };
    let mut input = String::new();
    buf_reader.read_line(&mut input)?;
    let line = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("parse error"))
        .collect::<Vec<usize>>();
    let n = line[0];
    let e = line[1];
    let mut graph = Graph::new(n);
    for _ in 0..e {
        input.clear();
        buf_reader.read_line(&mut input)?;
        let line = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().expect("parse error"))
            .collect::<Vec<usize>>();
        graph.add_edge(line[0], line[1]);
    }
    Ok(graph)
}

/// Writes a given coloring in a file or on stdin, depending upon the
/// defined input.
pub fn write_coloring(colors: &[usize], graph_input: &GraphInput) -> io::Result<()> {
    let k = colors.iter().max().unwrap() + 1;
    match *graph_input {
        GraphInput::File(ref path) => {
            let mut buf_writer = BufWriter::new(File::create(format!("{}.{}.sol", path, k))?);
            buf_writer.write_all(format!("{} {}\n", k, 0).as_bytes())?;
            for color in colors {
                buf_writer.write_all(format!("{} ", color).as_bytes())?;
            }
            buf_writer.write_all(b"\n")?;
        }
        GraphInput::StdIn => {
            println!("{} {}", k, 0);
            for color in colors {
                print!("{} ", color);
            }
            println!();
        }
    }
    Ok(())
}

/// Removes a previous solution file.
pub fn remove_file(k: usize, graph_input: &GraphInput) -> io::Result<()> {
    match *graph_input {
        GraphInput::File(ref path) => fs::remove_file(format!("{}.{}.sol", path, k)),
        GraphInput::StdIn => Ok(()),
    }
}

fn main() {
    let graph_input = match env::args().nth(1) {
        Some(path) => GraphInput::File(path),
        None => GraphInput::StdIn,
    };
    let graph = read_graph(&graph_input).unwrap();
    // for k in (0..graph.nodes.len()+1).rev() {
    for k in (0..15).rev() {
        println!();
        println!("### Searching for {}-coloring…", k);
        loop {
            match graph.solve(k, 0.5, 1_000_000) {
                Some(colors) => {
                    if graph.check(&colors) {
                        println!("{}-coloring found.", k);
                        write_coloring(&colors, &graph_input).expect("io error");
                        remove_file(k + 1, &graph_input).expect("io error");
                        break;
                    } else {
                        unreachable!()
                    }
                }
                None => {
                    println!("No {}-coloring found, retrying.", k);
                }
            }
        }
    }
}