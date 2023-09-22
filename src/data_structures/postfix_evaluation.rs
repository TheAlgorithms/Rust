fn evaluate_postfix(expression: &str) -> Result<i32, &'static str> {
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
                            return Err("Division by zero");
                        }
                        stack.push(a / b);
                    }
                    _ => return Err("Invalid operator"),
                }
            } else {
                return Err("Invalid postfix expression");
            }
        }
    }

    // The final result should be the only element on the stack.
    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Invalid postfix expression")
    }
}

fn main() {
    println!("Enter a postfix expression :");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let postfix_expression = input.trim();

    match evaluate_postfix(postfix_expression) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
