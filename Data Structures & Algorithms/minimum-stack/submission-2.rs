struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min_val = if let Some(&top) = self.min_stack.last() {
            val.min(top)
        } else {
            val
        };
        self.min_stack.push(min_val);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        match self.stack.last() {
            Some(val) => *val,
            None => -1,
        }
    }

    pub fn get_min(&self) -> i32 {
        match self.min_stack.last() {
            Some(val) => *val,
            None => -1,
        }
    }
}