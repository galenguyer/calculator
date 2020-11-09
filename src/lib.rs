enum  StackItem {
    Number(i32),
    Operation(Operation)
}

enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide
}

pub struct Stack {
    pub stack: Vec<String>,
}

impl Stack {
    pub fn new(input: &str) -> Stack {
        Stack {
            stack: input.split(" ").map(|x| x.to_string()).collect::<Vec<String>>(),
        }
    }

    pub fn eval(&mut self) -> i32 {
        while self.stack.len() > 1 {
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
            self.stack.push(stack_tail.to_string());
        }
        return self.stack.pop().unwrap().parse().unwrap();
    }
}

#[test]
fn addition() {
    let mut stack = Stack::new("+ 1 3");
    let result: i32 = stack.eval();
    assert_eq!(result, 4);
}

#[test]
fn subtraction() {
    let mut stack = Stack::new("- 6 3");
    let result: i32 = stack.eval();
    assert_eq!(result, 3);
}

#[test]
fn multiplication() {
    let mut stack = Stack::new("* 1 3");
    let result: i32 = stack.eval();
    assert_eq!(result, 3);
}

#[test]
fn division() {
    let mut stack = Stack::new("/ 4 2");
    let result: i32 = stack.eval();
    assert_eq!(result, 2);
}

#[test]
fn complex() {
    let mut stack: Stack = Stack::new("+ 3 * 4 2");
    let result: i32 = stack.eval();
    assert_eq!(result, 11);
}


#[test]
fn very_complex() {
    let mut stack: Stack = Stack::new("+ 3 * 4 2 3");
    let result: i32 = stack.eval();
    assert_eq!(result,27);
}
