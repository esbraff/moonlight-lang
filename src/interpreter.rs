use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub enum Types {
    Double(f64),
    String(String),
    Table(HashMap<String, Types>),
    Null
}

impl Neg for Types {
    type Output = Types;

    fn neg(self) -> Types {
        match self {
            Types::Double(value) => Types::Double(-value),
            _ => panic!("Cant negate items")
        }
    }
}

impl Add for Types {
    type Output = Types;

    fn add(self, other: Types) -> Types {
        match (self, other) {
            (Types::Double(value1), Types::Double(value2)) => Types::Double(value1 + value2),
            (_, _) => panic!("Cant add items")
        }
    }
}

impl Sub for Types {
    type Output = Types;

    fn sub(self, other: Types) -> Types {
        match (self, other) {
            (Types::Double(value1), Types::Double(value2)) => Types::Double(value1 - value2),
            (_, _) => panic!("Cant substract items")
        }
    }
}

impl Mul for Types {
    type Output = Types;

    fn mul(self, other: Types) -> Types {
        match (self, other) {
            (Types::Double(value1), Types::Double(value2)) => Types::Double(value1 * value2),
            (_, _) => panic!("Cant multiply items")
        }
    }
}

impl Div for Types {
    type Output = Types;

    fn div(self, other: Types) -> Types {
        match (self, other) {
            (Types::Double(value1), Types::Double(value2)) => Types::Double(value1 / value2),
            (_, _) => panic!("Cant divide items")
        }
    }
}

pub struct InterpreterContext {
    pub variable_map: Vec<HashMap<String, Types>>
}

impl InterpreterContext {
    pub fn new() -> InterpreterContext {
        InterpreterContext {
            variable_map: Vec::new()
        }
    }

    pub fn insert_double(&mut self, frame: usize, key: String, value: f64) {
        self.variable_map[frame].insert(key, Types::Double(value));
    }

    pub fn insert_string(&mut self, frame: usize, key: String, value: String) {
        self.variable_map[frame].insert(key, Types::String(value));
    }

    pub fn insert_table(&mut self, frame: usize, key: String, value: HashMap<String, Types>) {
        self.variable_map[frame].insert(key, Types::Table(value));
    }
}
