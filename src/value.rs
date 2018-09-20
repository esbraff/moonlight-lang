use expressions::Expression;
use types::{Number, Bool};
use std::fmt::Formatter;
use storage::Storage;
use std::fmt::Error;
use std::fmt::Debug;
use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Clone)]
pub enum Value {
    Null,

    Number(Number),

    Bool(Bool),

    String(String),

    Function(Box<Expression>, Vec<String>),
    NativeFunc(fn(Vec<Box<Expression>>, &mut Storage) -> Value)
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Null => write!(f, "null"),
            Value::Number(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Function(_, _) => write!(f, "function"),
            Value::NativeFunc(_) => write!(f, "function")
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::Number(value) => Value::Number(-value),
            _ => panic!("Cant negate items")
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(value1), Value::Number(value2)) => Value::Number(value1 + value2),
            (Value::String(value1), Value::String(value2)) => Value::String(value1 + &value2),
            (Value::String(value1), Value::Number(value2)) => Value::String(value1 + &value2.to_string()),
            (Value::Number(value1), Value::String(value2)) => Value::String(value1.to_string() + &value2),
            (_, _) => panic!("Cant add items")
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(value1), Value::Number(value2)) => Value::Number(value1 - value2),
            (_, _) => panic!("Cant substract items")
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(value1), Value::Number(value2)) => Value::Number(value1 * value2),
            (_, _) => panic!("Cant multiply items")
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(value1), Value::Number(value2)) => Value::Number(value1 / value2),
            (_, _) => panic!("Cant divide items")
        }
    }
}
