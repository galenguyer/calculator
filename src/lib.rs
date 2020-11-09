pub struct Stack {
    pub stack: Vec<i32>,
}

impl Stack {
    pub fn new(input: Vec<i32>) -> Stack {
        Stack {
            stack: input,
        }
    }
}

#[test]
fn initializes() {
    let _ = Stack::new(vec![0]);
}
