// Introduction to Catalan Numbers:
// Catalan numbers are a sequence of natural numbers with many applications in combinatorial mathematics.
// They are named after the Belgian mathematician Eugène Charles Catalan, who contributed to their study.
// Catalan numbers appear in various combinatorial problems, including counting correct bracket sequences,
// full binary trees, triangulations of polygons, and more.

// For more information, refer to the Wikipedia page on Catalan numbers:
// https://en.wikipedia.org/wiki/Catalan_number

// Author: [Gyandeep] (https://github.com/Gyan172004)

const MOD: i64 = 1000000007; // Define your MOD value here
const MAX: usize = 1005; // Define your MAX value here

pub fn init_catalan() -> Vec<i64> {
    let mut catalan = vec![0; MAX];
    catalan[0] = 1;
    catalan[1] = 1;

    for i in 2..MAX {
        catalan[i] = 0;
        for j in 0..i {
            catalan[i] += (catalan[j] * catalan[i - j - 1]) % MOD;
            if catalan[i] >= MOD {
                catalan[i] -= MOD;
            }
        }
    }

    catalan
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalan() {
        let catalan = init_catalan();

        // Test case 1: Catalan number for n = 0
        assert_eq!(catalan[0], 1);

        // Test case 2: Catalan number for n = 1
        assert_eq!(catalan[1], 1);

        // Test case 3: Catalan number for n = 5
        assert_eq!(catalan[5], 42);

        // Test case 4: Catalan number for n = 10
        assert_eq!(catalan[10], 16796);

        // Test case 5: Catalan number for n = 15
        assert_eq!(catalan[15], 9694845);

        // Print a success message if all tests pass
        println!("All tests passed!");
    }
}
