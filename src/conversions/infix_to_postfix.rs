#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixToPostfixError {
    UnknownCharacter(char),
    UnmatchedParent,
}

/// Function to convert [infix expression](https://en.wikipedia.org/wiki/Infix_notation) to [postfix expression](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
pub fn infix_to_postfix(infix: &str) -> Result<String, InfixToPostfixError> {
    let mut postfix = String::new();
    let mut stack: Vec<char> = Vec::new();

    // Define the precedence of operators
    let precedence = |op: char| -> u8 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            '^' => 3,
            _ => 0,
        }
    };

    for token in infix.chars() {
        match token {
            c if c.is_alphanumeric() => {
                postfix.push(c);
            }
            '(' => {
                stack.push('(');
            }
            ')' => {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    postfix.push(top);
                }
            }
            '+' | '-' | '*' | '/' | '^' => {
                while let Some(top) = stack.last() {
                    if *top == '(' || precedence(*top) < precedence(token) {
                        break;
                    }
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
            other => return Err(InfixToPostfixError::UnknownCharacter(other)),
        }
    }

    while let Some(top) = stack.pop() {
        if top == '(' {
            return Err(InfixToPostfixError::UnmatchedParent);
        }

        postfix.push(top);
    }

    Ok(postfix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix() {
        assert_eq!(infix_to_postfix("a-b+c-d*e"), Ok(String::from("ab-c+de*-")));
        assert_eq!(
            infix_to_postfix("a*(b+c)+d/(e+f)"),
            Ok(String::from("abc+*def+/+"))
        );
        assert_eq!(
            infix_to_postfix("(a-b+c)*(d+e*f)"),
            Ok(String::from("ab-c+def*+*"))
        );
    }

    #[test]
    fn infix_with_error() {
        assert!(infix_to_postfix("(a-b)*#")
            .is_err_and(|e| matches!(e, InfixToPostfixError::UnknownCharacter('#'))));

        assert!(infix_to_postfix("((a-b)")
            .is_err_and(|e| matches!(e, InfixToPostfixError::UnmatchedParent)));
    }
}
