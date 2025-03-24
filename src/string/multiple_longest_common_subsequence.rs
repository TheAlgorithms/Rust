use std::cmp::max;
use std::cmp::Ordering;
use std::collections::HashMap;

// alphabet : the common alphabet
// chains : the strings among which the common subsequence is
// d : the number of strings
// f : for each point, an heuristic function
// g : for each point, the number of ancestors
// ms : the table of suffix tables
// mt : the lookup table
// parents : the ancestor tree
struct Context {
    alphabet: Vec<char>,
    chains: Vec<Vec<char>>,
    d: usize,
    f: HashMap<Vec<Option<usize>>, u64>,
    g: HashMap<Vec<Option<usize>>, u64>,
    ms: Vec<Vec<Vec<u64>>>,
    mt: Vec<Vec<Vec<Option<usize>>>>,
    parents: HashMap<Vec<Option<usize>>, Option<Vec<Option<usize>>>>,
}

impl Context {
    pub fn new(strings: &[&str]) -> Self {
        // cast to ease [index] accessibily
        let chains: Vec<Vec<char>> = strings.iter().map(|s| s.chars().collect()).collect();
        let d = strings.len();

        let mut alphabet: Vec<char> = get_alphabet(&chains);

        let ms: Vec<Vec<Vec<u64>>> = matrices_score(&chains);

        // an impossible to reach point, father of all points
        let p0 = vec![None; d];

        let mut parents: HashMap<_, Option<Vec<Option<usize>>>> = HashMap::new();
        parents.insert(p0.clone(), None);

        let mut g = HashMap::new();
        g.insert(p0.clone(), 0);

        let mut f: HashMap<Vec<Option<usize>>, u64> = HashMap::new();
        f.insert(p0, 0);

        let mt = mt_table(&chains, &mut alphabet);

        Context {
            alphabet,
            chains,
            d,
            f,
            g,
            ms,
            mt,
            parents,
        }
    }

    // given a point p and his successor q, computes necessary informations
    // point p is marked PARENT of q
    pub fn update_suc(&mut self, p: Vec<Option<usize>>, q: Vec<Option<usize>>) {
        // g(q) = g(p) + 1
        let nb = &self.g[&p] + 1;
        self.g.insert(q.clone(), nb);
        // saves the cost function for point p : h(p) + g(p)
        self.f.insert(q.clone(), self.heuristic(&q) + nb);
        // saves the fact that p is the parent of q
        self.parents.insert(q, Some(p));
    }

    /// Finds all succcesors of the point p
    /// A successor of p = (p_1, p_2, etc, p_n) is a point q = (q_1, q_2, etc, q_n)
    /// such that q_1 > p_1, q_2 > p_2, etc, q_n > p_n
    /// [Documentation](https://github.com/epita-rs/MLCS/blob/main/doc/paper.pdf)
    ///
    /// # Arguments
    /// # 'Context' A struct containing informations
    /// # 'p' The point under examination
    ///
    /// # Returns
    /// An array of the successors
    pub fn get_successors(&self, p: &[Option<usize>]) -> Vec<Vec<Option<usize>>> {
        let mut successors: Vec<Vec<Option<usize>>> = vec![];

        // for all alphabet letters
        for (ch_idx, _) in self.alphabet.iter().enumerate() {
            // for each string, finds the next position of that letter
            let mut succ: Vec<Option<usize>> = vec![];
            for (i, p_ith_elt) in p.iter().enumerate().take(self.chains.len()) {

                let next_ch_idx = match p_ith_elt {
                    Some(idx) => self.mt[ch_idx][i][idx + 1],
                    None => continue, // Skip if current position is None
                };
                
                // in case the letter is not reachable in the string
                if next_ch_idx.is_none() {
                    break;
                }

                succ.push(next_ch_idx);
            }

            // the vector is complete, hence we add it to the successors
            if succ.len() == self.chains.len() {
                successors.push(succ);
            }
            // else we discard it and move on to the next letter
        }
        successors
    }

    // ascend back up the parent tree to form the common subsequence
    fn common_seq(&self, p: &Vec<Option<usize>>) -> String {
        let ref_str: &Vec<char> = &self.chains[0];
        let mut common_subsequence: Vec<char> = vec![];
        // Gaining mutability
        let mut p = p;

        while self.parents[p].is_some() {
            // Get the first element of p, which is the position in the first string
            if let Some(idx) = p[0] {
                common_subsequence.push(ref_str[idx]);
            }

            // getting the parent of current point
            p = self.parents[p].as_ref().unwrap();
        }

        common_subsequence.iter().rev().collect::<String>()
    }

