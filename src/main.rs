use std::collections::HashMap;

mod tokens;
mod lexer;
mod parser;
mod expressions;
mod interpreter;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();

    let mut f = File::open(file_path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut context = interpreter::InterpreterContext::new();
    context.variable_map.push(HashMap::new());

    let mut lexer = Lexer::new(&contents);
    lexer.tokenize();

    let mut parser = Parser::new(&lexer.output);
    parser.parse();

    for expr in parser.output {
        use expressions::Expression;

        let result = expr.eval(&mut context);

        println!("{:?}", expr);

        match Box::leak(expr) {
            Expression::CallFunc(_key, _args) => println!("{:?}", result),
            _ => { continue; }
        }
    }
}
