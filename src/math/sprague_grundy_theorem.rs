/**
 * Sprague Grundy Theorem for combinatorial games like Nim
 *
 * The Sprague Grundy Theorem is a fundamental concept in combinatorial game theory, commonly used to analyze
 * games like Nim. It calculates the Grundy number (also known as the nimber) for a position in a game.
 * The Grundy number represents the game's position, and it helps determine the winning strategy.
 *
 * The Grundy number of a terminal state is 0; otherwise, it is recursively defined as the minimum
 * excludant (mex) of the Grundy values of possible next states.
 *
 * For more details on Sprague Grundy Theorem, you can visit:(https://en.wikipedia.org/wiki/Sprague%E2%80%93Grundy_theorem)
 *
 * Author : [Gyandeep](https://github.com/Gyan172004)
 */

pub fn calculate_grundy_number(
    position: i64,
    grundy_numbers: &mut [i64],
    possible_moves: &[i64],
) -> i64 {
    // Check if we've already calculated the Grundy number for this position.
    if grundy_numbers[position as usize] != -1 {
        return grundy_numbers[position as usize];
    }

    // Base case: terminal state
    if position == 0 {
        grundy_numbers[0] = 0;
        return 0;
    }

    // Calculate Grundy values for possible next states.
    let mut next_state_grundy_values: Vec<i64> = vec![];
    for move_size in possible_moves.iter() {
        if position - move_size >= 0 {
            next_state_grundy_values.push(calculate_grundy_number(
                position - move_size,
                grundy_numbers,
                possible_moves,
            ));
        }
    }

    // Sort the Grundy values and find the minimum excludant.
    next_state_grundy_values.sort_unstable();
    let mut mex: i64 = 0;
    for grundy_value in next_state_grundy_values.iter() {
        if *grundy_value != mex {
            break;
        }
        mex += 1;
    }

    // Store the calculated Grundy number and return it.
    grundy_numbers[position as usize] = mex;
    mex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_grundy_number_test() {
        let mut grundy_numbers: Vec<i64> = vec![-1; 7];
        let possible_moves: Vec<i64> = vec![1, 4];
        calculate_grundy_number(6, &mut grundy_numbers, &possible_moves);
        assert_eq!(grundy_numbers, [0, 1, 0, 1, 2, 0, 1]);
    }
}
