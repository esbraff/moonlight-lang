use expressions::Expression;
use environment::{Environment, EnvironmentData};
use std::{cell::RefCell, rc::Rc};
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;
use stack::Stack;
use value::Value;

#[derive(Debug, Clone)]
pub enum StorageVariable {
    Local(usize),
    User(String),
}

#[derive(Default)]
pub struct Storage {
    items: Vec<Value>,
    last: usize,
    stack: Stack,
    env: Environment
}

pub struct VariableNotFoundError {
    key: String
}

impl Display for VariableNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Variable not found: {}", self. key)
    }
}

impl Storage {
    pub fn init_std(&mut self) {
        let print = Value::NativeFunc(|args: Vec<Box<Expression>>, storage: &mut Storage| -> Value {
            let obj_to_print = args[0].eval(storage);

            println!("{:?}", obj_to_print);

            Value::Null
        });

        match self.store(StorageVariable::User("print".to_string()), print.clone()) {
            Ok(_) => { },
            Err(_) => {
                self.new_var("print".to_string());
                let _ = self.store(StorageVariable::User("print".to_string()), print);
            }
        }
    }

    pub fn get_free(&mut self) -> StorageVariable {
        let result = self.last;

        self.items.push(Value::Null);
        self.last += 1;

        StorageVariable::Local(result)
    }

    pub fn new_var(&mut self, key: String) -> StorageVariable {
        let var = self.get_free();

        if let StorageVariable::Local(var) = var {
            self.env.borrow_mut().insert(key, var);
        }

        var
    }

    pub fn store(&mut self, var: StorageVariable, value: Value) -> Result<(), VariableNotFoundError> {
        match var {
            StorageVariable::Local(local) => {
                self.items[local] = value;
                Ok(())
            }
            StorageVariable::User(key) => {
                if let Some(var) = self.env.borrow().get(key.clone()) {
                    self.items[var] = value;
                    Ok(())
                } else {
                    Err(VariableNotFoundError { key })
                }
            }
        }
    }

    pub fn get(&self, var: StorageVariable) -> Value {
        match var {
            StorageVariable::Local(local) => self.items[local].clone(),
            StorageVariable::User(ident) => {
                let var: usize = self
                    .env
                    .borrow()
                    .get(ident.clone())
                    .expect(&format!("Variable not found: {}", ident));
                self.items[var].clone()
            }
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop()
    }

    pub fn scope_start(&mut self) {
        let outer = Rc::clone(&self.env);
        let data = EnvironmentData::new(Some(outer));
        self.env = Rc::new(RefCell::new(data));
    }

    pub fn scope_end(&mut self) {
        if self.env.borrow().outer.is_some() {
            let outer = Rc::clone(self.env.borrow().outer.as_ref().unwrap());
            self.env = outer;
        }
    }
}
