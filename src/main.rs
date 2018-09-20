/*use interpreter::InterpreterContext;
use expressions::Expression;
use interpreter::Value;
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

fn print(args: Vec<Box<Expression>>, context: &mut InterpreterContext) -> Value {
    let obj_to_print = args[0].eval(context);

    println!("{:?}", obj_to_print);

    Value::Null
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();

    let mut f = File::open(file_path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut context = interpreter::InterpreterContext::new();
    context.variable_map.push(HashMap::new());

    context.insert(0, "print".to_owned(), Value::RustFunc(&print));

    let mut lexer = Lexer::new(&contents);
    lexer.tokenize();

    let mut parser = Parser::new(&lexer.output);
    parser.parse();

    for expr in parser.output {
        expr.eval(&mut context);
    }
}
*/

mod stack;
mod types;
mod value;
mod environment;
mod storage;
mod tokens;
mod lexer;
mod parser;
mod expressions;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use lexer::Lexer;
use parser::Parser;
use storage::Storage;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();

    let mut f = File::open(file_path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut storage = Storage::default();
    storage.init_std();

    let mut lexer = Lexer::new(&contents);
    lexer.tokenize();

    let mut parser = Parser::new(&lexer.output);
    parser.parse();

    for expr in parser.output {
        expr.eval(&mut storage);
    }
}
