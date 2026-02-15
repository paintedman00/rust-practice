use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <expression>", args[0]);
        return;
    }

    let expression = &args[1..];
    let mut stack: Vec<f64> = Vec::new();

    for token in expression {
        match token.parse::<f64>() {
            Ok(num) => stack.push(num),
            Err(_) => {
                match token.as_str() {
                    "+" => {
                        if stack.len() < 2 {
                            eprintln!("Error: Not enough operands for addition");
                            return;
                        }
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        stack.push(a + b);
                    }
                    "-" => {
                        if stack.len() < 2 {
                            eprintln!("Error: Not enough operands for subtraction");
                            return;
                        }
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        stack.push(a - b);
                    }
                    "*" => {
                        if stack.len() < 2 {
                            eprintln!("Error: Not enough operands for multiplication");
                            return;
                        }
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        stack.push(a * b);
                    }
                    "/" => {
                        if stack.len() < 2 {
                            eprintln!("Error: Not enough operands for division");
                            return;
                        }
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        if b == 0.0 {
                            eprintln!("Error: Division by zero");
                            return;
                        }
                        stack.push(a / b);
                    }
                    _ => {
                        eprintln!("Error: Invalid token: {}", token);
                        return;
                    }
                }
            }
        }
    }

    if stack.len() != 1 {
        eprintln!("Error: Invalid expression");
        return;
    }

    println!("{}", stack.pop().unwrap());
}
