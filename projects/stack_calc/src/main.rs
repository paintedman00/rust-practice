use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let tokens: Vec<&str> = input.split_whitespace().collect();

        let mut stack: Vec<f64> = Vec::new();

        for token in tokens {
            match token {
                "+" => {
                    if stack.len() < 2 {
                        println!("Error: Not enough operands for addition");
                        continue;
                    }
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                }
                "-" => {
                    if stack.len() < 2 {
                        println!("Error: Not enough operands for subtraction");
                        continue;
                    }
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a - b);
                }
                "*" => {
                    if stack.len() < 2 {
                        println!("Error: Not enough operands for multiplication");
                        continue;
                    }
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a * b);
                }
                "/" => {
                    if stack.len() < 2 {
                        println!("Error: Not enough operands for division");
                        continue;
                    }
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if b == 0.0 {
                        println!("Error: Division by zero");
                        continue;
                    }
                    stack.push(a / b);
                }
                num => {
                    match num.parse::<f64>() {
                        Ok(n) => stack.push(n),
                        Err(_) => println!("Error: Invalid input: {}", num),
                    }
                }
            }
        }

        if stack.len() == 1 {
            println!("{}", stack.pop().unwrap());
        } else if stack.len() > 1 {
            println!("Error: Too many operands");
        }
    }
}
