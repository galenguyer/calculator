pub enum StackItem {
    Number(i32),
    Operation(Operation),
}

impl StackItem {
    pub fn unwrap(&self) -> i32 {
        match self {
            StackItem::Number(x) => x.to_owned(),
            _ => panic!("unwrap called on non-numeric value"),
        }
    }
}

pub enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
}

pub struct Stack {
    pub stack: Vec<StackItem>,
}

impl Stack {
    pub fn new(input: &str) -> Stack {
        Stack {
            stack: input
                .split(" ")
                .map(|x| match x {
                    "+" => StackItem::Operation(Operation::Add),
                    "-" => StackItem::Operation(Operation::Subtract),
                    "*" => StackItem::Operation(Operation::Multiply),
                    "/" => StackItem::Operation(Operation::Divide),
                    _ => StackItem::Number(x.parse::<i32>().unwrap()),
                })
                .collect::<Vec<StackItem>>(),
        }
    }

    pub fn eval(&mut self) -> i32 {
        let mut queue: Vec<i32> = Vec::<i32>::new();
        while self.stack.len() > 0 {
            let operation = self.stack.pop().unwrap();
            match operation {
                StackItem::Number(x) => {
                    if self.stack.len() == 0 {
                        return x;
                    } else {
                        queue.push(x);
                    }
                }
                StackItem::Operation(op) => {
                    while queue.len() > 1 {
                        let left = queue.pop().unwrap();
                        let right = queue.pop().unwrap();
                        match op {
                            Operation::Add => queue.push(left + right),
                            Operation::Subtract => queue.push(left - right),
                            Operation::Multiply => queue.push(left * right),
                            Operation::Divide => queue.push(left / right),
                        }
                    }
                    self.stack.push(StackItem::Number(queue.pop().unwrap()))
                }
            };
        }
        return self.stack.pop().unwrap().unwrap();
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
    let mut stack = Stack::new("/ 6 2");
    let result: i32 = stack.eval();
    assert_eq!(result, 3);
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
    assert_eq!(result, 27);
}
