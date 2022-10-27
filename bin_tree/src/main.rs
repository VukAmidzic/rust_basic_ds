use std::io;
use std::cmp::max;
use std::mem;

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    //initialization of a node of tree
    fn new(val: i32) -> Self {
        Self {
            value: val,
            left: None,
            right: None
        }
    }
    
    //adding new nodes to a tree
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
    
    //making a tree from stdin input
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
    
    //function for calculating a height of a tree
    fn get_height(&mut self) -> u32 {
        let h_left = match self.left {
            Some(ref mut l) => {
                l.get_height()
            }
            None => 0
        };
        
        let h_right = match self.right {
            Some(ref mut r) => {
                r.get_height()
            }
            None => 0
        };
        
        return max(h_left, h_right) + 1;
    }
    
    //helper function for calculating a width
    //for a certain level of tree
    fn get_lvl_width(root: &mut Node, lvl: u32) -> u32 {
        if lvl == 1 {
            return 1;
        }
        else {
            let w_left = match root.left {
                Some(ref mut l) => {
                    Node::get_lvl_width(l, lvl-1)
                }
                None => 0
            };
            
            let w_right = match root.right {
                Some(ref mut r) => {
                    Node::get_lvl_width(r, lvl-1)
                }
                None => 0
            };
            
            return w_right + w_left; 
        }
    }
    
    //main function for calculating a width of a tree
    fn get_total_width(root: &mut Node) -> u32 {
        let h = root.get_height();
        let mut max_width = 0;
        let mut width;
        
        for i in 1..h+1 {
            width = Node::get_lvl_width(root, i);
            if width > max_width {max_width = width};
        }
        
        return max_width;
    }
    
    //mirror-rotating a tree
    fn rotate_tree(&mut self) {
        match self.left {
            Some(ref mut l) => {
                Node::rotate_tree(l)
            },
            None => {}
        }
        
        match self.right {
            Some(ref mut r) => {
                Node::rotate_tree(r)
            },
            None => {}
        }
        
        mem::swap(&mut self.left, &mut self.right)
    }
}

fn main() {
    let mut root: Node = Node::make_list();
    
    println!("{:#?}", root);
    
    Node::rotate_tree(&mut root);
    
    println!("{:#?}", root);
}
