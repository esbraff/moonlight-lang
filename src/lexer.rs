use std::vec::Vec;
use tokens::Token;
use tokens::TokenType;

pub struct Lexer<'a> {
	pub input: &'a str,
	lenght: usize,
	pub output: Vec<Token>,
	position: usize
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Lexer<'a> {
		Lexer {
			input: input,
			lenght: input.len(),
			output: Vec::new(),
			position: 0
		}
	}

	fn peek(&self, relative_position: usize) -> char {
		let pos = self.position + relative_position;

		if pos >= self.lenght {
			return '\0';
		}

		let chars : Vec<char> = self.input.chars().collect();

		chars[pos]
	}

	fn next(&mut self) -> char {
		self.position += 1;

		self.peek(0)
	}

	fn add_token(&mut self, token_type: TokenType, data: String) {
		self.output.push(Token::new(token_type, data));
	}

	fn tokenize_number(&mut self) {
		let mut num = String::new();
		let mut curr_ch = self.peek(0);

		loop {
			if curr_ch == '.' {
				if num.find('.') != None {
					panic!("number already contains dot");
				}
				num.push(curr_ch);
				curr_ch = self.next();
			} else if curr_ch.is_digit(10) {
				num.push(curr_ch);
				curr_ch = self.next();
			} else {
				break;
			}
		}

		if (curr_ch == 'x' || curr_ch == 'X') && num == "0".to_string() {
			self.next();
			self.tokenize_hex_number();
			return;
		}

		self.add_token(TokenType::Number, num);
	}

	fn tokenize_hex_number(&mut self) {
		let mut num = String::new();
		let mut curr_ch = self.peek(0);

		while curr_ch.is_digit(16) {
			num.push(curr_ch);
			curr_ch = self.next();
		}

		self.add_token(TokenType::HexNumber, num);
	}

	fn tokenize_operator(&mut self, op_type: TokenType) {
		self.add_token(op_type, String::new());
	}

	fn tokenize_key_word(&mut self) {
		let mut key_word = String::new();
		let mut curr_ch = self.peek(0);

		loop {
		    if !curr_ch.is_alphanumeric() && curr_ch != '_' && curr_ch != '$' {
		    	break;
		    }
		    key_word.push(curr_ch);
		    curr_ch = self.next();
		}

		self.add_token(TokenType::KeyWord, key_word);
	}

	fn tokenize_string(&mut self) {
		let mut string = String::new();
		let mut curr_ch = self.peek(0);

		while curr_ch != '\"' {
			string.push(curr_ch);
			curr_ch = self.next();
		}

		self.add_token(TokenType::StringValue, string);

		self.next();
	}

	pub fn tokenize(&mut self) {
		let op_tokens : [TokenType; 6] = [
			TokenType::Add,
			TokenType::Substract,
			TokenType::Multiply,
			TokenType::Divide,
			TokenType::LeftParent,
			TokenType::RightParent
		];

		while self.position < self.lenght {
			let curr_ch = self.peek(0);
			
			if curr_ch == '\"' {
				self.next();
				self.tokenize_string();
			} else if curr_ch.is_digit(10) {
				self.tokenize_number();
			} else if curr_ch.is_alphabetic() {
				self.tokenize_key_word();
			} else {
				let ch = "+-*/()".find(curr_ch);

				if ch != None {
					self.tokenize_operator(op_tokens[ch.unwrap()].clone());
				}

				self.next();
			}
		}
	}
}