use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use std::collections::HashMap;
use expressions::Expression;
use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Clone)]
pub enum Value {
    Double(f64),
    String(String),
    Table(HashMap<String, Value>),
    Func(Vec<Box<Expression>>, Vec<String>),
    RustFunc(&'static Fn(Vec<Box<Expression>>, &mut InterpreterContext) -> Value, Vec<String>),
    Null,
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Double(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Table(_) => write!(f, "Table"),
            Value::Func(exprs, args) => write!(f, "{:?} {:?}", exprs, args),
            Value::RustFunc(_, _) => write!(f, "rust function"),
            Value::Null => write!(f, "Null")
        }
    }
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
            (Value::Double(value1), Value::String(value2)) => Value::String(value1.to_string() + &value2),
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

    pub fn insert_rust_func(&mut self, frame: usize, key: String, value: &'static Fn(Vec<Box<Expression>>, &mut InterpreterContext) -> Value, args: Vec<String>) {
        self.variable_map[frame].insert(key, Value::RustFunc(value, args));
    }

    pub fn insert_null(&mut self, frame: usize, key: String) {
        if self.variable_map[frame].contains_key(&key) {
            self.variable_map[frame].remove(&key);
        }
    }

    pub fn call_func(&mut self, value: &Value, args: Vec<Box<Expression>>) -> Value {
        match value {
            Value::Func(exprs, arg_names) => {
                let mut result = Value::Null;
                let mut arg_index = 0;

                for expr in args {
                    match expr.eval(self) {
                        Value::Double(value) => { self.insert_double(0, arg_names[arg_index].clone(), value.clone()); },
                        Value::String(value) => { self.insert_string(0, arg_names[arg_index].clone(), value.clone()); },
                        Value::Table(value) => { self.insert_table(0, arg_names[arg_index].clone(), value.clone()); },
                        Value::Func(value, args) => { self.insert_func(0, arg_names[arg_index].clone(), value.clone(), args.to_vec()); },
                        Value::RustFunc(value, args) => { self.insert_rust_func(0, arg_names[arg_index].clone(), value.clone(), args.to_vec()); },
                        Value::Null => { break; }
                    }
                    arg_index += 1;
                }

                for expr in exprs {
                    result = expr.eval(self);
                }

                for key in arg_names {
                    self.insert_null(0, key.to_string());
                }

                return result;
            },
            Value::RustFunc(func, _) => {
                return func(args.to_vec(), self);
            }
            _ => panic!("Attempt to call not a function")
        }
    }
}
