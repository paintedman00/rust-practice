use std::io;
use std::io::Write;

fn main() {
    println!("RPN Calculator. Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        match calculate(input) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn calculate(expression: &str) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expression.split_whitespace() {
        match token.parse::<f64>() {
            Ok(num) => stack.push(num),
            Err(_) => {
                match token {
                    "+" => {
                        let b = stack.pop().ok_or("Not enough operands")?;
                        let a = stack.pop().ok_or("Not enough operands")?;
                        stack.push(a + b);
                    }
                    "-" => {
                        let b = stack.pop().ok_or("Not enough operands")?;
                        let a = stack.pop().ok_or("Not enough operands")?;
                        stack.push(a - b);
                    }
                    "*" => {
                        let b = stack.pop().ok_or("Not enough operands")?;
                        let a = stack.pop().ok_or("Not enough operands")?;
                        stack.push(a * b);
                    }
                    "/" => {
                        let b = stack.pop().ok_or("Not enough operands")?;
                        let a = stack.pop().ok_or("Not enough operands")?;
                        if b == 0.0 {
                            return Err("Division by zero").into();
                        }
                        stack.push(a / b);
                    }
                    _ => return Err(format!("Invalid token: {}", token)).into(),
                }
            }
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression").into();
    }

    stack.pop().ok_or("Unexpected error".into())
}
