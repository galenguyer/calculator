pub struct Stack {
    pub stack: Vec<String>,
}

impl Stack {
    pub fn new(input: Vec<String>) -> Stack {
        Stack {
            stack: input,
        }
    }

}

#[test]
fn initializes() {
    let _ = Stack::new(vec!["+".to_string(), "1".to_string(), "3".to_string()]);
}
