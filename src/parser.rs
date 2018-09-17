﻿use expressions::Expression;
use expressions::NumberExpression;
use expressions::BinaryExpression;
use expressions::UnaryExpression;
use expressions::GetVariableExpression;
use expressions::SetVariableExpression;
use expressions::StringExpression;
use tokens::Token;
use tokens::TokenType;

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
                expr = Box::new(BinaryExpression::new(expr, self.multiplicative(), TokenType::Add));
                continue;
            }
            if self.match_type(TokenType::Substract) {
                expr = Box::new(BinaryExpression::new(expr, self.multiplicative(), TokenType::Substract));
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
                expr = Box::new(BinaryExpression::new(expr, self.unary(), TokenType::Multiply));
                continue;
            }
            if self.match_type(TokenType::Divide) {
                expr = Box::new(BinaryExpression::new(expr, self.unary(), TokenType::Divide));
                continue;
            }
            break;
        }

        expr
    }

    fn unary(&mut self) -> Box<Expression> {
        if self.match_type(TokenType::Substract) {
            return Box::new(UnaryExpression::new(self.primary(), TokenType::Substract));
        }
        if self.match_type(TokenType::Add) {
            return Box::new(UnaryExpression::new(self.primary(), TokenType::Add));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expression> {
        let curr_token = self.peek(0);

        if self.match_type(TokenType::Number) {
            return Box::new(NumberExpression::new(curr_token.data.parse().unwrap()));
        }
        if self.match_type(TokenType::StringValue) {
            return Box::new(StringExpression::new(curr_token.data));
        }
        if self.match_type(TokenType::VariableKey) {
            if self.match_type(TokenType::Setter) {
                let var_key_offset = -2;

                return Box::new(SetVariableExpression::new(self.peek(var_key_offset).data, self.expression()));
            }
            return Box::new(GetVariableExpression::new(curr_token.data));
        }
        if self.match_type(TokenType::HexNumber) {
            return Box::new(NumberExpression::new(i64::from_str_radix(&curr_token.data, 16).unwrap() as f64));
        }
        if self.match_type(TokenType::LeftParent) {
            let expr = self.expression();
            self.match_type(TokenType::RightParent);
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