    /// CF Initqueue
    fn get_starting_p(&self) -> Vec<Vec<Option<usize>>> {
        let mut successors: Vec<Vec<Option<usize>>> = vec![];

        // for each alphabet letter, finds the next match
        // meaning the a point where all strings share a character
        // example: In ["AB", "BC", "CB", "BF"],
        // A match for the letter B would be p = (1, 0, 1, 0)
        for (ch_idx, _) in self.alphabet.iter().enumerate() {
            // for each string, finds the next position of that letter
            let mut succ: Vec<Option<usize>> = vec![];
            for i in 0..(self.chains.len()) {
                // gets the next position of the current letter
                let next_ch_idx = self.mt[ch_idx][i][0];
                succ.push(next_ch_idx);
            }

            // once the vector is complete, we add it to the successors
            successors.push(succ);
        }

        successors
    }

    /// Computes the heuristic function given a point
    /// min ( { M_ij[ p[i] ][ p[j] ] | (i,j) in [0 ; d] } )
    /// [Documentation](https://github.com/epita-rs/MLCS/blob/main/doc/paper.pdf)
    fn heuristic(&self, p: &[Option<usize>]) -> u64 {
        let mut similarity: Vec<u64> = vec![];
        for i in 0..self.d {
            for j in 0..self.d {
                if i != j {
                    // Skip if either point is None
                    if let (Some(pi), Some(pj)) = (p[i], p[j]) {
                        similarity.push(self.ms[to_linear_index(i, j, self.d)][pi][pj]);
                    }
                }
            }
        }

        similarity.iter().min().copied().unwrap_or(0)
    }

    /// Add the first matches to the queue
    /// For each starting point found, sets an impossible point as parent
    /// [Documentation](https://github.com/epita-rs/MLCS/blob/main/doc/paper.pdf)
    ///
    /// # Arguments
    ///
    /// * `self' - A structure containing informations
    /// * 'queue' - The priority queue of points  
    fn init_queue(&mut self) -> Vec<Vec<Option<usize>>> {
        let mut queue = self.get_starting_p();

        for q in queue.clone() {
            self.update_suc(vec![None; self.d], q.clone());
        }

        self.reorder_queue(&mut queue);

        queue
    }

