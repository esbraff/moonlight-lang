use interpreter::InterpreterContext;
use interpreter::Types;
use tokens::TokenType;

pub trait Expression {
    fn eval(&self, context: &InterpreterContext) -> Types;
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
    fn eval(&self, context: &InterpreterContext) -> Types {
    	Types::Double(self.value)
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
    fn eval(&self, context: &InterpreterContext) -> Types {
    	match self.op_type {
    	    TokenType::Add => self.left_expr.eval(context) + self.right_expr.eval(context),
    	    TokenType::Substract => self.left_expr.eval(context) - self.right_expr.eval(context),
    	    TokenType::Multiply => self.left_expr.eval(context) * self.right_expr.eval(context),
    	    TokenType::Divide => self.left_expr.eval(context) / self.right_expr.eval(context),
    	    _ => Types::Null
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
    fn eval(&self, context: &InterpreterContext) -> Types {
    	match self.op_type {
    	    TokenType::Add => self.expr.eval(context),
    	    TokenType::Substract => -self.expr.eval(context),
    	    _ => Types::Null
    	}
    }
}

pub struct VariableExpression {
	key: String
}

impl VariableExpression {
    pub fn new(key: String) -> VariableExpression {
    	VariableExpression {
    		key: key
    	}
    }
}

impl Expression for VariableExpression {
	fn eval(&self, context: &InterpreterContext) -> Types {
    	if context.variable_map[0].contains_key(&self.key) {
    		let res = context.variable_map[0].get(&self.key).unwrap();

    		return res.clone();
    	}

    	Types::Null
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
	fn eval(&self, context: &InterpreterContext) -> Types {
		let string = self.value.clone();

		Types::String(string)
    }
}