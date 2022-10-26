use std::io;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>
}

impl Node {
    fn new (value: i32) -> Self {
        Self {
            value,
            next: None
        }
    }
    
    fn add_to_list(&mut self, val: i32) {
        match self.next {
            Some(ref mut x) => {
                x.add_to_list(val)
            }
            None => {
                self.next = Some(Box::new(Node::new(val)))
            }
        }
    }
    
    fn list_output(&mut self) {
        match self.next {
            Some(ref mut x) => {
                print!("{} -> ", self.value);
                x.list_output()
            }
            None => {
                print!("{}\n", self.value)
            }
        }
    }
    
    fn get_stdin() -> Node{
        let mut numbers = String::new();

        io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

        let numbers_arr: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
        
        let mut head: Node = Node::new(numbers_arr[0]);
        
        for i in 1..numbers_arr.len() {
            head.add_to_list(numbers_arr[i]);
        }
        
        head
    }
}

fn main() {
    let mut list_head = Node::get_stdin();
    
    println!("{:#?}", list_head);
}
