struct Stack {
    stack: Vec<i32>
}

impl Stack {
    fn new() -> Self {
        Stack {
            stack: Vec::new()
        }
    }
    
    fn length(&self) -> usize {
        self.stack.len()
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val)
    }
    
    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }
    
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    
    fn peek(&self) -> Option<&i32> {
        self.stack.last()
    }
}

fn main() {
    let mut st = Stack::new();
    
    st.push(1);
    st.push(2);
    st.push(3);
    
    st.pop();
    st.pop();
    
    println!("{}", st.peek().unwrap());
}
