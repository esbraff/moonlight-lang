use storage::{ StorageVariable, Storage };
use tokens::TokenType;
use value::Value;

#[derive(Clone, Debug)]
pub enum Expression {
    Null,
    NumberValue(f64),
    StringValue(String),
    Binary(TokenType, Box<Expression>, Box<Expression>),
    Unary(TokenType, Box<Expression>),
    GetVariable(String),
    SetVariable(String, Box<Expression>),
    Block(Vec<Box<Expression>>),
    Function(Box<Expression>, Vec<String>),
    CallFunc(String, Vec<Box<Expression>>),
    Return(Box<Expression>)
}

impl Expression {
    pub fn eval(&self, storage: &mut Storage) -> Value {
        match self {
            Expression::Null => Value::Null,
            Expression::NumberValue(value) => Value::Number(*value),
            Expression::StringValue(value) => Value::String(value.to_string()),
            Expression::Binary(op_type, left_expr, right_expr) => {
                let left = left_expr.eval(storage);
                let right = right_expr.eval(storage);

                match op_type {
                    TokenType::Add => left + right,
                    TokenType::Substract => left - right,
                    TokenType::Multiply => left * right,
                    TokenType::Divide => left / right,
                    _ => Value::Null
                }
            },
            Expression::Unary(op_type, expr) => {
                match op_type {
                    TokenType::Add => expr.eval(storage),
                    TokenType::Substract => -expr.eval(storage),
                    _ => Value::Null
                }
            },
            Expression::GetVariable(key) => {
                storage.get(StorageVariable::User(key.to_string()))
            },
            Expression::SetVariable(key, expr) => {
                let value = expr.eval(storage);

                match storage.store(StorageVariable::User(key.to_string()), value.clone()) {
                    Ok(_) => storage.get(StorageVariable::User(key.to_string())),
                    Err(_) => {
                        storage.new_var(key.to_string());
                        let _ = storage.store(StorageVariable::User(key.to_string()), value);
                        storage.get(StorageVariable::User(key.to_string()))
                    }
                }
            },
            Expression::Block(exprs) => {
                let mut result = Value::Null;

                storage.scope_start();

                for expr in exprs {
                    /*match Box::leak(expr.clone()) {
                        Expression::Return(_) => {
                            return expr.eval(storage);
                        },
                        _ => { expr.eval(storage); }
                    }*/

                    result = expr.eval(storage);
                }

                storage.scope_end();

                result
            },
            Expression::Function(ref expr, args) => {
                Value::Function(expr.clone(), args.to_vec())
            },
            Expression::CallFunc(key, args) => {
                let func = storage.get(StorageVariable::User(key.to_string()));
                let mut result = Value::Null;

                match func {
                    Value::Function(expr, arg_names) => {
                        storage.scope_start();

                        for i in 0..arg_names.len() {
                            let key = arg_names[i].to_string();
                            let value = args[i].eval(storage);
                            match storage.store(StorageVariable::User(key.to_string()), value.clone()) {
                                Ok(_) => { },
                                Err(_) => {
                                    storage.new_var(key.to_string());
                                    let _ = storage.store(StorageVariable::User(key.to_string()), value);
                                }
                            }
                        }

                        result = expr.eval(storage).clone();

                        storage.scope_end();
                    },
                    Value::NativeFunc(f) => {
                        f(args.to_vec(), storage);
                    },
                    _ => { panic!("Attempt to call not a function"); }
                }

                result
            },
            Expression::Return(expr) => {
                expr.eval(storage)
            }
        }
    }
}
