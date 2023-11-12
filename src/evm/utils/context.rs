use primitive_types::U256;

pub struct ExecutionContext {
    pub stack: Vec<U256>,
    pub pc: usize,
    pub input: Vec<u8>,
}

impl ExecutionContext {
    pub fn push_stack(&mut self, value: U256) {
        self.stack.push(value);
    }

    pub fn pop_stack(&mut self) -> U256 {
        self.stack.pop().unwrap()
    }

    pub fn pop_stack_n(&mut self, n: usize) -> Vec<U256> {
        let mut result: Vec<U256> = Vec::new();
        for _ in 0..n {
            result.push(self.stack.pop().unwrap());
        }
        result
    }
}
