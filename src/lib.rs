pub struct Stack {
    pub stack: Vec<String>,
}

impl Stack {
    pub fn new(input: Vec<String>) -> Stack {
        Stack {
            stack: input,
        }
    }

    pub fn eval(&mut self) -> i32 {
        let right: i32 = self.stack.pop().unwrap().parse().unwrap();
        let left: i32 = self.stack.pop().unwrap().parse().unwrap();
        let operation = self.stack.pop().unwrap();
        let stack_tail = match operation.as_str() {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => panic!("operator not supported")
        };
        return stack_tail;
    }
}

#[test]
fn addition() {
    let mut stack = Stack::new(vec!["+".to_string(), "1".to_string(), "3".to_string()]);
    let result: i32 = stack.eval();
    assert_eq!(result, 4);
}

#[test]
fn subtraction() {
    let mut stack = Stack::new(vec!["-".to_string(), "6".to_string(), "3".to_string()]);
    let result: i32 = stack.eval();
    assert_eq!(result, 3);
}

#[test]
fn multiplication() {
    let mut stack = Stack::new(vec!["*".to_string(), "1".to_string(), "3".to_string()]);
    let result: i32 = stack.eval();
    assert_eq!(result, 3);
}

#[test]
fn division() {
    let mut stack = Stack::new(vec!["/".to_string(), "4".to_string(), "2".to_string()]);
    let result: i32 = stack.eval();
    assert_eq!(result, 2);
}

