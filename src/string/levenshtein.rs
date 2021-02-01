/// An algorithm for determining the difference between two strings using single
pub fn levenshtein(base: String, options: &[String]) -> (usize, String) {
    // Define the  default string to be "closest"
    let mut closest: (usize, String) = (0, "".to_string());

    // Iterate over all the entries in the `options` array
    for vec_entry in options {
        let (entry, lower_base, lower_comparison) = (vec_entry.clone(), base.to_lowercase(), vec_entry.to_lowercase());
        let (a, b) = (lower_base.as_bytes(), lower_comparison.as_bytes());

        // if the string is the same, return the string
        // else, calculate the difference between the two strings
        if a == b {
            closest = (0, String::from_utf8_lossy(b).to_string());
        } else {
            let (mut len, mut character_index, mut edits): (usize, usize, usize) = (a.len(), 0, 0);

            // if both strings are the same length, set the length to the length
            // of the longest option
            if b.len() > len {
                len = b.len();
            }

            // Whilst the current character is at an index less than the length
            // of the longest string, continue calculating
            while character_index < len {
                // If one string is longer than the other, add one change
                // if the character is not the same, add one change
                if character_index >= a.len() || character_index >= b.len() {
                    edits += 1;
                } else if a[character_index] != b[character_index] {
                    edits += 1
                }

                // increase the character_index by 1
                character_index += 1;
            };

            // if the closest number has more edits
            // than the currently calculated word
            // set the new closest to the current word
            if closest.0 > edits {
                closest = (edits, entry);
            }
        };
    }


    // return the closest value
    closest
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lev() {
        let result = super::levenshtein("Hey".to_string(), &vec!["Hi".to_string(), "hey".to_string(), "hello".to_string(), "helo".to_string()]);
        assert_eq!(result.0, 0)
    }
}
