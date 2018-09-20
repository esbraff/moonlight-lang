use value::Value;

#[derive(Debug, Default)]
pub struct Stack {
    items: Vec<Value>,
}

impl Stack {
    pub fn push(&mut self, value: Value) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.items.pop().unwrap()
    }
}
