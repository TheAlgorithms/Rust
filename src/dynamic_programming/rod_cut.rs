//! Determine the maximum value obtainable by cutting up the rod and selling the pieces
//!
//! This problem can be aproximated to the knapsack problem, being the stolen articles and values the rope size and length size

// return the maximum value obtained
//rope_len -> length of the actual tope
//lenghts  -> vector with the possible lenghts a rope can be cut
//prices   -> prices of the respective leghts

pub fn rod_cut(rope_len: usize, lengths_vector: Vec<usize>, prices_vector: Vec<usize>) -> usize
{
    //both vectors have to be the same size
    if prices_vector.len()!=lengths_vector.len() {
        return 0;
    }
    if prices_vector.len()==0 {
        return 0;
    }

    let mut auxiliary_vec: Vec<usize> = vec![0; rope_len+1]; // vector to store information, "memoization"
    let mut current_vec: Vec<usize> = vec![0; rope_len+1];
    let mut minimum = 0;
    //for each possible lenght in the vector
    for (i,rope_available_len) in lengths_vector.iter().enumerate() {
        //try to cut rope with size [1,rope_len[
        for size in minimum..rope_len {
            //if the rope size trying to cut is bigger than the rope actual size, the segment cant be cut with that size
            if size+1>= *rope_available_len {
                let last_v = current_vec[(size + 1) - *rope_available_len] + prices_vector[i];
                let current_v = auxiliary_vec[size + 1];

                if last_v > current_v {
                    current_vec[size + 1] = last_v;
                } else {
                    current_vec[size + 1] = current_v;
                }
            }
        }
        //fill the auxiliary vector with the values from the current vector
        for (i,v) in current_vec.iter().enumerate(){
            auxiliary_vec[i] = *v;
        }
        minimum+=1;
    }
    return current_vec[rope_len];
}


#[cfg(test)]
mod tests {
    use super::rod_cut;

    #[test]
    fn test_rod_cut() {
        assert_eq!(rod_cut(11,vec![1,2,3,4,5,6,7,8],vec![1,5,8,9,10,17,17,20]),30);
        assert_eq!(rod_cut(8,vec![1,2,3,4,5,6,7,8],vec![1,5,8,9,10,17,17,20]),22);
        assert_eq!(rod_cut(4,vec![1,2,3,4,5,6,7,8],vec![1,5,8,9,10,17,17,20]),10);
        assert_eq!(rod_cut(105,vec![1,2,3,5,6,7],vec![1,5,9,10,17,17]),315);
        assert_eq!(rod_cut(8,vec![7],vec![17]),17);
        assert_eq!(rod_cut(1,vec![1,2,3,4],vec![1,2,3,4]),1);
    }
}
