use std::io::{self, Write};

fn main() -> io::Result<()> {
    loop {
        println!("Choose a data structure:\
1. Stack\
2. Queue\
3. Exit");

        print!("> ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?; 

        match choice.trim() {
            "1" => stack_operations(),
            "2" => queue_operations(),
            "3" => break,
            _ => println!("Invalid choice."),
        }
    }

    Ok(())
}

fn stack_operations() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Stack: {:?}", stack);
    println!("Popped: {:?}", stack.pop());
    println!("Peek: {:?}", stack.last());
}

fn queue_operations() {
    let mut queue: std::collections::VecDeque<i32> = std::collections::VecDeque::new();

    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    println!("Queue: {:?}", queue);
    println!("Dequeued: {:?}", queue.pop_front());
    println!("Peek: {:?}", queue.front());
}
