use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::vec_deque::VecDeque;
use std::ops::AddAssign;
type Vertex = u64;
type Graph = HashMap<Vertex, HashSet<Vertex>>;
pub fn from_pairs(pairs: &[(u64, u64)]) -> Graph {
    let mut graph: Graph = HashMap::new();
    for &(parent, child) in pairs {
        let children = graph.entry(parent).or_insert(HashSet::new());
        children.insert(child);
    }
    return graph;
}
pub struct DFSResult {
    starting_times: HashMap<Vertex, u64>,
    finishing_times: HashMap<Vertex, u64>,
    parents: HashMap<Vertex, Option<Vertex>>,
    forest: VecDeque<HashSet<Vertex>>,
    topologically_sorted: VecDeque<Vertex>,
}
pub fn dfs(graph: &Graph) -> DFSResult {
    let mut state = DFSResult {
        forest: VecDeque::new(),
        starting_times: HashMap::new(),
        finishing_times: HashMap::new(),
        parents: HashMap::new(),
        topologically_sorted: VecDeque::new(),
    };
    let mut seen: HashSet<Vertex> = HashSet::new();
    let mut time = 0;
    fn dfs_visit (graph: &Graph,
                  vertex: Vertex,
                  time: &mut u64,
                  seen: &mut HashSet<Vertex>,
                  state: &mut DFSResult) {
        time.add_assign(1);
        state.starting_times.insert(vertex, *time);
        seen.insert(vertex);
        let mut branch = state.forest.pop_back().expect("The Forest should not be empty!");
        branch.insert(vertex);
        state.forest.push_back(branch);
        for neighbor in graph.get(&vertex).unwrap_or(&HashSet::new()) {
            if !seen.contains(neighbor) {
                state.parents.insert(*neighbor, Option::Some(vertex));
                dfs_visit(graph, *neighbor, time, seen, state);
            }
        }
        time.add_assign(1);
        state.finishing_times.insert(vertex, *time);
        state.topologically_sorted.push_front(vertex);
    };
    for vertex in graph.keys() {
        state.parents.insert(*vertex, Option::None);
    }
    for vertex in graph.keys() {
        if !seen.contains(vertex) {
            state.forest.push_back(HashSet::new());
            dfs_visit(graph, *vertex, &mut time, &mut seen, &mut state);
        }
    }
    return state;
}
fn topological_sort(graph: &Graph) -> VecDeque<Vertex> {
    let DFSResult{topologically_sorted, ..} = dfs(graph);
    return topologically_sorted
}
fn main() {
    let pairs = [(1, 2), (2, 1)];
    let g = from_pairs(&pairs);
    println!("{:?}", g);
    println!("{:?}", dfs(&g));
    println!("{:?}", topological_sort(&g));
}
