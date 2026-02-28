use std::fmt;

// Define the linked list node
#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

// Define the linked list
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Create a new empty linked list
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Add a new node to the end of the linked list
    fn append(&mut self, data: i32) {
        let new_node = Box::new(Node { data, next: None });

        match &mut self.head {
            None => {
                self.head = Some(new_node);
            }
            Some(head) => {
                let mut current = head;
                while let Some(node) = &mut current.next {
                    current = node;
                }
                current.next = Some(new_node);
            }
        }
    }
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        let mut output = String::new();

        while let Some(node) = current {
            output.push_str(&format!("{}, ", node.data));
            current = &node.next;
        }

        // Remove the trailing comma and space
        if !output.is_empty() {
            output.pop();
            output.pop();
        }

        write!(f, "[{}]", output)
    }
}

fn main() {
    // Create a new linked list
    let mut list = LinkedList::new();

    // Add some elements to the linked list
    list.append(10);
    list.append(20);
    list.append(30);

    // Print the linked list
    println!("Linked List: {}", list);
}
