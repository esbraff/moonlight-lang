use interpreter::InterpreterContext;
use interpreter::Value;
use tokens::TokenType;

pub trait Expression {
    fn eval(&self, context: &mut InterpreterContext) -> Value;
}
pub struct NumberExpression {
    value: f64
}

impl NumberExpression {
    pub fn new(value: f64) -> NumberExpression {
        NumberExpression {
            value: value
        }
    }
}

impl Expression for NumberExpression {
    fn eval(&self, context: &mut InterpreterContext) -> Value {
        Value::Double(self.value)
    }
}

pub struct BinaryExpression {
    left_expr: Box<Expression>,
    right_expr: Box<Expression>,
    op_type: TokenType
}

impl BinaryExpression {
    pub fn new(left_expr: Box<Expression>, right_expr: Box<Expression>, op_type: TokenType) -> BinaryExpression {
        BinaryExpression {
            left_expr: left_expr,
            right_expr: right_expr,
            op_type: op_type
        }
    }
}

impl Expression for BinaryExpression {
    fn eval(&self, context: &mut InterpreterContext) -> Value {
        match self.op_type {
            TokenType::Add => self.left_expr.eval(context) + self.right_expr.eval(context),
            TokenType::Substract => self.left_expr.eval(context) - self.right_expr.eval(context),
            TokenType::Multiply => self.left_expr.eval(context) * self.right_expr.eval(context),
            TokenType::Divide => self.left_expr.eval(context) / self.right_expr.eval(context),
            _ => Value::Null
        }
    }
}

pub struct UnaryExpression {
    expr: Box<Expression>,
    op_type: TokenType
}

impl UnaryExpression {
    pub fn new(expr: Box<Expression>, op_type: TokenType) -> UnaryExpression {
        UnaryExpression {
            expr: expr,
            op_type: op_type
        }
    }
}

impl Expression for UnaryExpression {
    fn eval(&self, context: &mut InterpreterContext) -> Value {
        match self.op_type {
            TokenType::Add => self.expr.eval(context),
            TokenType::Substract => -self.expr.eval(context),
            _ => Value::Null
        }
    }
}

pub struct GetVariableExpression {
    key: String
}

impl GetVariableExpression {
    pub fn new(key: String) -> GetVariableExpression {
        GetVariableExpression {
            key: key
        }
    }
}

impl Expression for GetVariableExpression {
    fn eval(&self, context: &mut InterpreterContext) -> Value {
        if context.variable_map[0].contains_key(&self.key) {
            let res = context.variable_map[0].get(&self.key).unwrap();

            return res.clone();
        }

        Value::Null
    }
}

pub struct SetVariableExpression {
    key: String,
    value: Box<Expression>
}

impl SetVariableExpression {
    pub fn new(key: String, value: Box<Expression>) -> SetVariableExpression {
        SetVariableExpression {
            key: key,
            value: value
        }
    }
}

impl Expression for SetVariableExpression {
    fn eval(&self, context: &mut InterpreterContext) -> Value {
        let val = self.value.eval(context);
        let key = self.key.clone();

        match val {
            Value::Double(value) => { context.insert_double(0, key, value); Value::Null },
            Value::String(value) => { context.insert_string(0, key, value); Value::Null },
            Value::Table(value) => { context.insert_table(0, key, value); Value::Null },
            Value::Null => { context.insert_null(0, key); Value::Null }
        }
    }
}

pub struct StringExpression {
    value: String
}

impl StringExpression {
    pub fn new(value: String) -> StringExpression {
        StringExpression {
            value: value
        }
    }
}

impl Expression for StringExpression {
    fn eval(&self, context: &mut InterpreterContext) -> Value {
        let string = self.value.clone();

        Value::String(string)
    }
}
