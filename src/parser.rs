use expressions::Expression;
use tokens::TokenType;
use tokens::Token;

pub struct Parser<'a> {
    pub input: &'a Vec<Token>,
    pub output: Vec<Box<Expression>>,
    position: usize,
    lenght: usize
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Vec<Token>) -> Parser {
        Parser {
            input: input,
            output: Vec::new(),
            position: 0,
            lenght: input.len()
        }
    }

    fn match_type(&mut self, token_type: TokenType) -> bool {
        let curr_token = self.peek(0);

        if curr_token.token_type != token_type {
            return false;
        }

        self.position += 1;

        true
    }

    fn peek(&self, relative_position: i32) -> Token {
        let pos = (self.position as i32 + relative_position) as usize;

        if pos >= self.lenght {
            return Token::new(TokenType::EOF, String::new());
        }

        self.input[pos].clone()
    }

    fn expression(&mut self) -> Box<Expression> {
        self.additive()
    }

    fn additive(&mut self) -> Box<Expression> {
        let mut expr = self.multiplicative();

        loop {
            if self.match_type(TokenType::Add) {
                expr = Box::new(Expression::Binary(TokenType::Add, expr, self.multiplicative()));
                continue;
            }
            if self.match_type(TokenType::Substract) {
                expr = Box::new(Expression::Binary(TokenType::Substract, expr, self.multiplicative()));
                continue;
            }
            break;
        }

        expr
    }

    fn multiplicative(&mut self) -> Box<Expression> {
        let mut expr = self.unary();

        loop {
            if self.match_type(TokenType::Multiply) {
                expr = Box::new(Expression::Binary(TokenType::Multiply, expr, self.multiplicative()));
                continue;
            }
            if self.match_type(TokenType::Divide) {
                expr = Box::new(Expression::Binary(TokenType::Divide, expr, self.multiplicative()));
                continue;
            }
            break;
        }

        expr
    }

    fn unary(&mut self) -> Box<Expression> {
        if self.match_type(TokenType::Substract) {
            return Box::new(Expression::Unary(TokenType::Substract, self.primary()));
        }
        if self.match_type(TokenType::Add) {
            return Box::new(Expression::Unary(TokenType::Add, self.primary()));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expression> {
        let mut curr_token = self.peek(0);

        if self.match_type(TokenType::Func) {
            let mut args = Vec::new();

            if self.match_type(TokenType::LeftParen) {
                loop {
                    curr_token = self.peek(0);
                    if self.match_type(TokenType::VariableKey) {
                        let arg_name = curr_token.data;

                        args.push(arg_name);
                    } else if self.match_type(TokenType::RightParen) {
                        break;
                    } else {
                        let token_type = curr_token.token_type;

                        panic!("Expected {:?} or {:?}, found {:?}", TokenType::RightParen, TokenType::VariableKey, token_type);
                    }
                }
            }

            if self.match_type(TokenType::LeftBrace) {
                let mut exprs = Vec::new();

                while !self.match_type(TokenType::RightBrace) {
                    let expr = self.expression();

                    exprs.push(expr);
                }

                return Box::new(Expression::Function(exprs, args));
            } else if self.match_type(TokenType::ActionPointer) {
                let mut exprs = Vec::new();

                let expr = self.expression();

                exprs.push(expr);

                return Box::new(Expression::Function(exprs, args));
            }
        }
        if self.match_type(TokenType::Null) {
            return Box::new(Expression::Null);
        }
        if self.match_type(TokenType::Number) {
            return Box::new(Expression::NumberValue(curr_token.data.parse().unwrap()));
        }
        if self.match_type(TokenType::StringValue) {
            return Box::new(Expression::StringValue(curr_token.data));
        }
        if self.match_type(TokenType::VariableKey) {
            if self.match_type(TokenType::Setter) {
                let var_key_offset = -2;

                return Box::new(Expression::SetVariable(self.peek(var_key_offset).data, self.expression()));
            }
            if self.match_type(TokenType::Remover) {
                let var_key_offset = -2;

                return Box::new(Expression::SetVariable(self.peek(var_key_offset).data, Box::new(Expression::Null)));
            }
            if self.match_type(TokenType::LeftParen) {
                let mut args = Vec::new();

                loop {
                    if self.match_type(TokenType::RightParen) {
                        break;
                    } else {
                        args.push(self.expression());
                    }
                }

                return Box::new(Expression::CallFunc(curr_token.data, args));
            }
            return Box::new(Expression::GetVariable(curr_token.data));
        }
        if self.match_type(TokenType::HexNumber) {
            return Box::new(Expression::NumberValue(i64::from_str_radix(&curr_token.data, 16).unwrap() as f64));
        }
        if self.match_type(TokenType::LeftParen) {
            let expr = self.expression();
            self.match_type(TokenType::RightParen);
            return expr;
        }

        panic!("unknown expr");
    }

    pub fn parse(&mut self) {
        while !self.match_type(TokenType::EOF) {
            let expr = self.expression();

            self.output.push(expr);
        }
    }
}
