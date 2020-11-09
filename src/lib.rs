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
    let _ = Stack::new(vec!["0".to_string()]);
}
