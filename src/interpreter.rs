use std::collections::HashMap;
use expressions::Expression;
use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug)]
#[derive(Clone)]
pub enum Value {
    Double(f64),
    String(String),
    Table(HashMap<String, Value>),
    Func(Vec<Box<Expression>>, Vec<String>),
    Null,
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::Double(value) => Value::Double(-value),
            _ => panic!("Cant negate items")
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Double(value1), Value::Double(value2)) => Value::Double(value1 + value2),
            (Value::String(value1), Value::String(value2)) => Value::String(value1 + &value2),
            (Value::String(value1), Value::Double(value2)) => Value::String(value1 + &value2.to_string()),
            (_, _) => panic!("Cant add items")
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Double(value1), Value::Double(value2)) => Value::Double(value1 - value2),
            (_, _) => panic!("Cant substract items")
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Double(value1), Value::Double(value2)) => Value::Double(value1 * value2),
            (_, _) => panic!("Cant multiply items")
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Double(value1), Value::Double(value2)) => Value::Double(value1 / value2),
            (_, _) => panic!("Cant divide items")
        }
    }
}

pub struct InterpreterContext {
    pub variable_map: Vec<HashMap<String, Value>>
}

impl InterpreterContext {
    pub fn new() -> InterpreterContext {
        InterpreterContext {
            variable_map: Vec::new()
        }
    }

    pub fn insert_double(&mut self, frame: usize, key: String, value: f64) {
        self.variable_map[frame].insert(key, Value::Double(value));
    }

    pub fn insert_string(&mut self, frame: usize, key: String, value: String) {
        self.variable_map[frame].insert(key, Value::String(value));
    }

    pub fn insert_table(&mut self, frame: usize, key: String, value: HashMap<String, Value>) {
        self.variable_map[frame].insert(key, Value::Table(value));
    }

    pub fn insert_func(&mut self, frame: usize, key: String, value: Vec<Box<Expression>>, args: Vec<String>) {
        self.variable_map[frame].insert(key, Value::Func(value, args));
    }

    pub fn insert_null(&mut self, frame: usize, key: String) {
        if self.variable_map[frame].contains_key(&key) {
            self.variable_map[frame].remove(&key);
        }
    }
}
