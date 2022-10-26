use std::io;
use std::cmp::max;

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            value: val,
            left: None,
            right: None
        }
    }
    
    fn add_to_tree(&mut self, val: i32) {
        if val < self.value {
            match self.left {
                Some(ref mut l) => {
                    l.add_to_tree(val)
                }
                None => {
                    self.left = Some(Box::new(Node::new(val)))
                }
            }
        } 
        else {
            match self.right {
                Some(ref mut r) => {
                    r.add_to_tree(val)
                }
                None => {
                    self.right = Some(Box::new(Node::new(val)))
                }
            }
        }
    }
    
    fn make_list() -> Node {
        let mut numbers = String::new();

        io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

        let numbers_arr: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
        
        let mut root: Node = Node::new(numbers_arr[0]);
        
        for i in 1..numbers_arr.len() {
            root.add_to_tree(numbers_arr[i]);
        }
        
        root
    }
    
    /*fn get_height(&mut self) -> i32 {
        
    }*/
}

fn main() {
    let mut root = Node::make_list();
    
    println!("{}", Node::get_height(&mut root));
}
