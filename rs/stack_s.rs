fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(101);

    println!("{stack:?}");
    stack.pop();
    println!("{stack:?}");
}


#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    fn push(&mut self, val: T) {
        self.stack.push(val);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }
}
