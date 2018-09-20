use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;


pub type Environment = Rc<RefCell<EnvironmentData>>;

#[derive(Debug, Clone, Default)]
pub struct EnvironmentData {
    variables: HashMap<String, usize>,
    pub outer: Option<Environment>
}

impl EnvironmentData {
    pub fn new(outer: Option<Environment>) -> EnvironmentData {
        EnvironmentData {
            variables: HashMap::new(),
            outer: outer
        }
    }

    pub fn top(env: &Environment) -> Environment {
        match env.borrow().outer {
            Some(ref outer) => EnvironmentData::top(outer),
            None => env.clone()
        }
    }

    pub fn insert(&mut self, key: String, var: usize) {
        self.variables.insert(key, var);
    }

    pub fn env_of(&self, key: String) -> Option<Environment> {
        if self.variables.contains_key(&key) {
            Some(Rc::new(RefCell::new(self.clone())))
        } else {
            match self.outer {
                Some(ref outer) => outer.borrow().env_of(key),
                None => None,
            }
        }
    }

    pub fn get(&self, key: String) -> Option<usize> {
        match self.variables.get(&key) {
            Some(var) => Some(var.clone()),
            None => {
                if let Some(env) = self.env_of(key.to_string()) {
                    env.borrow().get(key)
                } else {
                    None
                }
            }
        }
    }
}