    // sorts the queue
    fn reorder_queue(&self, queue: &mut [Vec<Option<usize>>]) {
        queue.sort_unstable_by(|p, q| {
            if (self.f.get(p) > self.f.get(q))
                || (self.f.get(p) == self.f.get(q) && self.heuristic(p) > self.heuristic(q))
            {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    }
}

/// Heuristic to find the smallest common alphabet among the strings
/// gets the shortest string and remove duplicates
///
/// # Arguments
/// # 'chains' The strings among wich the mlcs is
///
/// # Returns
/// A vector
fn get_alphabet(chains: &[Vec<char>]) -> Vec<char> {
    let mut alphabet: Vec<char> = chains
        .iter()
        .min_by_key(|s| s.len())
        .expect("No minimum found")
        .to_vec();
    alphabet.sort();
    alphabet.dedup();

    alphabet
}

/// Computes the suffix tables between each pair of string
/// used by the MLCS-Astar heuristic function
/// [Documentation](https://github.com/epita-rs/MLCS/blob/main/doc/paper.pdf)
///
/// # Arguments
///
/// * `chains` - A slice of collected strings
///            - from which the suffix tables are computed.
fn matrices_score(chains: &[Vec<char>]) -> Vec<Vec<Vec<u64>>> {
    let mut scores: Vec<Vec<Vec<u64>>> = vec![];
    for s1 in chains.iter() {
        for s2 in chains.iter() {
            scores.push(score_matrix(s1, s2));
        }
    }

    scores
}

/// Builds the lookup table used for accessing the index of the next char
/// updates the alphabet to be the alphabet of the letters common to all strings
///
/// # Arguments
/// # 'chains' the strings as a matrix of char
/// # 'alphabet' the letters in the strings
///
/// # Returns
/// An array of matrices.
/// Each matrix is tied to a string and can indicate, given a letter,
/// the next position of that letter in the string.
fn mt_table(chains: &Vec<Vec<char>>, alphabet: &mut Vec<char>) -> Vec<Vec<Vec<Option<usize>>>> {
    let mut mt: Vec<Vec<Vec<Option<usize>>>> = vec![];

    for ch in alphabet.clone() {
        let mut chain: Vec<Vec<Option<usize>>> = vec![];

        for s in chains {
            let mut v: Vec<Option<usize>> = vec![None; s.len()];
            let mut lpos = None;

            // iterating backwards on the string
            for i in (0..(s.len())).rev() {
                if s[i] == ch {
                    lpos = Some(i);
                }
                // pushing the index of the last encounter with the current letter
                v[i] = lpos;
            }

            chain.push(v);

            // if the letter was never seen in the current string
            // then it can't part of the common alphabet
            if lpos.is_none() {
                // removing that letter
                alphabet.retain(|&x| x != ch);
                chain = vec![];
                break;
            }
        }

        // the letter was seen at leat once
        if !chain.is_empty() {
            mt.push(chain);
        }
    }

    mt
}

/// Finds one of the longest_common_subsequence among multiple strings
/// using a similar approach to the A* algorithm in graph theory
/// [Documentation](https://github.com/epita-rs/MLCS/blob/main/doc/paper.pdf)
/// # Arguments
///
/// * `S` - Array of strings.
///
/// # Returns
///
/// * `String` if a Longest Common Subsequence exists
/// * `String' if no LCS was found
pub fn multiple_longest_common_subsequence(chains: &Vec<&str>) -> String {
    const C: u64 = 20;

    // Preprocessing
    let mut ctx = Context::new(chains);

    // queue
    let mut queue: Vec<Vec<Option<usize>>> = ctx.init_queue();

    while !queue.is_empty() {
        // y = max( {f(p) | p in queue} )
        let mut y = ctx.f[queue.last().unwrap()];

        // y = y - c // without overflow
        if y > C {
            y -= C;
        }

        // R = { p | p in queue and y <= f(p) }
        let second_queue = queue
            .clone()
            .into_iter()
            .filter(|p| y <= ctx.f[p])
            .collect::<Vec<Vec<Option<usize>>>>();
        queue.clear();

        for p in second_queue {
            if ctx.heuristic(&p) == 0 {
                // An MLCS match was found
                return ctx.common_seq(&p);
            }
            // inserting all succesors in the queue
            let succs = ctx.get_successors(&p);
            for q in succs {
                // basically saying if the queue queue does not already
                // contain the point q
                if !queue.contains(&q) {
                    ctx.update_suc(p.clone(), q.clone());
                    queue.push(q);
                }
            }
        }
        // sorting the queue
        ctx.reorder_queue(&mut queue);
    }
    String::from("")
}

/// Computes the suffix table
fn score_matrix(s1: &[char], s2: &[char]) -> Vec<Vec<u64>> {
    let m = s1.len();
    let n = s2.len();
    let mut matrix: Vec<Vec<u64>> = vec![vec![0; n + 1]; m + 1];

    if n > 0 && m > 0 {
        for i in (0..(m - 1)).rev() {
            for j in (0..(n - 1)).rev() {
                matrix[i][j] = if s1[i + 1] == s2[j + 1] {
                    matrix[i + 1][j + 1] + 1
                } else {
                    max(matrix[i][j + 1], matrix[i + 1][j])
                };
            }
        }
    }

    matrix
}

//given given 2D coordinates, to_linear_indexs into 1D coordinates
fn to_linear_index(i: usize, j: usize, d: usize) -> usize {
    i * d + j
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! astar_tests {
        ($($name:ident: ($input:expr, $expected:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    let input: Vec<&str> = $input.iter().map(|s| *s).collect();
                    let expected: String = String::from($expected);
                    let result = multiple_longest_common_subsequence(&input);
                    assert_eq!(result, expected);
                }
             )*
        };
    }

    astar_tests! {
             all_empty_strings: (["", ""], ""),
             all_same: (["abcdef", "abcdef", "abcdef", "abcdef"], "abcdef"),
             empty_strings: (["", "ABC"], ""),
             long: (
                    [
                        "qwertyuiop$asd$fgh$jkl;zxcvbnmqwert|yuiop1234567890-0",
                        "qwertyuiopasdfghj$kl;zx$cvbnmqwe$rtyu|iop,1234567890-0"
                    ],
                    "qwertyuiopasdfghjkl;zxcvbnmqwert|iop1234567890-0"
             ),
             medium_case: (
                    [
                        "gxt#xayb",
                        "abgt#ab",
                        "gyayt#ahjb",
                        "gyayjjjt#ab",
                        "gyayt#ahhhhb",
                        "ygaytp#pppahjb",
                        "ylllgaytm#8765majb",
                    ],
                    "gt#ab"
             ),
             medium_plus: (
                 [
                     "=串-用2于测试2展示测中测中0shgksjklkjlj测测🚀测测串文|",
                     "=串-串用2于测2试测中ss中0展示测测l中🚀文|串",
                     "=串-用2于测试2展67中中0xs中中中kkljhkkh示中测🚀测|测文|",
                     "=串-|用2于ss串试056u展xx🚀示中lj测ggk测|ss文|",
                     "=串-用2于-测22中中中uyty试串lj展gkks中示🚀测测s|测中文|b",
                     "=串-用2于测s-试2中中0中hgtihlkk展串🚀中示s中|文|",
                     "=2串2中2中2中s用-于0t测🚀j试展示测s测hkkkg测中中串文|l",
                     "=2串2中2中2中s用-于0测🚀试展示测s中k中l串文|",
                     "=2串2中2中2中s用ur-于0测🚀试展示测jkjljkkllkskg中串文|;",
                     "=2串2中2中2中s用u-ur于0测🚀试展jll示测gks中中串文|0",
                     "=2串2中2中2中s用-uurr于0测🚀试kl展示测s测中中串文|8",
                     "=2中2中s用-于0测🚀试展示测jsjhg测测中串文|",
                     "=2串2中2中2中s用-于0rttru测ljjgjh🚀试示测s测测中中串文|",
                     "=2串2中2中2中s用-于0gjg测lu🚀试展示测s测测中中串文|6",
                     "=2中22s-于0测🚀展测j测ljy中中串文|",
                     "=2串2中2中2中s用-jklkjll于hgj0测🚀试展示测s测中中串文|",
                     "=2串2中2中2中s用-于0g🚀试展示测s测中中lj串文|",
                     "=2串2中2中2中s用-于hj0试展示测sghhjjhgjl测测中串文|",
                     "=2串2中2中2中s用-于0h🚀试展示测sj测中jkl中串文|",
                     "=2串2中2中2中s用-于0j🚀试展示测gjgjsjk测串文|",
                     "=2串2中2中2中s用-于kj0🚀试展示测jjjlks中串文|",
                     "=2串2中2中2中s用-于0l🚀试展示fdj测l测中中串文|",
                     "=2串2中2中2中s用-于0🚀kl试展测测djkhdd中文|",
                     "=2串2中2中2中s用-于0试展示测s测fdljh中中串文|",
                     "=2串2中2中2中s用-于0测l🚀l试展示lshd测测中中串文|",
                     "=2串2中2中2中s用-于0测🚀jk试展示sf测测中中串文|",
                     "=串用2串2中🚀2-中于0测试中lk展中ks中23文|串",
                 ],
                 "=2于测文|"
             ),
             mix: (
                 [
                     "=串-用2于测试2展示测中测中0ss测测🚀测测串文|",
                     "=串-串用2于测2试测中ss中0展示测测中🚀文|串",
                     "=串-用2于测试2展中中0xs中中中示中测🚀测|测文",
                     "=串-|用2于ss串试0展xx🚀示中测测|ss文",
                     "=串-用2于-测22中中中试串展s中示🚀测测s|测中文",
                     "=串用2于测s-试2中中0中展串🚀中示s中|文",
                     "=2串2中2中2中s用-于0测🚀试展示测s测测中中串文|",
                     "=串用2串2中🚀2-中于0测试中展中示s中文|串",
                     "=串2🚀用1于-2测2中20中试s中展s示中文测|测测测测串",
                 ],
                 "=串用于试展示中文"
             ),
             no_match: (["ABC", "DEF"], ""),
             simple_case: (["ABC", "AC", "BAC"], "AC"),
             unicode: (
                    [
                        "串用于测试展示测中测中测测🚀测测串文",
                        "串串用于测试测中中展示测测中🚀文串",
                        "串用于测试展中中中中中示中测🚀测测文",
                        "串用于测串试展示中测测文",
                        "串用于测中中中试串展中示🚀测测测中文",
                        "串用于测试中中中展串🚀中示中文",
                        "串中中中用于测🚀试展示测测测中中串文",
                        "串用串中🚀中于测试中展中示中文串",
                        "串🚀用于测中中试中展示中文测测测测测串",
                    ],
                    "串用于测试展示中文"
             ),

    }
}