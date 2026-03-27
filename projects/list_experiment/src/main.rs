use std::io;

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, data: i32) {
        let new_node = Box::new(Node { data, next: self.head.take() });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    fn print_list(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{}, ", node.data);
            current = &node.next;
        }
        println!();
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(2);
    list.push(3);

    list.print_list();

    println!("Popped: {:?}", list.pop());
    list.print_list();

    println!("Popped: {:?}", list.pop());
    list.print_list();

    println!("Popped: {:?}", list.pop());
    list.print_list();

    println!("Popped: {:?}", list.pop()); // Try to pop from empty list
    list.print_list();
}
