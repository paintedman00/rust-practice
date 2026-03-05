use std::collections::VecDeque;
use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to the struct_explorer!");

    let mut stack: Vec<i32> = Vec::new();
    let mut queue: VecDeque<i32> = VecDeque::new();

    loop {
        println!("\nChoose an action:");
        println!("1. Stack Operations");
        println!("2. Queue Operations");
        println!("3. Exit");

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => stack_operations(&mut stack),
            "2" => queue_operations(&mut queue),
            "3" => break,
            _ => println!("Invalid choice."),
        }
    }

    println!("Exiting struct_explorer.");
}

fn stack_operations(stack: &mut Vec<i32>) {
    loop {
        println!("\nStack Operations:");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Peek");
        println!("4. Back to Main Menu");

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => {
                let value = get_user_input("Enter value to push: ").parse::<i32>().unwrap();
                stack.push(value);
                println!("Stack: {:?}", stack);
            }
            "2" => {
                if let Some(value) = stack.pop() {
                    println!("Popped: {}", value);
                } else {
                    println!("Stack is empty.");
                }
                println!("Stack: {:?}", stack);
            }
            "3" => {
                if let Some(&value) = stack.last() {
                    println!("Peek: {}", value);
                } else {
                    println!("Stack is empty.");
                }
            }
            "4" => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn queue_operations(queue: &mut VecDeque<i32>) {
    loop {
        println!("\nQueue Operations:");
        println!("1. Enqueue");
        println!("2. Dequeue");
        println!("3. Peek");
        println!("4. Back to Main Menu");

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => {
                let value = get_user_input("Enter value to enqueue: ").parse::<i32>().unwrap();
                queue.push_back(value);
                println!("Queue: {:?}", queue);
            }
            "2" => {
                if let Some(value) = queue.pop_front() {
                    println!("Dequeued: {}", value);
                } else {
                    println!("Queue is empty.");
                }
                println!("Queue: {:?}", queue);
            }
            "3" => {
                if let Some(&value) = queue.front() {
                    println!("Peek: {}", value);
                } else {
                    println!("Queue is empty.");
                }
            }
            "4" => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
