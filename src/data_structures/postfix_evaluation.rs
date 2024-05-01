#[derive(Debug, PartialEq)]
pub enum PostfixError {
    DivisionByZero,
    InvalidOperator,
    InsufficientOperands,
    InvalidExpression,
}

pub fn evaluate_postfix(expression: &str) -> Result<i32, PostfixError> {
    let mut stack: Vec<i32> = Vec::new();

    for token in expression.split_whitespace() {
        if let Ok(number) = token.parse::<i32>() {
            // If the token is a number, push it onto the stack.
            stack.push(number);
        } else {
            // If the token is an operator, pop the top two values from the stack,
            // apply the operator, and push the result back onto the stack.
            if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                match token {
                    "+" => stack.push(a + b),
                    "-" => stack.push(a - b),
                    "*" => stack.push(a * b),
                    "/" => {
                        if b == 0 {
                            return Err(PostfixError::DivisionByZero);
                        }
                        stack.push(a / b);
                    }
                    _ => return Err(PostfixError::InvalidOperator),
                }
            } else {
                return Err(PostfixError::InsufficientOperands);
            }
        }
    }
    // The final result should be the only element on the stack.
    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err(PostfixError::InvalidExpression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_postfix_expression() {
        assert_eq!(evaluate_postfix("2 3 +"), Ok(5));
        assert_eq!(evaluate_postfix("5 2 * 4 +"), Ok(14));
        assert_eq!(evaluate_postfix("10 2 /"), Ok(5));
    }

    #[test]
    fn test_insufficient_operands() {
        assert_eq!(
            evaluate_postfix("+"),
            Err(PostfixError::InsufficientOperands)
        );
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(evaluate_postfix("5 0 /"), Err(PostfixError::DivisionByZero));
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(
            evaluate_postfix("2 3 #"),
            Err(PostfixError::InvalidOperator)
        );
    }

    #[test]
    fn test_invalid_expression() {
        assert_eq!(
            evaluate_postfix("2 3"),
            Err(PostfixError::InvalidExpression)
        );
        assert_eq!(
            evaluate_postfix("2 3 4 +"),
            Err(PostfixError::InvalidExpression)
        );
    }
}
