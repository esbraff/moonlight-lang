use interpreter::InterpreterContext;
use interpreter::Value;
use tokens::TokenType;

#[derive(Clone, Debug)]
pub enum Expression {
    Null,
    NumberValue(f64),
    StringValue(String),
    Binary(TokenType, Box<Expression>, Box<Expression>),
    Unary(TokenType, Box<Expression>),
    GetVariable(String),
    SetVariable(String, Box<Expression>),
    Function(Vec<Box<Expression>>, Vec<String>),
    CallFunc(String, Vec<Box<Expression>>),
}

impl Expression {
    pub fn eval(&self, context: &mut InterpreterContext) -> Value {
        match self {
            Expression::Null => Value::Null,
            Expression::NumberValue(value) => Value::Double(*value),
            Expression::StringValue(value) => Value::String(value.to_string()),
            Expression::Binary(op_type, left_expr, right_expr) => {
                match op_type {
                    TokenType::Add => left_expr.eval(context) + right_expr.eval(context),
                    TokenType::Substract => left_expr.eval(context) - right_expr.eval(context),
                    TokenType::Multiply => left_expr.eval(context) * right_expr.eval(context),
                    TokenType::Divide => left_expr.eval(context) / right_expr.eval(context),
                    _ => Value::Null
                }
            },
            Expression::Unary(op_type, expr) => {
                match op_type {
                    TokenType::Add => expr.eval(context),
                    TokenType::Substract => -expr.eval(context),
                    _ => Value::Null
                }
            },
            Expression::GetVariable(key) => {
                if context.variable_map[0].contains_key(key) {
                    let res = context.variable_map[0].get(key).unwrap();

                    return res.clone();
                }

                Value::Null
            },
            Expression::SetVariable(key, value) => {
                let val = value.eval(context);
                let key = key.clone();

                match val {
                    Value::Double(value) => { context.insert_double(0, key, value.clone()); Value::Double(value) },
                    Value::String(value) => { context.insert_string(0, key, value.clone()); Value::String(value) },
                    Value::Table(value) => { context.insert_table(0, key, value.clone()); Value::Table(value) },
                    Value::Func(value, args) => { context.insert_func(0, key, value.clone(), args.to_vec()); Value::Func(value, args) },
                    Value::RustFunc(value) => { context.insert_rust_func(0, key, value.clone()); Value::RustFunc(value) },
                    Value::Null => { context.insert_null(0, key); Value::Null }
                }
            },
            Expression::Function(exprs, args) => {
                Value::Func(exprs.to_vec(), args.to_vec())
            },
            Expression::CallFunc(key, args) => {
                if context.variable_map[0].contains_key(key) {
                    let value = context.variable_map[0].get(key).unwrap().clone();

                    return context.call_func(&value, args.to_vec());
                }

                Value::Null
            }
        }
    }
}
