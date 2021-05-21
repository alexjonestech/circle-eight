pub struct Stack {
    values: [usize; 16],
    pointer: usize
}

impl Stack {
    pub fn push(&mut self, address: usize) {
        self.pointer += 1;
        self.values[self.pointer-1] = address;
    }

    pub fn pop(&mut self) -> usize {
        self.pointer -= 1;
        return self.values[self.pointer];
    }

    pub fn init() -> Stack {
        return Stack {
            values: [0; 16],
            pointer: 0
        }
    }
}