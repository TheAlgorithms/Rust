//Here’s an example of the counting sort on a list of 0’s and 1's:
/* author Sreejita Roy (https://github.com/pseudonerd16) */

pub fn count_sort_binary(list: &mut [u8]) {
    let (zero_count, one_count) = list.iter()
                                      .fold((0, 0),
                                            |(zero, one), &el| {
        if el == 0 {
          (zero + 1, one)
        } else {
          (zero, one + 1)
        }
      });
    
    for i in 0..zero_count {
      list[i] = 0;
    }
    for i in zero_count..list.len() {
      list[i] = 1;
    }
  }