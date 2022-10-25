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
}

fn main() {
    let mut head: Node = Node::new(1);
    
    for i in 2..6 {
        Node::add_to_list(&mut head, i);
    }
    
    Node::list_output(&mut head);
}
