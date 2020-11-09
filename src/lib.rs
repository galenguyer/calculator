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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
