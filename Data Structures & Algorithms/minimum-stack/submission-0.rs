struct MinStack {
    stack: Vec<i32>
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: vec![]
        }
    }

    pub fn push(&mut self, val: i32) {
        &self.stack.push(val);
    }

    pub fn pop(&mut self) {
        &self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        match &self.stack.last() {
            Some(val) => **val,
            None => -1
        }
    }

    pub fn get_min(&self) -> i32 {
        match &self.stack.iter().min() {
            Some(val) => **val,
            None => -1
        }
    }
}
