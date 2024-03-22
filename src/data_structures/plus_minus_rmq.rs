use std::cmp::PartialOrd;
use super:: range_minimum_query::build_sparse_table;

/// A data-structure for answering +-1 range minimum queries on arrays
///
/// # Complexity
/// Precomputation in O(n) and queries in O(1) time
///
/// # Notes
/// This is NOT the general RMQ on arrays but 'just' the +-1 RMQ, which is used in combination with LCA and
/// cartiesian trees on arrays to get a general RMQ implementation
///
/// # Sources
/// used <https://cp-algorithms.com/graph/lca_farachcoltonbender.html#implementation> as reference
pub struct PlusMinusOneRMQ<T: PartialOrd + Copy> {
    array: Vec<T>,
    n: usize, 
    k: usize,
    block_min: Vec<T>,
    block_min_idx: Vec<usize>,
    sparse_idx: Vec<Vec<usize>>,
    block_rmq: Vec<Vec<Vec<usize>>>,
    block_mask: Vec<u32>,
}

impl<T: PartialOrd + Copy> PlusMinusOneRMQ<T> {
    pub fn new(mut array: Vec<T>) -> Self {
        input_padding(&mut array);
        let n = array.len() as u32;
        let k = n.ilog2()/ 2;        
        let mut new = Self {
            array: array,
            n: n as usize,
            k: k as usize,
            block_min: Vec::new(),
            block_min_idx: Vec::new(),
            sparse_idx: vec![Vec::new()], // is a sparse table, which only stores the indeces
            block_rmq: Vec::new(),
            block_mask: Vec::new(),
        };
        new.calc_block_min();
        new.sparse_idx = build_sparse_table(&new.array);
        new.fill_block_rmq();
        new.precompute_masks();

        return new;
    }
    fn calc_block_min(&mut self) {
        for i in 0..(self.n + self.k -1) / self.k {
            let (min, min_idx) = self.calc_min(i*self.k);
            self.block_min.push(min);
            self.block_min_idx.push(min_idx)
        }
    }

    fn calc_min(&mut self, i: usize) -> (T, usize) {
        let mut current_min = self.array[i];
        let mut min_idx: usize = i;
        for j in i..i + self.k {
            match self.array.get(j) {
                Some(x) => {
                    current_min = min(current_min, *x);
                    min_idx = self.min_idx(min_idx, j);
                },
                None => break,
            };
        }
        return (current_min, min_idx);
    }

    pub fn get_range_min(&self, start: usize, end: usize) -> Result<T, &str> { 
        if start >= end || start >= self.array.len() || end > self.array.len() {
            return Err("invalid range");
        }

        let block_l = start / self.k ;
        let block_r = (end - 1) / self.k ;
        let l_suffix = self.get_in_block(block_l, start % self.k, self.k - 1);
        let r_prefix = self.get_in_block(block_r, 0, (end - 1) % self.k);
        match block_r - block_l {
            0 => return Ok(self.array[self.get_in_block(block_l, start % self.k, (end - 1) % self.k)]),
            1 => return Ok(self.array[self.min_idx(l_suffix, r_prefix)]),
            _ => return Ok(self.array[self.min_idx(self.min_idx(l_suffix, self.get_on_blocks(block_l+1, block_r-1)), r_prefix)]),
        };
    }

    fn get_on_blocks(&self, l: usize, r: usize) -> usize {
        let loglen = (r-l+1).ilog2() as usize;
        let idx: usize = ((r as i64) - (1 << loglen as i64) + 1) as usize;
        let a = self.sparse_idx[loglen][l as usize];
        let b = self.sparse_idx[loglen][idx];
        return self.min_idx(a,b);
    }

    fn get_in_block(&self, block_idx: usize, l: usize, r: usize) -> usize {  
        let mask = self.block_mask[block_idx];
        let min_idx = self.block_rmq[mask as usize][l][r];
        return min_idx + block_idx * self.k;
    }

    fn fill_block_rmq(&mut self) {
        let mask_amount = 1 << (self.k - 1);
        for mask in 0..mask_amount {
            let tmp = self.rmq_bitmask(mask as u32); // maybe change to usize
            self.block_rmq.push(tmp);
        }
    }

    fn rmq_bitmask(&mut self, mask: u32) -> Vec<Vec<usize>> {  
        let mut rmq_matrix: Vec<Vec<usize>> = vec![vec![0;self.k]; self.k];
        let list = bitmask_to_array(self.k, mask);
        for i in 0..self.k {
            for j in i..self.k {
                if i == j {
                    rmq_matrix[i][j] = i;
                }
                else {
                    let min = list[rmq_matrix[i][j-1]];     //Do we want range-minimum or range-maximum
                    if list[j] < min {
                        rmq_matrix[i][j] = j;
                    }
                    else {
                        rmq_matrix[i][j] = rmq_matrix[i][j-1];
                    }
                }
            }
        }
        return rmq_matrix;
    }

    fn precompute_masks(&mut self) {
        for i in 0..self.block_min.len() {
            self.block_mask.push(self.calc_bitmask(i));
        }
    }

    // we initialize the mask with k-1 ones
    // this is necessary so if blocks are of size < k the bitmask is still correct
    fn calc_bitmask(&self, block_idx: usize) -> u32{
        let mut mask: u32 = (1 << (self.k - 1)) - 1;  
        for i in self.k*block_idx + 1..self.k * (block_idx + 1) {
            let last = self.array[i-1];
            match self.array.get(i) {
                Some(&x) => {
                    if last >= x {
                        mask -= 1 << (self.k-1-(i % self.k));
                    }
                },
                None => break,
            };
        }                                
        return mask;
    }
    fn min_idx(&self, i: usize, j: usize) -> usize {
        if self.array[i] < self.array[j] {
            return i;
        }
        return j;
    }
}

// padds the given array to have at least length 4
// this is needed to have a valid k = 0.5 * log n
fn input_padding<T: Copy>(array: &mut Vec<T>) {
    while array.len() < 4 {
        let last = array[array.len() - 1];
        array.push(last);
    }
}

fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    match a < b {
        true => a,
        _ => b,
    } 
}

fn bitmask_to_array(k: usize, mut mask: u32) -> Vec<i32> {
    let mut list: Vec<i32> = vec![0];
    for i in 0..k-1{
        match mask % 2 {
            1 => list.push(list[i] - 1),
            _ => list.push(list[i] + 1),
        };
        mask /= 2;
    }
    list.reverse();
    return list;
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_query_tests() {
        let v1 = vec![1, 2, 3, 2, 3, 4, 5, 4, 3, 2, 1, 0, -1];
        let sparse_v1 = super::PlusMinusOneRMQ::new(v1);

        assert_eq!(Ok(2), sparse_v1.get_range_min(1, 6));
        assert_eq!(Ok(1), sparse_v1.get_range_min(0, 10));
        assert_eq!(Ok(-1), sparse_v1.get_range_min(10, 13));
        assert!(sparse_v1.get_range_min(4, 3).is_err());
        assert!(sparse_v1.get_range_min(0, 1000).is_err());
        assert!(sparse_v1.get_range_min(1000, 1001).is_err());
    }

}