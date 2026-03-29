use std::fmt;

// Define the linked list node
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

    // Append a new node to the end of the list
    fn append(&mut self, data: i32) {
        let new_node = Box::new(Node { data, next: None });

        match &mut self.head {
            None => {
                self.head = Some(new_node);
            }
            Some(head) => {
                let mut current = head;
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
        }
    }

    // Prepend a new node to the beginning of the list
    fn prepend(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
}

// Implement Display trait for LinkedList
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        let mut output = String::new();

        while let Some(node) = current {
            output.push_str(&format!("{} ", node.data));
            current = &node.next;
        }

        write!(f, "[{}]", output.trim_end())
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.append(1);
    list.append(2);
    list.prepend(0);
    list.append(3);

    println!("Linked List: {}", list);
}
