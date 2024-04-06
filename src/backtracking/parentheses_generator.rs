/// Generates all combinations of well-formed parentheses given a positive integer `n`.
///
/// This function uses backtracking to generate all possible combinations of well-formed
/// parentheses. The resulting combinations are returned as a vector of strings.
///
/// # Arguments
///
/// * `n` - A positive integer representing the number of pairs of parentheses.
pub fn generate_parentheses(n: u32) -> Vec<String> {
    let mut result = Vec::new();
    generate("", 0, 0, n, &mut result);
    result
}

/// Helper function for generating parentheses recursively.
///
/// This function is called recursively to build combinations of well-formed parentheses.
/// It tracks the number of open and close parentheses added so far and adds a new parenthesis
/// if it's valid to do so.
///
/// # Arguments
///
/// * `current` - The current string of parentheses being built.
/// * `open` - The count of open parentheses in the current string.
/// * `close` - The count of close parentheses in the current string.
/// * `n` - The total number of pairs of parentheses to be generated.
/// * `result` - A mutable reference to the vector storing the generated combinations.
fn generate(current: &str, open: u32, close: u32, n: u32, result: &mut Vec<String>) {
    if current.len() == (n * 2) as usize {
        result.push(current.to_string());
        return;
    }

    if open < n {
        let mut new_str = current.to_string();
        new_str.push('(');
        generate(&new_str, open + 1, close, n, result);
    }

    if close < open {
        let mut new_str = current.to_string();
        new_str.push(')');
        generate(&new_str, open, close + 1, n, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_parentheses_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, expected_result) = $test_case;
                    assert_eq!(generate_parentheses(n), expected_result);
                }
            )*
        };
    }

    generate_parentheses_tests! {
        test_generate_parentheses_1: (1, vec!["()"]),
        test_generate_parentheses_2: (2, vec!["(())", "()()"]),
        test_generate_parentheses_3: (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
        test_generate_parentheses_4: (4, vec!["(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()", "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"]),
    }
}
