use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match evaluate_rpn(&tokens) {
            Ok(result) => println!("{}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn evaluate_rpn(tokens: &[&str]) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token.parse::<f64>() {
            Ok(num) => stack.push(num),
            Err(_) => {
                match token {
                    "+" => {
                        let val2 = stack.pop().ok_or("Not enough operands")?;
                        let val1 = stack.pop().ok_or("Not enough operands")?;
                        stack.push(val1 + val2);
                    }
                    "-" => {
                        let val2 = stack.pop().ok_or("Not enough operands")?;
                        let val1 = stack.pop().ok_or("Not enough operands")?;
                        stack.push(val1 - val2);
                    }
                    "*" => {
                        let val2 = stack.pop().ok_or("Not enough operands")?;
                        let val1 = stack.pop().ok_or("Not enough operands")?;
                        stack.push(val1 * val2);
                    }
                    "/" => {
                        let val2 = stack.pop().ok_or("Not enough operands")?;
                        let val1 = stack.pop().ok_or("Not enough operands")?;
                        if val2 == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        stack.push(val1 / val2);
                    }
                    _ => return Err(format!("Invalid token: {}", token)),
                }
            }
        }
    }

    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Too many operands".to_string())
    }
}
