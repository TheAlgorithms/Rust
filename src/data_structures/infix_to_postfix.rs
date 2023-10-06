// Function to convert infix expression to postfix expression
fn infix_to_postfix(infix: &str) -> String {
    let mut postfix = String::new();
    let mut stack: Vec<char> = Vec::new();

    // Define the precedence of operators
    let precedence = |op: char| -> i32 {
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
            _ => {}
        }
    }

    while let Some(top) = stack.pop() {
        if top == '(' {
            // If there are unmatched parentheses, it's an error.
            return "Error: Unmatched parentheses".to_string();
        }
        postfix.push(top);
    }

    postfix
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix() {
        assert_eq!(
            infix_to_postfix("a-b+c-d*e"),
            "ab-c+de*-".to_string()
        );
        assert_eq!(
            infix_to_postfix("a*(b+c)+d/(e+f)"),
            "abc+*def+/+".to_string()
        );
        assert_eq!(
            infix_to_postfix("(a-b+c)*(d+e*f)"),
            "ab-c+def*+*".to_string()
        );
    }
}
